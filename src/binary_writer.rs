use super::endianness::{swap_inplace, Endianness, SYS_ENDIANNESS};
use std::io::{self, Error, ErrorKind, Seek, Write};

pub trait BinaryWriterExt: Write + Seek {
    fn write_u8(&mut self, value: u8) -> io::Result<()>;
    fn write_i8(&mut self, value: i8) -> io::Result<()>;

    fn write_u16(&mut self, value: u16, endianness: Endianness) -> io::Result<()>;
    fn write_be_u16(&mut self, value: u16) -> io::Result<()>;
    fn write_le_u16(&mut self, value: u16) -> io::Result<()>;
    fn write_ne_u16(&mut self, value: u16) -> io::Result<()>;

    fn write_i16(&mut self, value: i16, endianness: Endianness) -> io::Result<()>;
    fn write_be_i16(&mut self, value: i16) -> io::Result<()>;
    fn write_le_i16(&mut self, value: i16) -> io::Result<()>;
    fn write_ne_i16(&mut self, value: i16) -> io::Result<()>;

    fn write_u32(&mut self, value: u32, endianness: Endianness) -> io::Result<()>;
    fn write_be_u32(&mut self, value: u32) -> io::Result<()>;
    fn write_le_u32(&mut self, value: u32) -> io::Result<()>;
    fn write_ne_u32(&mut self, value: u32) -> io::Result<()>;

    fn write_i32(&mut self, value: i32, endianness: Endianness) -> io::Result<()>;
    fn write_be_i32(&mut self, value: i32) -> io::Result<()>;
    fn write_le_i32(&mut self, value: i32) -> io::Result<()>;
    fn write_ne_i32(&mut self, value: i32) -> io::Result<()>;

    fn write_u64(&mut self, value: u64, endianness: Endianness) -> io::Result<()>;
    fn write_be_u64(&mut self, value: u64) -> io::Result<()>;
    fn write_le_u64(&mut self, value: u64) -> io::Result<()>;
    fn write_ne_u64(&mut self, value: u64) -> io::Result<()>;

    fn write_i64(&mut self, value: i64, endianness: Endianness) -> io::Result<()>;
    fn write_be_i64(&mut self, value: i64) -> io::Result<()>;
    fn write_le_i64(&mut self, value: i64) -> io::Result<()>;
    fn write_ne_i64(&mut self, value: i64) -> io::Result<()>;

    fn write_f32(&mut self, value: f32, endianness: Endianness) -> io::Result<()>;
    fn write_be_f32(&mut self, value: f32) -> io::Result<()>;
    fn write_le_f32(&mut self, value: f32) -> io::Result<()>;
    fn write_ne_f32(&mut self, value: f32) -> io::Result<()>;

    fn write_f64(&mut self, value: f64, endianness: Endianness) -> io::Result<()>;
    fn write_be_f64(&mut self, value: f64) -> io::Result<()>;
    fn write_le_f64(&mut self, value: f64) -> io::Result<()>;
    fn write_ne_f64(&mut self, value: f64) -> io::Result<()>;

    fn write_string(&mut self, value: &str) -> io::Result<()>;
    fn write_nt_string(&mut self, value: &str) -> io::Result<()>;
}

impl<StreamT: Write + Seek> BinaryWriterExt for StreamT {
    /// Write an u8 to stream
    ///
    /// # Parameters
    ///     `value` - The variable to write to the stream
    ///
    /// # Errors
    ///     io::Error during write
    ///     io::Error::UnexpectedEof when filestream could not write full buffer
    fn write_u8(&mut self, value: u8) -> io::Result<()> {
        if self.write(&value.to_ne_bytes())? != std::mem::size_of::<u8>() {
            return Err(Error::from(ErrorKind::UnexpectedEof));
        }

        Ok(())
    }

    /// Write an i8 to stream
    ///
    /// # Parameters
    ///     `value` - The variable to write to the stream
    ///
    /// # Errors
    ///     io::Error during write
    ///     io::Error::UnexpectedEof when filestream could not write full buffer
    fn write_i8(&mut self, value: i8) -> io::Result<()> {
        if self.write(&value.to_ne_bytes())? != std::mem::size_of::<i8>() {
            return Err(Error::from(ErrorKind::UnexpectedEof));
        }

        Ok(())
    }

    /// Write an u16 to stream
    ///
    /// # Parameters
    ///     `value` - The variable to write to the stream
    ///     `endianness` - Endianness of the stream to write to
    ///
    /// # Errors
    ///     io::Error during write
    ///     io::Error::UnexpectedEof when filestream could not write full buffer
    fn write_u16(&mut self, value: u16, endianness: Endianness) -> io::Result<()> {
        let mut bytes: [u8; 2] = value.to_ne_bytes();
        swap_inplace(bytes.as_mut_slice(), endianness);

        if self.write(&bytes)? != std::mem::size_of::<u16>() {
            return Err(Error::from(ErrorKind::UnexpectedEof));
        }

        Ok(())
    }

    /// Write a big endian u16 to stream
    ///
    /// # Parameters
    ///     `value` - The variable to write to the stream
    ///
    /// # Errors
    ///     io::Error during write
    ///     io::Error::UnexpectedEof when filestream could not write full buffer
    fn write_be_u16(&mut self, value: u16) -> io::Result<()> {
        self.write_u16(value, Endianness::Big)
    }


    /// Write a little endian u16 to stream
    ///
    /// # Parameters
    ///     `value` - The variable to write to the stream
    ///
    /// # Errors
    ///     io::Error during write
    ///     io::Error::UnexpectedEof when filestream could not write full buffer
    fn write_le_u16(&mut self, value: u16) -> io::Result<()> {
        self.write_u16(value, Endianness::Little)
    }


    /// Write a native endian u16 to stream
    ///
    /// # Parameters
    ///     `value` - The variable to write to the stream
    ///
    /// # Errors
    ///     io::Error during write
    ///     io::Error::UnexpectedEof when filestream could not write full buffer
    fn write_ne_u16(&mut self, value: u16) -> io::Result<()> {
        self.write_u16(value, *SYS_ENDIANNESS)
    }

    /// Write an i16 to stream
    ///
    /// # Parameters
    ///     `value` - The variable to write to the stream
    ///     `endianness` - Endianness of the stream to write to
    ///
    /// # Errors
    ///     io::Error during write
    ///     io::Error::UnexpectedEof when filestream could not write full buffer
    fn write_i16(&mut self, value: i16, endianness: Endianness) -> io::Result<()> {
        let mut bytes: [u8; 2] = value.to_ne_bytes();
        swap_inplace(bytes.as_mut_slice(), endianness);

        if self.write(&bytes)? != std::mem::size_of::<i16>() {
            return Err(Error::from(ErrorKind::UnexpectedEof));
        }

        Ok(())
    }


    /// Write a big endian i16 to stream
    ///
    /// # Parameters
    ///     `value` - The variable to write to the stream
    ///
    /// # Errors
    ///     io::Error during write
    ///     io::Error::UnexpectedEof when filestream could not write full buffer
    fn write_be_i16(&mut self, value: i16) -> io::Result<()> {
        self.write_i16(value, Endianness::Big)
    }


    /// Write a big endian i16 to stream
    ///
    /// # Parameters
    ///     `value` - The variable to write to the stream
    ///
    /// # Errors
    ///     io::Error during write
    ///     io::Error::UnexpectedEof when filestream could not write full buffer
    fn write_le_i16(&mut self, value: i16) -> io::Result<()> {
        self.write_i16(value, Endianness::Little)
    }


    /// Write a big endian i16 to stream
    ///
    /// # Parameters
    ///     `value` - The variable to write to the stream
    ///
    /// # Errors
    ///     io::Error during write
    ///     io::Error::UnexpectedEof when filestream could not write full buffer
    fn write_ne_i16(&mut self, value: i16) -> io::Result<()> {
        self.write_i16(value, *SYS_ENDIANNESS)
    }

    /// Write an u32 to stream
    ///
    /// # Parameters
    ///     `value` - The variable to write to the stream
    ///     `endianness` - Endianness of the stream to write to
    ///
    /// # Errors
    ///     io::Error during write
    ///     io::Error::UnexpectedEof when filestream could not write full buffer
    fn write_u32(&mut self, value: u32, endianness: Endianness) -> io::Result<()> {
        let mut bytes: [u8; 4] = value.to_ne_bytes();
        swap_inplace(bytes.as_mut_slice(), endianness);

        if self.write(&bytes)? != std::mem::size_of::<u32>() {
            return Err(Error::from(ErrorKind::UnexpectedEof));
        }

        Ok(())
    }


    /// Write a big endian u32 to stream
    ///
    /// # Parameters
    ///     `value` - The variable to write to the stream
    ///
    /// # Errors
    ///     io::Error during write
    ///     io::Error::UnexpectedEof when filestream could not write full buffer
    fn write_be_u32(&mut self, value: u32) -> io::Result<()> {
        self.write_u32(value, Endianness::Big)
    }


    /// Write a big endian u32 to stream
    ///
    /// # Parameters
    ///     `value` - The variable to write to the stream
    ///
    /// # Errors
    ///     io::Error during write
    ///     io::Error::UnexpectedEof when filestream could not write full buffer
    fn write_le_u32(&mut self, value: u32) -> io::Result<()> {
        self.write_u32(value, Endianness::Little)
    }


    /// Write a big endian u32 to stream
    ///
    /// # Parameters
    ///     `value` - The variable to write to the stream
    ///
    /// # Errors
    ///     io::Error during write
    ///     io::Error::UnexpectedEof when filestream could not write full buffer
    fn write_ne_u32(&mut self, value: u32) -> io::Result<()> {
        self.write_u32(value, *SYS_ENDIANNESS)
    }

    /// Write an i32 to stream
    ///
    /// # Parameters
    ///     `value` - The variable to write to the stream
    ///     `endianness` - Endianness of the stream to write to
    ///
    /// # Errors
    ///     io::Error during write
    ///     io::Error::UnexpectedEof when filestream could not write full buffer
    fn write_i32(&mut self, value: i32, endianness: Endianness) -> io::Result<()> {
        let mut bytes: [u8; 4] = value.to_ne_bytes();
        swap_inplace(bytes.as_mut_slice(), endianness);

        if self.write(&bytes)? != std::mem::size_of::<i32>() {
            return Err(Error::from(ErrorKind::UnexpectedEof));
        }

        Ok(())
    }


    /// Write a big endian i32 to stream
    ///
    /// # Parameters
    ///     `value` - The variable to write to the stream
    ///
    /// # Errors
    ///     io::Error during write
    ///     io::Error::UnexpectedEof when filestream could not write full buffer
    fn write_be_i32(&mut self, value: i32) -> io::Result<()> {
        self.write_i32(value, Endianness::Big)
    }


    /// Write a big endian i32 to stream
    ///
    /// # Parameters
    ///     `value` - The variable to write to the stream
    ///
    /// # Errors
    ///     io::Error during write
    ///     io::Error::UnexpectedEof when filestream could not write full buffer
    fn write_le_i32(&mut self, value: i32) -> io::Result<()> {
        self.write_i32(value, Endianness::Little)
    }


    /// Write a big endian i32 to stream
    ///
    /// # Parameters
    ///     `value` - The variable to write to the stream
    ///
    /// # Errors
    ///     io::Error during write
    ///     io::Error::UnexpectedEof when filestream could not write full buffer
    fn write_ne_i32(&mut self, value: i32) -> io::Result<()> {
        self.write_i32(value, *SYS_ENDIANNESS)
    }

    /// Write an u64 to stream
    ///
    /// # Parameters
    ///     `value` - The variable to write to the stream
    ///     `endianness` - Endianness of the stream to write to
    ///
    /// # Errors
    ///     io::Error during write
    ///     io::Error::UnexpectedEof when filestream could not write full buffer
    fn write_u64(&mut self, value: u64, endianness: Endianness) -> io::Result<()> {
        let mut bytes: [u8; 8] = value.to_ne_bytes();
        swap_inplace(bytes.as_mut_slice(), endianness);

        if self.write(&bytes)? != std::mem::size_of::<u64>() {
            return Err(Error::from(ErrorKind::UnexpectedEof));
        }

        Ok(())
    }


    /// Write a big endian u64 to stream
    ///
    /// # Parameters
    ///     `value` - The variable to write to the stream
    ///
    /// # Errors
    ///     io::Error during write
    ///     io::Error::UnexpectedEof when filestream could not write full buffer
    fn write_be_u64(&mut self, value: u64) -> io::Result<()> {
        self.write_u64(value, Endianness::Big)
    }


    /// Write a big endian u64 to stream
    ///
    /// # Parameters
    ///     `value` - The variable to write to the stream
    ///
    /// # Errors
    ///     io::Error during write
    ///     io::Error::UnexpectedEof when filestream could not write full buffer
    fn write_le_u64(&mut self, value: u64) -> io::Result<()> {
        self.write_u64(value, Endianness::Little)
    }


    /// Write a big endian u64 to stream
    ///
    /// # Parameters
    ///     `value` - The variable to write to the stream
    ///
    /// # Errors
    ///     io::Error during write
    ///     io::Error::UnexpectedEof when filestream could not write full buffer
    fn write_ne_u64(&mut self, value: u64) -> io::Result<()> {
        self.write_u64(value, *SYS_ENDIANNESS)
    }

    /// Write an i64 to stream
    ///
    /// # Parameters
    ///     `value` - The variable to write to the stream
    ///     `endianness` - Endianness of the stream to write to
    ///
    /// # Errors
    ///     io::Error during write
    ///     io::Error::UnexpectedEof when filestream could not write full buffer
    fn write_i64(&mut self, value: i64, endianness: Endianness) -> io::Result<()> {
        let mut bytes: [u8; 8] = value.to_ne_bytes();
        swap_inplace(bytes.as_mut_slice(), endianness);

        if self.write(&bytes)? != std::mem::size_of::<i64>() {
            return Err(Error::from(ErrorKind::UnexpectedEof));
        }

        Ok(())
    }


    /// Write a big endian i64 to stream
    ///
    /// # Parameters
    ///     `value` - The variable to write to the stream
    ///
    /// # Errors
    ///     io::Error during write
    ///     io::Error::UnexpectedEof when filestream could not write full buffer
    fn write_be_i64(&mut self, value: i64) -> io::Result<()> {
        self.write_i64(value, Endianness::Big)
    }


    /// Write a big endian i64 to stream
    ///
    /// # Parameters
    ///     `value` - The variable to write to the stream
    ///
    /// # Errors
    ///     io::Error during write
    ///     io::Error::UnexpectedEof when filestream could not write full buffer
    fn write_le_i64(&mut self, value: i64) -> io::Result<()> {
        self.write_i64(value, Endianness::Little)
    }


    /// Write a big endian i64 to stream
    ///
    /// # Parameters
    ///     `value` - The variable to write to the stream
    ///
    /// # Errors
    ///     io::Error during write
    ///     io::Error::UnexpectedEof when filestream could not write full buffer
    fn write_ne_i64(&mut self, value: i64) -> io::Result<()> {
        self.write_i64(value, *SYS_ENDIANNESS)
    }

    /// Write an f32 to stream
    ///
    /// # Parameters
    ///     `value` - The variable to write to the stream
    ///     `endianness` - Endianness of the stream to write to
    ///
    /// # Errors
    ///     io::Error during write
    ///     io::Error::UnexpectedEof when filestream could not write full buffer
    fn write_f32(&mut self, value: f32, endianness: Endianness) -> io::Result<()> {
        let mut bytes: [u8; 4] = value.to_ne_bytes();
        swap_inplace(bytes.as_mut_slice(), endianness);

        if self.write(&bytes)? != std::mem::size_of::<f32>() {
            return Err(Error::from(ErrorKind::UnexpectedEof));
        }

        Ok(())
    }


    /// Write a big endian f32 to stream
    ///
    /// # Parameters
    ///     `value` - The variable to write to the stream
    ///
    /// # Errors
    ///     io::Error during write
    ///     io::Error::UnexpectedEof when filestream could not write full buffer
    fn write_be_f32(&mut self, value: f32) -> io::Result<()> {
        self.write_f32(value, Endianness::Big)
    }


    /// Write a big endian f32 to stream
    ///
    /// # Parameters
    ///     `value` - The variable to write to the stream
    ///
    /// # Errors
    ///     io::Error during write
    ///     io::Error::UnexpectedEof when filestream could not write full buffer
    fn write_le_f32(&mut self, value: f32) -> io::Result<()> {
        self.write_f32(value, Endianness::Little)
    }


    /// Write a big endian f32 to stream
    ///
    /// # Parameters
    ///     `value` - The variable to write to the stream
    ///
    /// # Errors
    ///     io::Error during write
    ///     io::Error::UnexpectedEof when filestream could not write full buffer
    fn write_ne_f32(&mut self, value: f32) -> io::Result<()> {
        self.write_f32(value, *SYS_ENDIANNESS)
    }

    /// Write an f64 to stream
    ///
    /// # Parameters
    ///     `value` - The variable to write to the stream
    ///     `endianness` - Endianness of the stream to write to
    ///
    /// # Errors
    ///     io::Error during write
    ///     io::Error::UnexpectedEof when filestream could not write full buffer
    fn write_f64(&mut self, value: f64, endianness: Endianness) -> io::Result<()> {
        let mut bytes: [u8; 8] = value.to_ne_bytes();
        swap_inplace(bytes.as_mut_slice(), endianness);

        if self.write(&bytes)? != std::mem::size_of::<f64>() {
            return Err(Error::from(ErrorKind::UnexpectedEof));
        }

        Ok(())
    }


    /// Write a big endian f64 to stream
    ///
    /// # Parameters
    ///     `value` - The variable to write to the stream
    ///
    /// # Errors
    ///     io::Error during write
    ///     io::Error::UnexpectedEof when filestream could not write full buffer
    fn write_be_f64(&mut self, value: f64) -> io::Result<()> {
        self.write_f64(value, Endianness::Big)
    }


    /// Write a big endian f64 to stream
    ///
    /// # Parameters
    ///     `value` - The variable to write to the stream
    ///
    /// # Errors
    ///     io::Error during write
    ///     io::Error::UnexpectedEof when filestream could not write full buffer
    fn write_le_f64(&mut self, value: f64) -> io::Result<()> {
        self.write_f64(value, Endianness::Little)
    }


    /// Write a big endian f64 to stream
    ///
    /// # Parameters
    ///     `value` - The variable to write to the stream
    ///
    /// # Errors
    ///     io::Error during write
    ///     io::Error::UnexpectedEof when filestream could not write full buffer
    fn write_ne_f64(&mut self, value: f64) -> io::Result<()> {
        self.write_f64(value, *SYS_ENDIANNESS)
    }


    /// Write a fixed size string to stream
    ///
    /// # Parameters
    ///     `value` - The variable to write to the stream
    ///
    /// # Errors
    ///     io::Error during write
    ///     io::Error::UnexpectedEof when filestream could not write full buffer
    fn write_string(&mut self, value: &str) -> io::Result<()> {
        if self.write(value.as_bytes())? != value.len() {
            return Err(Error::from(ErrorKind::UnexpectedEof));
        }

        Ok(())
    }


    /// Write a null terminated string to stream
    ///
    /// # Parameters
    ///     `value` - The variable to write to the stream
    ///
    /// # Errors
    ///     io::Error during write
    ///     io::Error::UnexpectedEof when filestream could not write full buffer
    fn write_nt_string(&mut self, value: &str) -> io::Result<()> {
        if self.write(value.as_bytes())? != value.len() {
            return Err(Error::from(ErrorKind::UnexpectedEof));
        }
        self.write_u8(0u8)?;

        Ok(())
    }
}
