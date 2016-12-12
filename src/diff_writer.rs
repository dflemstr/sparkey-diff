use std::fs;
use std::io;
use std::path;
use std::sync;

use pbr;
use protobuf;

use diff;
use error;

const MAGIC: u64 = 0x4479656b72617053;
const UNSET: u64 = 0xffffffffffffffff;

pub struct Stats {
    pub num_entries: u64,
    pub num_puts: u64,
    pub num_deletes: u64,
    pub num_patches: u64,
}

pub fn write<W>(entries: sync::mpsc::Receiver<Option<diff::DiffEntry>>,
                diff_path: &path::Path,
                mut bar: pbr::ProgressBar<W>)
                -> error::Result<Stats>
    where W: io::Write
{
    use std::io::Seek;
    use protobuf::Message;

    let mut diff_file = fs::File::create(diff_path)?;

    let mut num_entries = 0;
    let mut num_puts = 0;
    let mut num_deletes = 0;
    let mut num_patches = 0;

    {
        let mut buf_out = io::BufWriter::new(&mut diff_file);
        let mut coded_out = protobuf::CodedOutputStream::new(&mut buf_out);
        coded_out.write_fixed64_no_tag(MAGIC)?;
        write_header(&mut coded_out, 0, 0, 0, 0)?;

        for entry in entries {

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

        bar.message("Flushing ");
    }

    bar.message("Header   ");
    diff_file.seek(io::SeekFrom::Start(8))?;
    let mut coded_out = protobuf::CodedOutputStream::new(&mut diff_file);

    write_header(&mut coded_out,
                 num_entries,
                 num_puts,
                 num_deletes,
                 num_patches)?;

    bar.message("Writer   ");
    bar.finish();

    Ok(Stats {
        num_entries: num_entries,
        num_puts: num_puts,
        num_deletes: num_deletes,
        num_patches: num_patches,
    })
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
