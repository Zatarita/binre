# Binre
This crate provides a wrapper for any type that implements the read and write traits. It enables the parsing or serialization of endian-aware binary data from the stream, as well as offers additional utility functions for common operations when working with binary data, such as padding and byte alignment.

## Basic functionality
The primary goal of this project was to simplify the process of parsing binary data by introducing an opt-in architecture based on the BinaryReader and BinaryWriter traits. This approach allows for more efficient use of resources and reduces the clutter in IntelliSense, making it easier to focus on the essential tasks at hand.

## Examples
For this example we're going to imagine we have data in the following pseudo c-style data structure.
```c
// Size prefixed string - Non-null terminated
struct BinURI {
    uint16_t size;              // size of the URI string
    char string_data[size];     // The URI string
}

struct FileData {
    const char Signature[4] = "BIN\x00";  // File header
    uint16_t uriCount = 0;                // count of URIs & corresponding MIMEs
    // 2 bytes padding for u32 alignment
    uint32_t uriOffset = 0;               // u32 file offset
    uint32_t mimeOffset = 0;              // u32 file offset
    std::vector<BinURI> URIs = [];        // Array of size prefixed strings
    std::vector<char*> mimeTypes = [];    // Array of null terminated
}
```

This is a vastly simplified example of a file that contains plain text URIs and MimeTypes embedded with plain text data for sake of presenting concepts. To parse this data we'll just add the BinaryReader trait to our existing stream 

```rust
use std::{fs::File, io::{self, BufReader}, sync::Arc};
use crate::{BinaryReader, BinaryUtils, Endianness};

fn read_data() -> std::io::Result<()> {
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
```

Given a properly formatted file one potential output you could have is (this example file formatted properly is in this repo)
```
3 URIs and MIME types at 0x10 and 0x48
URIs:[
    "../images/image.png",
    "../audio/musc.mp3",
    "other_file.png",
]
MIME Types:[
    "image/png",
    "audio/mpeg",
    "image/png",
]
```

The same concepts work for writing binary formatted data to stream as well.
This is mostly for my own internal use in making modding tools for video games, but if other's find this useful help yourself.