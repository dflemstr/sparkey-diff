#![feature(plugin)]
#![plugin(docopt_macros)]

extern crate chan;
extern crate docopt;
#[macro_use]
extern crate error_chain;
extern crate num_cpus;
extern crate pbr;
extern crate protobuf;
extern crate rustc_serialize;
extern crate scoped_pool;
extern crate sparkey;
extern crate vcdiff;

mod diff;
mod error;

use std::cmp;
use std::fs;
use std::io;
use std::path;
use std::time;

const MAGIC: u64 = 0x4479656b72617053;
const UNSET: u64 = 0xffffffffffffffff;

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

    let num_cpus = num_cpus::get();
    let pool = scoped_pool::Pool::new(0);

    let old_reader = sparkey::hash::Reader::open(paths.old_index_path,
                                                 paths.old_log_path)?;
    let new_reader = sparkey::hash::Reader::open(paths.new_index_path,
                                                 paths.new_log_path)?;

    let mut mb = pbr::MultiBar::on(io::stderr());
    mb.println("Computing diff...");

    pool.scoped(|scope| {
        let old_reader = &old_reader;
        let new_reader = &new_reader;

        let (key_send, key_recv) = chan::async();
        let (diff_send, diff_recv) = chan::async();

        let mut key_bar = mb.create_bar(new_reader.num_entries());
        style_bar(&mut key_bar);
        key_bar.message("Key scan ");
        execute(&pool, scope, move || {
            send_all_keys(&new_reader, key_send, key_bar).unwrap()
        });

        let mut write_bar = mb.create_bar(new_reader.num_entries());
        style_bar(&mut write_bar);
        write_bar.message("Write    ");
        execute(&pool, scope, move || {
            write_diff(diff_recv, &paths.diff_path, write_bar).unwrap()
        });

        for i in 0..cmp::max(num_cpus - 2, 1) {
            let key_recv = key_recv.clone();
            let diff_send = diff_send.clone();

            let mut diff_bar = mb.create_bar(new_reader.num_entries());
            style_bar(&mut diff_bar);
            diff_bar.message(&format!("Diff ({}) ", i + 1));
            diff_bar.show_bar = false;
            diff_bar.show_percent = false;
            diff_bar.show_counter = false;
            diff_bar.show_time_left = false;

            execute(&pool, scope, move || {
                diff(key_recv, old_reader, new_reader, diff_send, diff_bar)
                    .unwrap()
            });
        }

        execute(&pool, scope, || mb.listen());
    });

    Ok(())
}

fn apply(paths: &Paths) -> error::Result<()> {
    Ok(())
}

fn send_all_keys<W>(reader: &sparkey::hash::Reader,
                    key_send: chan::Sender<Vec<u8>>,
                    mut bar: pbr::ProgressBar<W>)
                    -> error::Result<()>
    where W: io::Write
{
    for key in reader.keys()? {
        let key = key?;
        key_send.send(key);
        bar.inc();
    }

    bar.finish();

    Ok(())
}

fn diff<W>(key_recv: chan::Receiver<Vec<u8>>,
           old_reader: &sparkey::hash::Reader,
           new_reader: &sparkey::hash::Reader,
           diff_send: chan::Sender<Option<diff::DiffEntry>>,
           mut bar: pbr::ProgressBar<W>)
           -> error::Result<()>
    where W: io::Write
{

    for key in key_recv {
        let old_entry = old_reader.get(&key)?;
        let new_entry = new_reader.get(&key)?;

        let diff_entry = match (old_entry, new_entry) {
            (Some(a), Some(b)) => {
                if a != b {
                    let delta = vcdiff::encode(&a,
                                               &b,
                                               vcdiff::FORMAT_INTERLEAVED,
                                               true);

                    let mut entry = diff::DiffEntry::new();
                    entry.mut_patch().set_key(key);
                    entry.mut_patch().set_delta(delta);

                    Some(entry)
                } else {
                    None
                }
            }
            (None, Some(b)) => {
                let mut entry = diff::DiffEntry::new();
                entry.mut_put().set_key(key);
                entry.mut_put().set_value(b);

                Some(entry)
            }
            (Some(_), None) => {
                let mut entry = diff::DiffEntry::new();
                entry.mut_delete().set_key(key);

                Some(entry)
            }
            (None, None) => unreachable!(),
        };

        diff_send.send(diff_entry);
        bar.inc();
    }

    bar.finish();

    Ok(())
}

fn write_diff<W>(diff_recv: chan::Receiver<Option<diff::DiffEntry>>,
                 diff_path: &path::Path,
                 mut bar: pbr::ProgressBar<W>)
                 -> error::Result<()>
    where W: io::Write
{
    use std::io::Seek;
    use protobuf::Message;

    let mut num_entries = 0;
    let mut num_puts = 0;
    let mut num_deletes = 0;
    let mut num_patches = 0;

    let mut diff_file = fs::File::create(diff_path)?;
    {
        let mut coded_out = protobuf::CodedOutputStream::new(&mut diff_file);

        coded_out.write_fixed64_no_tag(MAGIC)?;

        for entry in diff_recv {
            if let Some(entry) = entry {
                coded_out.write_raw_varint32(entry.compute_size())?;
                entry.write_to_with_cached_sizes(&mut coded_out)?;

                num_entries += 1;

                if entry.has_put() {
                    num_puts += 1;
                }

                if entry.has_delete() {
                    num_deletes += 1;
                }

                if entry.has_patch() {
                    num_patches += 1;
                }
            }

            bar.inc();
        }
    }

    diff_file.seek(io::SeekFrom::Start(8))?;

    {
        let mut coded_out = protobuf::CodedOutputStream::new(&mut diff_file);

        write_header(&mut coded_out,
                     num_entries,
                     num_puts,
                     num_deletes,
                     num_patches)?;
    }

    bar.finish();

    Ok(())
}

fn write_header(coded_out: &mut protobuf::CodedOutputStream,
                num_entries: u64,
                num_puts: u64,
                num_deletes: u64,
                num_patches: u64)
                -> error::Result<()> {
    use protobuf::Message;

    let mut entry = diff::DiffEntry::new();

    entry.mut_header().set_num_entries(smear(num_entries));
    entry.mut_header().set_num_puts(smear(num_puts));
    entry.mut_header().set_num_deletes(smear(num_deletes));
    entry.mut_header().set_num_patches(smear(num_patches));

    coded_out.write_raw_varint32(entry.compute_size())?;
    entry.write_to_with_cached_sizes(coded_out)?;

    Ok(())
}

fn smear(value: u64) -> u64 {
    if value == 0 { UNSET } else { value }
}

fn style_bar<W>(bar: &mut pbr::ProgressBar<W>)
    where W: io::Write
{
    bar.format("[=> ]");
    bar.tick_format("▏▎▍▌▋▊▉██▉▊▋▌▍▎▏");
    bar.set_max_refresh_rate(Some(time::Duration::from_millis(15)));
}

fn execute<'a, F>(pool: &scoped_pool::Pool,
                  scope: &scoped_pool::Scope<'a>,
                  task: F)
    where F: FnOnce() + Send + 'a
{
    pool.expand();
    scope.execute(task);
}
