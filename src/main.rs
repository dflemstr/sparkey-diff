#![feature(plugin)]
#![plugin(docopt_macros)]

extern crate docopt;
#[macro_use]
extern crate error_chain;
extern crate itertools;
extern crate jobsteal;
extern crate num_cpus;
extern crate pbr;
extern crate protobuf;
extern crate rustc_serialize;
extern crate sparkey;
extern crate vcdiff;

mod diff;
mod diff_writer;
mod error;

use std::io;
use std::path;
use std::sync;
use std::thread;
use std::time;

const CHUNK_SIZE: usize = 8192;

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
    use itertools::Itertools;

    if !assume_no_removals {
        return Err("Checking for removals is not yet implemented".into());
    }

    let old_reader = sparkey::hash::Reader::open(paths.old_index_path,
                                                 paths.old_log_path)?;
    errln!("Opened old index {:?} log {:?}",
           paths.old_index_path, paths.old_log_path);

    // TODO: open index to check that entries exist...
    // but I haven't implemented that in the Sparkey lib yet!
    let new_reader = sparkey::log::Reader::open(paths.new_log_path)?;
    errln!("Opened new index {:?} log {:?}",
           paths.new_index_path, paths.new_log_path);

    let mut pool = jobsteal::make_pool(num_cpus::get())?;
    let mut mb = pbr::MultiBar::on(io::stderr());
    mb.println("Scanning all keys...");

    let mut key_bar = mb.create_bar(new_reader.num_entries());
    style_bar(&mut key_bar);
    key_bar.message("Key scan ");

    let mut diff_bar = mb.create_bar(new_reader.num_entries());
    style_bar(&mut diff_bar);
    diff_bar.message("Diff     ");

    let mut write_bar = mb.create_bar(new_reader.num_entries());
    style_bar(&mut write_bar);
    write_bar.message("Write    ");

    let (write_tx, write_rx) = sync::mpsc::channel();
    let (log_diff_tx, log_diff_rx) = sync::mpsc::channel();

    thread::spawn(move || mb.listen());

    let diff_path = paths.diff_path.to_path_buf();
    let writer = thread::spawn(move || {
        diff_writer::write(write_rx, &diff_path, write_bar)
    });
    let diff_logger = thread::spawn(move || {
        for i in log_diff_rx {
            diff_bar.add(i);
        }
        diff_bar.finish();
    });

    (pool.scope(|scope| {
        for chunk in &new_reader.entries().chunks(CHUNK_SIZE) {
            let chunk = chunk.collect::<sparkey::error::Result<Vec<_>>>()?;
            let old_reader = &old_reader;
            let write_tx = write_tx.clone();
            let log_diff_tx = log_diff_tx.clone();

            key_bar.add(chunk.len() as u64);

            scope.submit(move || {
                let len = chunk.len();
                for entry in chunk {
                    let (key, new_value) = match entry {
                        sparkey::log::Entry::Put(k, v) => (k, Some(v)),
                        sparkey::log::Entry::Delete(k) => (k, None),
                    };
                    let new_value_slice = new_value.as_ref().map(|c| &**c);
                    let diff_entry = diff(&*key, new_value_slice, old_reader).unwrap();

                    write_tx.send(diff_entry).unwrap();
                }
                log_diff_tx.send(len as u64).unwrap();
            });
        }

        key_bar.finish();
        drop(write_tx);
        drop(log_diff_tx);
        Ok(())
    }) as error::Result<()>)?;

    diff_logger.join().unwrap();
    let stats = writer.join().unwrap()?;

    errln!("Done writing diff {:?}", paths.diff_path);
    errln!("Entries: {:?}", stats.num_entries);
    errln!("Added: {:?}", stats.num_puts);
    errln!("Deleted: {:?}", stats.num_deletes);
    errln!("Patched: {:?}", stats.num_patches);

    Ok(())
}

fn apply(_paths: &Paths) -> error::Result<()> {
    Ok(())
}

fn diff(key: &[u8],
        new_value: Option<&[u8]>,
        old_reader: &sparkey::hash::Reader)
        -> error::Result<Option<diff::DiffEntry>> {

    let old_value = old_reader.get(&key)?;

    let diff_entry = match (old_value, new_value) {
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
    };

    Ok(diff_entry)
}

fn style_bar<W>(bar: &mut pbr::ProgressBar<W>)
    where W: io::Write
{
    bar.format("[=> ]");
    bar.tick_format("▏▎▍▌▋▊▉██▉▊▋▌▍▎▏");
    bar.set_max_refresh_rate(Some(time::Duration::from_millis(15)));
}
