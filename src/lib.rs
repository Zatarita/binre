mod binary_reader;
mod binary_utils;
mod binary_writer;
mod endianness;

pub use binary_reader::BinaryReader;
pub use binary_utils::BinaryUtils;
pub use binary_writer::BinaryWriter;
pub use endianness::Endianness;

#[cfg(test)]
mod tests {
    use std::{fs::File, io::{self, BufReader}, sync::Arc};

    use crate::{BinaryReader, BinaryUtils, Endianness};

    #[test]
    fn it_works() -> std::io::Result<()> {
        const EXPECTED_HEADER: [u8; 4] = [0x42, 0x49, 0x4E, 0x00];

        let file = File::open("./file.bin")?;
        let mut stream = BufReader::new(file);

        // Read new bytes from stream (without having to read into an existing array)
        let header: Arc<[u8]> = stream.read_raw(4)?;
        if *header != EXPECTED_HEADER {
            panic!("Header mismatch");
        }

        let count = stream.read_le_u16()?;
        // Padding
        stream.ignore(2)?;

        // Initialize with a read value read from stream
        let uri_offset: u32 = stream.read_le_u32()?;

        // Initialize variable then read into the value from stream
        let mut mime_offset: u32 = 0;
        stream.read_into_le_u32(&mut mime_offset)?;

        // Read our array of u16 size prefixed strings from the stream
        let uris = (0..count)
            .into_iter()
            .map(|_| stream.read_size_prefixed_string::<u16>(Endianness::Little))
            .collect::< Result<Vec<String>, io::Error> >()?;

        // Read our array of null terminated strings from the stream
        let mime_types = (0..count)
            .into_iter()
            .map(|_| stream.read_null_terminated_string())
            .collect::< Result<Vec<String>, io::Error> >()?;

        println!("{count} URIs and MIME types at 0x{uri_offset:x} and 0x{mime_offset:x}\nURIs:{uris:#?}\nMIME Types:{mime_types:#?}");

        Ok(())
    }
}
