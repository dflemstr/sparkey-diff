#![feature(plugin)]
#![plugin(docopt_macros)]

#[macro_use]
extern crate chan;
extern crate docopt;
#[macro_use]
extern crate error_chain;
extern crate jobsteal;
extern crate num_cpus;
extern crate pbr;
extern crate protobuf;
extern crate rustc_serialize;
extern crate sparkey;
extern crate thread_scoped;
extern crate vcdiff;

mod diff;
mod diff_writer;
mod error;

use std::borrow;
use std::collections;
use std::io;
use std::path;
use std::time;

// TODO: Use some log framework
macro_rules! errln(
    ($($arg:tt)*) => {
        {
            use std::io::Write;
            writeln!(io::stderr(), $($arg)*)
                .expect("failed printing to stderr");
        }
    }
);

// This generates an Args struct that parses the following CLI
// argument specification
docopt!(Args derive Debug, "
A tool for creating differences between Sparkey files

Usage:
  sparkey-diff --help
  sparkey-diff -c [-f] [-r] -l <old-log> -i <old-index> -L <new-log> -I <new-index> -d <diff>
  sparkey-diff -a [-f] -l <old-log> -i <old-index> -L <new-log> -I <new-index> -d <diff>

Options:
  --help
      Show this screen

  --create, -c
      Create the diff file (.spd).
  --apply, -a
      Apply the diff file (.spd)

  --force, -f
      Force overwrite of existing files.
  --assume-no-removals, -r
      Assume there are no removals between the old and new index,
      skipping a pass over the files.

  --old-log <old-log>, -l <old-log>
      Old Sparkey log file (.spl).
  --old-index <old-index>, -i <old-index>
      Old Sparkey index file (.spi).
  --new-log <new-log>, -L <new-log>
      New Sparkey log file (.spl).
  --new-index <new-index>, -I <new-index>
      New Sparkey index file (.spi).
  --diff <diff>, -d <diff>
      Diff file (.spd).
");

struct Paths<'a> {
    old_log_path: &'a path::Path,
    old_index_path: &'a path::Path,
    new_log_path: &'a path::Path,
    new_index_path: &'a path::Path,
    diff_path: &'a path::Path,
}

fn main() {
    let args: Args = Args::docopt().decode().unwrap_or_else(|e| e.exit());

    let paths = Paths {
        old_log_path: path::Path::new(&args.flag_old_log),
        old_index_path: path::Path::new(&args.flag_old_index),
        new_log_path: path::Path::new(&args.flag_new_log),
        new_index_path: path::Path::new(&args.flag_new_index),
        diff_path: path::Path::new(&args.flag_diff),
    };

    if args.flag_create {
        create(&paths, args.flag_assume_no_removals).unwrap()
    } else if args.flag_apply {
        apply(&paths).unwrap()
    } else {
        unreachable!()
    }
}

fn create(paths: &Paths, assume_no_removals: bool) -> error::Result<()> {
    if !assume_no_removals {
        return Err("Checking for removals is not yet implemented".into());
    }

    let old_reader = sparkey::log::Reader::open(paths.old_log_path)?;
    errln!("Opened old log {:?}", paths.old_log_path);

    let new_reader = sparkey::log::Reader::open(paths.new_log_path)?;
    errln!("Opened new log {:?}", paths.new_log_path);

    create_from_readers(&old_reader, &new_reader, paths.diff_path)
}

fn create_from_readers<'a>(old_reader: &'a sparkey::log::Reader,
                           new_reader: &'a sparkey::log::Reader,
                           diff_path: &path::Path)
                           -> error::Result<()> {

    let mut pool = jobsteal::make_pool(num_cpus::get())?;
    let mut mb = pbr::MultiBar::on(io::stderr());
    mb.println("Scanning all entries...");

    let mut old_bar = mb.create_bar(old_reader.num_entries());
    style_bar(&mut old_bar);
    old_bar.message("Scan old ");

    let mut new_bar = mb.create_bar(new_reader.num_entries());
    style_bar(&mut new_bar);
    new_bar.message("Scan new ");

    let mut write_bar = mb.create_bar(new_reader.num_entries());
    style_bar(&mut write_bar);
    write_bar.message("Write    ");

    let (old_tx, old_rx) = chan::async();
    let (new_tx, new_rx) = chan::async();
    let (write_tx, write_rx) = chan::async();

    let _g = scoped_thread(move || mb.listen());

    let writer = scoped_thread(move || {
        diff_writer::write(write_rx, diff_path, write_bar)
    });

    let old_reader_thread =
        scoped_thread(move || read_all(old_reader, old_tx, old_bar));

    let new_reader_thread =
        scoped_thread(move || read_all(new_reader, new_tx, new_bar));

    pool.scope(|scope| {
        let mut old_entries = collections::HashMap::new();
        let mut new_entries = collections::HashMap::new();

        let mut old_open = true;
        let mut new_open = true;

        enum Source<A> {
            Old(A),
            New(A),
        }

        'outer: loop {
            let entry;

            loop {
                match (old_open, new_open) {
                    (true, true) => {
                        let mut sel = chan::Select::new();
                        let old = sel.recv(&old_rx);
                        let new = sel.recv(&new_rx);

                        let which = sel.select();

                        if which == old.id() {
                            if let Some(e) = old.into_value() {
                                entry = Source::Old(e);
                                break;
                            } else {
                                old_open = false;
                            }
                        } else if which == new.id() {
                            if let Some(e) = new.into_value() {
                                entry = Source::New(e);
                                break;
                            } else {
                                new_open = false;
                            }
                        } else {
                            unreachable!()
                        }
                    },
                    (true, false) => {
                        if let Some(e) = old_rx.recv() {
                            entry = Source::Old(e);
                            break;
                        } else {
                            old_open = false;
                        }
                    },
                    (false, true) => {
                        if let Some(e) = new_rx.recv() {
                            entry = Source::New(e);
                            break;
                        } else {
                            new_open = false;
                        }
                    },
                    (false, false) => {
                        break 'outer;
                    }
                }
            }

            let key = match entry {
                Source::Old(old_entry) => {
                    match old_entry {
                        sparkey::log::Entry::Put(k, v) => {
                            old_entries.insert(k.clone().into_owned(), Some(v));
                            k.into_owned()
                        }
                        sparkey::log::Entry::Delete(k) => {
                            old_entries.insert(k.clone().into_owned(), None);
                            k.into_owned()
                        }
                    }
                },
                Source::New(new_entry) => {
                    match new_entry {
                        sparkey::log::Entry::Put(k, v) => {
                            new_entries.insert(k.clone().into_owned(), Some(v));
                            k.into_owned()
                        }
                        sparkey::log::Entry::Delete(k) => {
                            new_entries.insert(k.clone().into_owned(), None);
                            k.into_owned()
                        }
                    }
                },
            };

            match (old_entries.entry(key.clone()),
                   new_entries.entry(key.clone())) {
                (collections::hash_map::Entry::Occupied(old),
                 collections::hash_map::Entry::Occupied(new)) => {
                    let old_value = old.remove();
                    let new_value = new.remove();
                    let write_tx = &write_tx;

                    scope.submit(move || {
                        let old_value = old_value.as_ref().map(|c| &**c);
                        let new_value = new_value.as_ref().map(|c| &**c);
                        let diff_entry = diff(&key, old_value, new_value);
                        write_tx.send(diff_entry);
                    });
                }
                _ => (),
            }
        }

        Ok(()) as error::Result<()>
    })?;

    old_reader_thread.join()?;
    new_reader_thread.join()?;
    let stats = writer.join()?;

    errln!("Done writing diff {:?}", diff_path);
    errln!("Entries: {:?}", stats.num_entries);
    errln!("Added: {:?}", stats.num_puts);
    errln!("Deleted: {:?}", stats.num_deletes);
    errln!("Patched: {:?}", stats.num_patches);

    Ok(())
}

fn apply(_paths: &Paths) -> error::Result<()> {
    Ok(())
}

fn read_all<'a, W>(
    reader: &'a sparkey::log::Reader,
    entry_tx: chan::Sender<sparkey::log::Entry<borrow::Cow<'a, [u8]>>>,
    mut bar: pbr::ProgressBar<W>)
    -> error::Result<()>
    where W: io::Write
{
    for entry in reader.entries() {
        entry_tx.send(entry?);
        bar.inc();
    }
    bar.finish();
    drop(entry_tx);
    Ok(())
}

fn diff(key: &[u8],
        new_value: Option<&[u8]>,
        old_value: Option<&[u8]>)
        -> Option<diff::DiffEntry> {

    match (old_value, new_value) {
        (Some(a), Some(b)) => {
            if a != b {
                let delta =
                    vcdiff::encode(&a, &b, vcdiff::FORMAT_INTERLEAVED, true);

                let mut entry = diff::DiffEntry::new();
                entry.mut_patch().set_key(key.to_vec());
                entry.mut_patch().set_delta(delta);

                Some(entry)
            } else {
                None
            }
        }
        (None, Some(b)) => {
            let mut entry = diff::DiffEntry::new();
            entry.mut_put().set_key(key.to_vec());
            entry.mut_put().set_value(b.to_vec());

            Some(entry)
        }
        (Some(_), None) => {
            let mut entry = diff::DiffEntry::new();
            entry.mut_delete().set_key(key.to_vec());

            Some(entry)
        }
        (None, None) => unreachable!(),
    }
}

fn style_bar<W>(bar: &mut pbr::ProgressBar<W>)
    where W: io::Write
{
    bar.format("[=> ]");
    bar.tick_format("▏▎▍▌▋▊▉██▉▊▋▌▍▎▏");
    bar.set_max_refresh_rate(Some(time::Duration::from_millis(15)));
}

fn scoped_thread<'a, A, F>(f: F) -> thread_scoped::JoinGuard<'a, A>
    where A: Send + 'a,
          F: FnOnce() -> A + Send + 'a
{
    unsafe { thread_scoped::scoped(f) }
}
