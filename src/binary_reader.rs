use std::{io::{Error, ErrorKind, Read}, sync::Arc};

use num_traits::PrimInt;

use super::endianness::{swap_inplace, Endianness, SYS_ENDIANNESS};

pub trait BinaryReader: Read  {
    fn read_raw(&mut self, size: usize) -> Result<Arc<[u8]>, Error>;
    fn read_to_delim(&mut self, delim: u8) -> Result<Arc<[u8]>, Error>;

    fn read_u8(&mut self) -> Result<u8, Error>;
    fn read_into_u8(&mut self, target: &mut u8) -> Result<(), Error>;

    fn read_i8(&mut self) -> Result<i8, Error>;
    fn read_into_i8(&mut self, target: &mut i8) -> Result<(), Error>;

    fn read_u16(&mut self, endianness: Endianness) -> Result<u16, Error>;
    fn read_le_u16(&mut self) -> Result<u16, Error>;
    fn read_be_u16(&mut self) -> Result<u16, Error>;
    fn read_ne_u16(&mut self) -> Result<u16, Error>;
    fn read_into_u16(&mut self, target: &mut u16, endianness: Endianness) -> Result<(), Error>;
    fn read_into_le_u16(&mut self, target: &mut u16) -> Result<(), Error>;
    fn read_into_be_u16(&mut self, target: &mut u16) -> Result<(), Error>;
    fn read_into_ne_u16(&mut self, target: &mut u16) -> Result<(), Error>;

    fn read_i16(&mut self, endianness: Endianness) -> Result<i16, Error>;
    fn read_le_i16(&mut self) -> Result<i16, Error>;
    fn read_be_i16(&mut self) -> Result<i16, Error>;
    fn read_ne_i16(&mut self) -> Result<i16, Error>;
    fn read_into_i16(&mut self, target: &mut i16, endianness: Endianness) -> Result<(), Error>;
    fn read_into_le_i16(&mut self, target: &mut i16) -> Result<(), Error>;
    fn read_into_be_i16(&mut self, target: &mut i16) -> Result<(), Error>;
    fn read_into_ne_i16(&mut self, target: &mut i16) -> Result<(), Error>;

    fn read_u32(&mut self, endianness: Endianness) -> Result<u32, Error>;
    fn read_le_u32(&mut self) -> Result<u32, Error>;
    fn read_be_u32(&mut self) -> Result<u32, Error>;
    fn read_ne_u32(&mut self) -> Result<u32, Error>;
    fn read_into_u32(&mut self, target: &mut u32, endianness: Endianness) -> Result<(), Error>;
    fn read_into_le_u32(&mut self, target: &mut u32) -> Result<(), Error>;
    fn read_into_be_u32(&mut self, target: &mut u32) -> Result<(), Error>;
    fn read_into_ne_u32(&mut self, target: &mut u32) -> Result<(), Error>;

    fn read_i32(&mut self, endianness: Endianness) -> Result<i32, Error>;
    fn read_le_i32(&mut self) -> Result<i32, Error>;
    fn read_be_i32(&mut self) -> Result<i32, Error>;
    fn read_ne_i32(&mut self) -> Result<i32, Error>;
    fn read_into_i32(&mut self, target: &mut i32, endianness: Endianness) -> Result<(), Error>;
    fn read_into_le_i32(&mut self, target: &mut i32) -> Result<(), Error>;
    fn read_into_be_i32(&mut self, target: &mut i32) -> Result<(), Error>;
    fn read_into_ne_i32(&mut self, target: &mut i32) -> Result<(), Error>;

    fn read_u64(&mut self, endianness: Endianness) -> Result<u64, Error>;
    fn read_le_u64(&mut self) -> Result<u64, Error>;
    fn read_be_u64(&mut self) -> Result<u64, Error>;
    fn read_ne_u64(&mut self) -> Result<u64, Error>;
    fn read_into_u64(&mut self, target: &mut u64, endianness: Endianness) -> Result<(), Error>;
    fn read_into_le_u64(&mut self, target: &mut u64) -> Result<(), Error>;
    fn read_into_be_u64(&mut self, target: &mut u64) -> Result<(), Error>;
    fn read_into_ne_u64(&mut self, target: &mut u64) -> Result<(), Error>;

    fn read_i64(&mut self, endianness: Endianness) -> Result<i64, Error>;
    fn read_le_i64(&mut self) -> Result<i64, Error>;
    fn read_be_i64(&mut self) -> Result<i64, Error>;
    fn read_ne_i64(&mut self) -> Result<i64, Error>;
    fn read_into_i64(&mut self, target: &mut i64, endianness: Endianness) -> Result<(), Error>;
    fn read_into_le_i64(&mut self, target: &mut i64) -> Result<(), Error>;
    fn read_into_be_i64(&mut self, target: &mut i64) -> Result<(), Error>;
    fn read_into_ne_i64(&mut self, target: &mut i64) -> Result<(), Error>;

    fn read_f32(&mut self, endianness: Endianness) -> Result<f32, Error>;
    fn read_le_f32(&mut self) -> Result<f32, Error>;
    fn read_be_f32(&mut self) -> Result<f32, Error>;
    fn read_ne_f32(&mut self) -> Result<f32, Error>;
    fn read_into_f32(&mut self, target: &mut f32, endianness: Endianness) -> Result<(), Error>;
    fn read_into_le_f32(&mut self, target: &mut f32) -> Result<(), Error>;
    fn read_into_be_f32(&mut self, target: &mut f32) -> Result<(), Error>;
    fn read_into_ne_f32(&mut self, target: &mut f32) -> Result<(), Error>;

    fn read_f64(&mut self, endianness: Endianness) -> Result<f64, Error>;
    fn read_le_f64(&mut self) -> Result<f64, Error>;
    fn read_be_f64(&mut self) -> Result<f64, Error>;
    fn read_ne_f64(&mut self) -> Result<f64, Error>;
    fn read_into_f64(&mut self, target: &mut f64, endianness: Endianness) -> Result<(), Error>;
    fn read_into_le_f64(&mut self, target: &mut f64) -> Result<(), Error>;
    fn read_into_be_f64(&mut self, target: &mut f64) -> Result<(), Error>;
    fn read_into_ne_f64(&mut self, target: &mut f64) -> Result<(), Error>;

    fn read_fixed_size_string(&mut self, size: usize) -> Result<String, Error>;
    fn read_into_fixed_size_string(
        &mut self,
        size: usize,
        target: &mut String,
    ) -> Result<(), Error>;

    fn read_null_terminated_string(&mut self) -> Result<String, Error>;
    fn read_into_null_terminated_string(&mut self, target: &mut String) -> Result<(), Error>;

    fn read_size_prefixed_string<PrefixT: PrimInt>(
        &mut self,
        endianness: Endianness,
    ) -> Result<String, Error>;
    fn read_into_size_prefixed_string<PrefixT: PrimInt>(
        &mut self,
        target: &mut String,
        endianness: Endianness,
    ) -> Result<(), Error>;
}

impl<StreamT: Read > BinaryReader for StreamT {
    /// Read a [u8] from stream. Similar to the base read function, but instead it returns
    /// an array instead of reading into an array. 
    ///
    /// # Parameters
    ///    size - The size in bytes to read from stream
    ///
    /// # Error
    ///     io::Error                - Error bubbled up from stream read
    ///     io::Error::UnexpectedEof - There wasn't enough data in the stream for data
    ///
    /// # Returns
    ///     Arc<[u8]> - The data read from the stream
    fn read_raw(&mut self, size: usize) -> Result<Arc<[u8]>, Error> {
        let mut buffer = vec![0u8; size];
        let bytes_read = self.read(&mut buffer)?;
        if bytes_read != size {
            return Err(Error::from(ErrorKind::UnexpectedEof));
        }
        Ok(buffer.into())
    }

    /// Reads data from stream until a delimiter is encountered or EOF reached. 
    ///
    /// # Parameters
    ///    delim - The delimiter to stop parsing at
    ///
    /// # Error
    ///     io::Error                - Error bubbled up from stream read
    ///     io::Error::UnexpectedEof - There wasn't enough data in the stream for data
    ///
    /// # Returns
    ///     Arc<[u8]> - The data read from the stream
    fn read_to_delim(&mut self, delim: u8) -> Result<Arc<[u8]>, Error> {
        let mut buffer = Vec::<u8>::new();
        loop {
            let byte = self.read_u8()?;
            if byte == delim {
                break;
            }
            buffer.push(byte);
        }
        return Ok(buffer.into());
    }

    // ------------------------------------------------------------------------------- u8
    /// Read an u8 from stream
    ///
    /// # Error
    ///     io::Error                - Error bubbled up from stream read
    ///     io::Error::UnexpectedEof - There wasn't enough data in the stream for data
    ///
    /// # Returns
    ///     u8 - The value read from the stream
    fn read_u8(&mut self) -> Result<u8, Error> {
        let mut buffer: [u8; 1] = [0u8; 1];

        if self.read(&mut buffer)? != std::mem::size_of::<u8>() {
            return Err(Error::from(ErrorKind::UnexpectedEof));
        }

        Ok(buffer[0])
    }

    /// Read an u8 from stream into an existing variable
    ///
    /// # Parameters
    ///    target - The destination variable to read into
    ///
    /// # Error
    ///     io::Error during read
    ///     io::Error::UnexpectedEof if there wasn't enough data in the stream
    fn read_into_u8(&mut self, target: &mut u8) -> Result<(), Error> {
        *target = self.read_u8()?;

        Ok(())
    }

    // ------------------------------------------------------------------------------- i8
    /// Read an i8 from stream
    ///
    /// # Error
    ///     io::Error during read
    ///     io::Error::UnexpectedEof if there wasn't enough data in the stream
    ///
    /// # Returns
    ///     i8 - The value read from the stream
    fn read_i8(&mut self) -> Result<i8, Error> {
        let mut buffer: [u8; 1] = [0u8; 1];

        if self.read(&mut buffer)? != std::mem::size_of::<i8>() {
            return Err(Error::from(ErrorKind::UnexpectedEof));
        }

        Ok(buffer[0] as i8)
    }

    /// Read an i8 from stream into an existing variable
    ///
    /// # Parameters
    ///    target - The destination variable to read into
    ///
    /// # Error
    ///     io::Error during read
    ///     io::Error::UnexpectedEof if there wasn't enough data in the stream
    fn read_into_i8(&mut self, target: &mut i8) -> Result<(), Error> {
        *target = self.read_i8()?;

        Ok(())
    }

    // ------------------------------------------------------------------------------- u16
    /// Read an u16 from stream
    ///
    /// # Parameters
    ///    endianness - The endianness of the data to be read
    ///
    /// # Error
    ///    io::Error during read
    ///    io::Error::UnexpectedEof if there wasn't enough data in the stream
    ///
    /// # Returns
    ///    u16 - The value read from the stream
    fn read_u16(&mut self, endianness: Endianness) -> Result<u16, Error> {
        let mut buffer: [u8; 2] = [0u8; 2];

        if self.read(&mut buffer)? != std::mem::size_of::<u16>() {
            return Err(Error::from(ErrorKind::UnexpectedEof));
        }
        swap_inplace(&mut buffer, endianness);

        Ok(u16::from_ne_bytes(buffer))
    }

    /// Read a little endian u16 from stream
    ///
    /// # Error
    ///     io::Error during read
    ///     io::Error::UnexpectedEof if there wasn't enough data in the stream
    ///
    /// # Returns
    ///     u16 - The little endian value read from the stream
    fn read_le_u16(&mut self) -> Result<u16, Error> {
        self.read_u16(Endianness::Little)
    }

    /// Read a big endian u16 from stream
    ///
    /// # Error
    ///     io::Error during read
    ///     io::Error::UnexpectedEof if there wasn't enough data in the stream
    ///
    /// # Returns
    ///      u16 - The big endian value read from the stream
    fn read_be_u16(&mut self) -> Result<u16, Error> {
        self.read_u16(Endianness::Big)
    }

    /// Read a native endian u16 from stream
    ///
    /// # Error
    ///     io::Error during read
    ///     io::Error::UnexpectedEof if there wasn't enough data in the stream
    ///
    /// # Returns
    ///      u16 - The native endian value read from the stream
    fn read_ne_u16(&mut self) -> Result<u16, Error> {
        self.read_u16(*SYS_ENDIANNESS)
    }

    /// Read an u16 into a variable
    ///
    /// # Parameters
    ///    target - The destination variable to read into
    ///
    /// # Parameters
    ///     `target`     - The variable to store the value in
    ///     `endianness` - The endianness to use when reading the value from the stream
    ///
    /// # Error
    ///     io::Error during read
    ///     io::Error::UnexpectedEof if there wasn't enough data in the stream
    fn read_into_u16(&mut self, target: &mut u16, endianness: Endianness) -> Result<(), Error> {
        *target = self.read_u16(endianness)?;

        Ok(())
    }

    /// Read a little endian u16 into a variable
    ///
    /// # Parameters
    ///    target - The destination variable to read into
    ///
    /// # Parameters
    ///     `target` - The variable to store the value in
    ///
    /// # Error
    ///     io::Error during read
    ///     io::Error::UnexpectedEof if there wasn't enough data in the stream
    fn read_into_le_u16(&mut self, target: &mut u16) -> Result<(), Error> {
        *target = self.read_u16(Endianness::Little)?;

        Ok(())
    }

    /// Read a big endian u16 into a variable
    ///
    /// # Parameters
    ///    target - The destination variable to read into
    ///
    /// # Parameters
    ///     `target` - The variable to store the value in
    ///
    /// # Error
    ///     io::Error during read
    ///     io::Error::UnexpectedEof when there isn't enough data in the stream
    fn read_into_be_u16(&mut self, target: &mut u16) -> Result<(), Error> {
        *target = self.read_u16(Endianness::Big)?;

        Ok(())
    }

    /// Read a native endian u16 into a variable
    ///
    /// # Parameters
    ///    target - The destination variable to read into
    ///
    /// # Parameters
    ///     `target` - The variable to store the value in
    ///
    /// # Error
    ///     io::Error during read
    ///     io::Error::UnexpectedEof when there isn't enough data in the stream
    fn read_into_ne_u16(&mut self, target: &mut u16) -> Result<(), Error> {
        *target = self.read_u16(*SYS_ENDIANNESS)?;

        Ok(())
    }

    // ------------------------------------------------------------------------------- i16
    /// Read an i16 from a stream
    ///
    /// # Parameters
    ///     `endianness` - The endianness of the data to read
    ///
    /// # Error
    ///     io::Error during read
    ///     io::Error::UnexpectedEof when there isn't enough data in the stream
    ///
    /// # Returns
    ///     i16 - The value read from the stream
    fn read_i16(&mut self, endianness: Endianness) -> Result<i16, Error> {
        let mut buffer: [u8; 2] = [0u8; 2];

        if self.read(&mut buffer)? != std::mem::size_of::<i16>() {
            return Err(Error::from(ErrorKind::UnexpectedEof));
        }
        swap_inplace(&mut buffer, endianness);

        Ok(i16::from_ne_bytes(buffer))
    }

    /// read a little endian i16 from a stream
    ///
    /// # Parameters
    ///     `endianness` - The endianness of the data to read
    ///
    /// # Error
    ///     io::Error during read
    ///     io::Error::UnexpectedEof when there isn't enough data in the stream
    ///
    /// # Returns
    ///     i16 - The little endian value read from the stream
    fn read_le_i16(&mut self) -> Result<i16, Error> {
        self.read_i16(Endianness::Little)
    }

    /// read a big endian i16 from a stream
    ///
    /// # Parameters
    ///     `endianness` - The endianness of the data to read
    ///
    /// # Error
    ///     io::Error during read
    ///     io::Error::UnexpectedEof when there isn't enough data in the stream
    ///
    /// # Returns
    ///     i16 - The big endian value read from the stream
    fn read_be_i16(&mut self) -> Result<i16, Error> {
        self.read_i16(Endianness::Big)
    }

    /// read a native endian i16 from a stream
    ///
    /// # Parameters
    ///     `endianness` - The endianness of the data to read
    ///
    /// # Error
    ///     io::Error during read
    ///     io::Error::UnexpectedEof when there isn't enough data in the stream
    ///
    /// # Returns
    ///     i16 - The native endian value read from the stream
    fn read_ne_i16(&mut self) -> Result<i16, Error> {
        self.read_i16(*SYS_ENDIANNESS)
    }

    /// Read a u16 into a variable
    ///
    /// # Parameters
    ///     `target` - The variable to store the value in
    ///     `endianness` - The endianness of the data to read
    ///
    /// # Error
    ///     io::Error during read
    ///     io::Error::UnexpectedEof when there isn't enough data in the stream
    fn read_into_i16(&mut self, target: &mut i16, endianness: Endianness) -> Result<(), Error> {
        *target = self.read_i16(endianness)?;

        Ok(())
    }

    /// Read a little endian i16 into a variable
    ///
    /// # Parameters
    ///     `target` - The variable to store the value in
    ///
    /// # Error
    ///     io::Error during read
    ///     io::Error::UnexpectedEof when there isn't enough data in the stream
    fn read_into_le_i16(&mut self, target: &mut i16) -> Result<(), Error> {
        *target = self.read_i16(Endianness::Little)?;

        Ok(())
    }

    /// Read a big endian i16 into a variable
    ///
    /// # Parameters
    ///     `target` - The variable to store the value in
    ///
    /// # Error
    ///     io::Error during read
    ///     io::Error::UnexpectedEof when there isn't enough data in the stream
    fn read_into_be_i16(&mut self, target: &mut i16) -> Result<(), Error> {
        *target = self.read_i16(Endianness::Big)?;

        Ok(())
    }

    /// Read a native endian i16 into a variable
    ///
    /// # Parameters
    ///     `target` - The variable to store the value in
    ///
    /// # Error
    ///     io::Error during read
    ///     io::Error::UnexpectedEof when there isn't enough data in the stream
    fn read_into_ne_i16(&mut self, target: &mut i16) -> Result<(), Error> {
        *target = self.read_i16(*SYS_ENDIANNESS)?;

        Ok(())
    }

    // ------------------------------------------------------------------------------- u32
    /// read a u32 from the stream
    ///
    /// # Parameters
    ///     `endianness` - The endianness of the data to read
    ///
    /// # Error
    ///     io::Error during read
    ///     io::Error::UnexpectedEof when there isn't enough data in the stream
    ///
    /// # Returns
    ///     u32 - the value read from the stream
    fn read_u32(&mut self, endianness: Endianness) -> Result<u32, Error> {
        let mut buffer: [u8; 4] = [0u8; 4];

        if self.read(&mut buffer)? != std::mem::size_of::<u32>() {
            return Err(Error::from(ErrorKind::UnexpectedEof));
        }
        swap_inplace(&mut buffer, endianness);

        Ok(u32::from_ne_bytes(buffer))
    }

    /// Read a little endian u32 from the stream
    ///
    /// # Error
    ///     io::Error during read
    ///     io::Error::UnexpectedEof when there isn't enough data in the stream
    ///
    /// # Returns
    ///    u32 - the little endian u32 value read from the stream
    fn read_le_u32(&mut self) -> Result<u32, Error> {
        self.read_u32(Endianness::Little)
    }

    /// Read a big endian u32 from the stream
    ///
    /// # Errors
    ///     io::Error during read
    ///     io::Error::UnexpectedEof when there isn't enough data in the stream
    ///
    /// # Returns
    ///    u32 - the big endian u32 value read from the stream
    fn read_be_u32(&mut self) -> Result<u32, Error> {
        self.read_u32(Endianness::Big)
    }

    /// Read a native endian u32 from the stream
    ///
    /// # Errors
    ///     io::Error during read
    ///     io::Error::UnexpectedEof when there isn't enough data in the stream
    ///
    /// # Returns
    ///    u32 - the native endian u32 value read from the stream
    fn read_ne_u32(&mut self) -> Result<u32, Error> {
        self.read_u32(*SYS_ENDIANNESS)
    }

    /// Read a u32 into a variable
    ///
    /// # Parameters
    ///     `target` - The variable to store the value in
    ///
    /// # Errors
    ///     io::Error during read
    ///     io::Error::UnexpectedEof when there isn't enough data in the stream
    fn read_into_u32(&mut self, target: &mut u32, endianness: Endianness) -> Result<(), Error> {
        *target = self.read_u32(endianness)?;

        Ok(())
    }

    /// Read a little endian u32 into a variable
    ///
    /// # Parameters
    ///     `target` - The variable to store the value in
    ///
    /// # Errors
    ///     io::Error during read
    ///     io::Error::UnexpectedEof when there isn't enough data in the stream
    fn read_into_le_u32(&mut self, target: &mut u32) -> Result<(), Error> {
        *target = self.read_u32(Endianness::Little)?;

        Ok(())
    }

    /// Read a big endian u32 into a variable
    ///
    /// # Parameters
    ///     `target` - The variable to store the value in
    ///
    /// # Errors
    ///     io::Error during read
    ///     io::Error::UnexpectedEof when there isn't enough data in the stream
    fn read_into_be_u32(&mut self, target: &mut u32) -> Result<(), Error> {
        *target = self.read_u32(Endianness::Big)?;

        Ok(())
    }

    /// Read a native endian u32 into a variable
    ///
    /// # Parameters
    ///     `target` - The variable to store the value in
    ///
    /// # Errors
    ///     io::Error during read
    ///     io::Error::UnexpectedEof when there isn't enough data in the stream
    fn read_into_ne_u32(&mut self, target: &mut u32) -> Result<(), Error> {
        *target = self.read_u32(*SYS_ENDIANNESS)?;

        Ok(())
    }

    // ------------------------------------------------------------------------------- i32
    /// Read an i32 from the stream
    ///
    /// # Parameters
    ///    `endianness` - Endianness of the stream to read from
    ///
    /// # Errors
    ///     io::Error during read
    ///     io::Error::UnexpectedEof when there isn't enough data in the stream
    ///
    /// # Returns
    ///     i32 value read from the stream
    fn read_i32(&mut self, endianness: Endianness) -> Result<i32, Error> {
        let mut buffer: [u8; 4] = [0u8; 4];

        if self.read(&mut buffer)? != std::mem::size_of::<i32>() {
            return Err(Error::from(ErrorKind::UnexpectedEof));
        }
        swap_inplace(&mut buffer, endianness);

        Ok(i32::from_ne_bytes(buffer))
    }

    /// Read a little endian i32 into a variable
    ///
    /// # Errors
    ///     io::Error during read
    ///     io::Error::UnexpectedEof when there isn't enough data in the stream
    ///
    /// # Returns
    ///     i32 value read from the stream
    fn read_le_i32(&mut self) -> Result<i32, Error> {
        self.read_i32(Endianness::Little)
    }

    /// Read a big endian i32 into a variable
    ///
    /// # Errors
    ///     io::Error during read
    ///     io::Error::UnexpectedEof when there isn't enough data in the stream
    ///
    /// # Returns
    ///     i32 value read from the stream
    fn read_be_i32(&mut self) -> Result<i32, Error> {
        self.read_i32(Endianness::Big)
    }

    /// Read a native endian i32 into a variable
    ///
    /// # Errors
    ///     io::Error during read
    ///     io::Error::UnexpectedEof when there isn't enough data in the stream
    ///
    /// # Returns
    ///     i32 value read from the stream
    fn read_ne_i32(&mut self) -> Result<i32, Error> {
        self.read_i32(*SYS_ENDIANNESS)
    }

    /// Read i32 into a variable
    ///
    /// # Parameters
    ///     `target` - The variable to store the value in
    ///     `endianness` - The endianness of the data to read
    ///
    /// # Errors
    ///     io::Error during read
    ///     io::Error::UnexpectedEof when there isn't enough data in the stream
    fn read_into_i32(&mut self, target: &mut i32, endianness: Endianness) -> Result<(), Error> {
        *target = self.read_i32(endianness)?;

        Ok(())
    }

    /// Read a little endian i32 into a variable
    ///
    /// # Parameters
    ///     `target` - The variable to store the value in
    ///
    /// # Errors
    ///     io::Error during read
    ///     io::Error::UnexpectedEof when there isn't enough data in the stream
    fn read_into_le_i32(&mut self, target: &mut i32) -> Result<(), Error> {
        *target = self.read_i32(Endianness::Little)?;

        Ok(())
    }

    /// Read a big endian i32 into a variable
    ///
    /// # Parameters
    ///     `target` - The variable to store the value in
    ///
    /// # Errors
    ///     io::Error during read
    ///     io::Error::UnexpectedEof when there isn't enough data in the stream
    fn read_into_be_i32(&mut self, target: &mut i32) -> Result<(), Error> {
        *target = self.read_i32(Endianness::Big)?;

        Ok(())
    }

    /// Read a native endian i32 into a variable
    ///
    /// # Parameters
    ///     `target` - The variable to store the value in
    ///
    /// # Errors
    ///     io::Error during read
    ///     io::Error::UnexpectedEof when there isn't enough data in the stream
    fn read_into_ne_i32(&mut self, target: &mut i32) -> Result<(), Error> {
        *target = self.read_i32(*SYS_ENDIANNESS)?;

        Ok(())
    }

    // ------------------------------------------------------------------------------- u64
    /// Read an u64 from the stream
    ///
    /// # Parameters
    ///    `endianness` - Endianness of the stream to read from
    ///
    /// # Errors
    ///     io::Error during read
    ///     io::Error::UnexpectedEof when there isn't enough data in the stream
    ///
    /// # Returns
    ///     u64 value read from the stream
    fn read_u64(&mut self, endianness: Endianness) -> Result<u64, Error> {
        let mut buffer: [u8; 8] = [0u8; 8];

        if self.read(&mut buffer)? != std::mem::size_of::<u64>() {
            return Err(Error::from(ErrorKind::UnexpectedEof));
        }
        swap_inplace(&mut buffer, endianness);

        Ok(u64::from_ne_bytes(buffer))
    }

    /// Read a little endian u64
    ///
    /// # Errors
    ///     io::Error during read
    ///     io::Error::UnexpectedEof when there isn't enough data in the stream
    ///
    /// # Returns
    ///     u64 value read from the stream
    fn read_le_u64(&mut self) -> Result<u64, Error> {
        self.read_u64(Endianness::Little)
    }

    /// Read a big endian u64
    ///
    /// # Errors
    ///     io::Error during read
    ///     io::Error::UnexpectedEof when there isn't enough data in the stream
    ///
    /// # Returns
    ///     u64 value read from the stream
    fn read_be_u64(&mut self) -> Result<u64, Error> {
        self.read_u64(Endianness::Big)
    }

    /// Read a native endian u64
    ///
    /// # Errors
    ///     io::Error during read
    ///     io::Error::UnexpectedEof when there isn't enough data in the stream
    ///
    /// # Returns
    ///     u64 value read from the stream
    fn read_ne_u64(&mut self) -> Result<u64, Error> {
        self.read_u64(*SYS_ENDIANNESS)
    }

    /// Read a u64 into a variable
    ///
    /// # Errors
    ///     io::Error during read
    ///     io::Error::UnexpectedEof when there isn't enough data in the stream
    ///
    /// # Parameters
    ///     `endianness` - Endianness of the stream to read from
    ///     `target` - The variable to store the value in
    ///
    /// # Returns
    ///     u64 value read from the stream
    fn read_into_u64(&mut self, target: &mut u64, endianness: Endianness) -> Result<(), Error> {
        *target = self.read_u64(endianness)?;

        Ok(())
    }

    /// Read a little endian u64 into a variable
    ///
    /// # Parameters
    ///     `target` - The variable to store the value in
    ///
    /// # Errors
    ///     io::Error during read
    ///     io::Error::UnexpectedEof when there isn't enough data in the stream
    fn read_into_le_u64(&mut self, target: &mut u64) -> Result<(), Error> {
        *target = self.read_u64(Endianness::Little)?;

        Ok(())
    }

    /// Read a big endian u64 into a variable
    ///
    /// # Parameters
    ///     `target` - The variable to store the value in
    ///
    /// # Errors
    ///     io::Error during read
    ///     io::Error::UnexpectedEof when there isn't enough data in the stream
    fn read_into_be_u64(&mut self, target: &mut u64) -> Result<(), Error> {
        *target = self.read_u64(Endianness::Big)?;

        Ok(())
    }

    /// Read a native endian u64 into a variable
    ///
    /// # Parameters
    ///     `target` - The variable to store the value in
    ///
    /// # Errors
    ///     io::Error during read
    ///     io::Error::UnexpectedEof when there isn't enough data in the stream
    fn read_into_ne_u64(&mut self, target: &mut u64) -> Result<(), Error> {
        *target = self.read_u64(*SYS_ENDIANNESS)?;

        Ok(())
    }

    // ------------------------------------------------------------------------------- i64
    /// Read an i64 from the stream
    ///
    /// # Errors
    ///     io::Error during read
    ///     io::Error::UnexpectedEof when there isn't enough data in the stream
    ///
    /// # Parameters
    ///    `endianness` - Endianness of the stream to read from
    ///
    /// # Returns
    ///     i64 value read from the stream
    fn read_i64(&mut self, endianness: Endianness) -> Result<i64, Error> {
        let mut buffer: [u8; 8] = [0u8; 8];

        if self.read(&mut buffer)? != std::mem::size_of::<i64>() {
            return Err(Error::from(ErrorKind::UnexpectedEof));
        }
        swap_inplace(&mut buffer, endianness);

        Ok(i64::from_ne_bytes(buffer))
    }

    /// Read a little endian i64
    ///
    /// # Errors
    ///     io::Error during read
    ///     io::Error::UnexpectedEof when there isn't enough data in the stream
    ///
    /// # Returns
    ///     u64 value read from the stream
    fn read_le_i64(&mut self) -> Result<i64, Error> {
        self.read_i64(Endianness::Little)
    }

    /// Read a big endian i64
    ///
    /// # Errors
    ///     io::Error during read
    ///     io::Error::UnexpectedEof when there isn't enough data in the stream
    ///
    /// # Returns
    ///     u64 value read from the stream
    fn read_be_i64(&mut self) -> Result<i64, Error> {
        self.read_i64(Endianness::Big)
    }

    /// Read a native endian i64
    ///
    /// # Errors
    ///     io::Error during read
    ///     io::Error::UnexpectedEof when there isn't enough data in the stream
    ///
    /// # Returns
    ///     u64 value read from the stream
    fn read_ne_i64(&mut self) -> Result<i64, Error> {
        self.read_i64(*SYS_ENDIANNESS)
    }

    /// Read a i64 into a variable
    ///
    /// # Errors
    ///     io::Error during read
    ///     io::Error::UnexpectedEof when there isn't enough data in the stream
    ///
    /// # Parameters
    ///     `endianness` - Endianness of the stream to read from
    ///     `target` - The variable to store the value in
    ///
    /// # Returns
    ///     i64 value read from the stream
    fn read_into_i64(&mut self, target: &mut i64, endianness: Endianness) -> Result<(), Error> {
        *target = self.read_i64(endianness)?;

        Ok(())
    }

    /// Read a little endian u64 into a variable
    ///
    /// # Parameters
    ///     `target` - The variable to store the value in
    ///
    /// # Errors
    ///     io::Error during read
    ///     io::Error::UnexpectedEof when there isn't enough data in the stream
    fn read_into_le_i64(&mut self, target: &mut i64) -> Result<(), Error> {
        *target = self.read_i64(Endianness::Little)?;

        Ok(())
    }

    /// Read a big endian u64 into a variable
    ///
    /// # Parameters
    ///     `target` - The variable to store the value in
    ///
    /// # Errors
    ///     io::Error during read
    ///     io::Error::UnexpectedEof when there isn't enough data in the stream
    fn read_into_be_i64(&mut self, target: &mut i64) -> Result<(), Error> {
        *target = self.read_i64(Endianness::Big)?;

        Ok(())
    }

    /// Read a native endian u64 into a variable
    ///
    /// # Parameters
    ///     `target` - The variable to store the value in
    ///
    /// # Errors
    ///     io::Error during read
    ///     io::Error::UnexpectedEof when there isn't enough data in the stream
    fn read_into_ne_i64(&mut self, target: &mut i64) -> Result<(), Error> {
        *target = self.read_i64(*SYS_ENDIANNESS)?;

        Ok(())
    }

    // ------------------------------------------------------------------------------- f32
    /// Read an f32 from the stream
    ///
    /// # Errors
    ///     io::Error during read
    ///     io::Error::UnexpectedEof when there isn't enough data in the stream
    ///
    /// # Parameters
    ///    `endianness` - Endianness of the stream to read from
    ///
    /// # Returns
    ///     f32 value read from the stream
    fn read_f32(&mut self, endianness: Endianness) -> Result<f32, Error> {
        let mut buffer: [u8; 4] = [0u8; 4];

        if self.read(&mut buffer)? != std::mem::size_of::<f32>() {
            return Err(Error::from(ErrorKind::UnexpectedEof));
        }
        swap_inplace(&mut buffer, endianness);

        Ok(f32::from_ne_bytes(buffer))
    }

    /// Read a little endian f32
    ///
    /// # Errors
    ///     io::Error during read
    ///     io::Error::UnexpectedEof when there isn't enough data in the stream
    ///
    /// # Returns
    ///     f32 value read from the stream
    fn read_le_f32(&mut self) -> Result<f32, Error> {
        self.read_f32(Endianness::Little)
    }

    /// Read a big endian f32
    ///
    /// # Errors
    ///     io::Error during read
    ///     io::Error::UnexpectedEof when there isn't enough data in the stream
    ///
    /// # Returns
    ///     f32 value read from the stream
    fn read_be_f32(&mut self) -> Result<f32, Error> {
        self.read_f32(Endianness::Big)
    }

    /// Read a native endian f32
    ///
    /// # Errors
    ///     io::Error during read
    ///     io::Error::UnexpectedEof when there isn't enough data in the stream
    ///
    /// # Returns
    ///     f32 value read from the stream
    fn read_ne_f32(&mut self) -> Result<f32, Error> {
        self.read_f32(*SYS_ENDIANNESS)
    }

    /// Read a f32 into a variable
    ///
    /// # Errors
    ///     io::Error during read
    ///     io::Error::UnexpectedEof when there isn't enough data in the stream
    ///
    /// # Parameters
    ///     `endianness` - Endianness of the stream to read from
    ///
    /// # Returns
    ///     f32 value read from the stream
    fn read_into_f32(&mut self, target: &mut f32, endianness: Endianness) -> Result<(), Error> {
        *target = self.read_f32(endianness)?;

        Ok(())
    }

    /// Read a little endian f32 into a variable
    ///
    /// # Parameters
    ///     `target` - The variable to store the value in
    ///
    /// # Errors
    ///     io::Error during read
    ///     io::Error::UnexpectedEof when there isn't enough data in the stream
    fn read_into_le_f32(&mut self, target: &mut f32) -> Result<(), Error> {
        *target = self.read_f32(Endianness::Little)?;

        Ok(())
    }

    /// Read a big endian f32 into a variable
    ///
    /// # Parameters
    ///     `target` - The variable to store the value in
    ///
    /// # Errors
    ///     io::Error during read
    ///     io::Error::UnexpectedEof when there isn't enough data in the stream
    fn read_into_be_f32(&mut self, target: &mut f32) -> Result<(), Error> {
        *target = self.read_f32(Endianness::Big)?;

        Ok(())
    }

    /// Read a native endian f32 into a variable
    ///
    /// # Parameters
    ///     `target` - The variable to store the value in
    ///
    /// # Errors
    ///     io::Error during read
    ///     io::Error::UnexpectedEof when there isn't enough data in the stream
    fn read_into_ne_f32(&mut self, target: &mut f32) -> Result<(), Error> {
        *target = self.read_f32(Endianness::Big)?;

        Ok(())
    }

    // ------------------------------------------------------------------------------- f64
    /// Read an f64 from the stream
    ///
    /// # Errors
    ///     io::Error during read
    ///     io::Error::UnexpectedEof when there isn't enough data in the stream
    ///
    /// # Parameters
    ///    `endianness` - Endianness of the stream to read from
    ///
    /// # Returns
    ///     f64 value read from the stream
    fn read_f64(&mut self, endianness: Endianness) -> Result<f64, Error> {
        let mut buffer: [u8; 8] = [0u8; 8];

        if self.read(&mut buffer)? != std::mem::size_of::<f64>() {
            return Err(Error::from(ErrorKind::UnexpectedEof));
        }
        swap_inplace(&mut buffer, endianness);

        Ok(f64::from_ne_bytes(buffer))
    }

    /// Read a little endian f64
    ///
    /// # Errors
    ///     io::Error during read
    ///     io::Error::UnexpectedEof when there isn't enough data in the stream
    ///
    /// # Returns
    ///     f64 value read from the stream
    fn read_le_f64(&mut self) -> Result<f64, Error> {
        self.read_f64(Endianness::Little)
    }

    /// Read a big endian f64
    ///
    /// # Errors
    ///     io::Error during read
    ///     io::Error::UnexpectedEof when there isn't enough data in the stream
    ///
    /// # Returns
    ///     f64 value read from the stream
    fn read_be_f64(&mut self) -> Result<f64, Error> {
        self.read_f64(Endianness::Big)
    }

    /// Read a native endian f64
    ///
    /// # Errors
    ///     io::Error during read
    ///     io::Error::UnexpectedEof when there isn't enough data in the stream
    ///
    /// # Returns
    ///     f64 value read from the stream
    fn read_ne_f64(&mut self) -> Result<f64, Error> {
        self.read_f64(Endianness::Big)
    }

    /// Read a f64 into a variable
    ///
    /// # Errors
    ///     io::Error during read
    ///     io::Error::UnexpectedEof when there isn't enough data in the stream
    ///
    /// # Parameters
    ///     `endianness` - Endianness of the stream to read from
    ///     `target` - The variable to store the value in
    ///
    /// # Returns
    ///     f64 value read from the stream
    fn read_into_f64(&mut self, target: &mut f64, endianness: Endianness) -> Result<(), Error> {
        *target = self.read_f64(endianness)?;

        Ok(())
    }

    /// Read a little endian f64 into a variable
    ///
    /// # Parameters
    ///     `target` - The variable to store the value in
    ///
    /// # Errors
    ///     io::Error during read
    ///     io::Error::UnexpectedEof when there isn't enough data in the stream
    fn read_into_le_f64(&mut self, target: &mut f64) -> Result<(), Error> {
        *target = self.read_f64(Endianness::Little)?;

        Ok(())
    }

    /// Read a big endian f64 into a variable
    ///
    /// # Parameters
    ///     `target` - The variable to store the value in
    ///
    /// # Errors
    ///     io::Error during read
    ///     io::Error::UnexpectedEof when there isn't enough data in the stream
    fn read_into_be_f64(&mut self, target: &mut f64) -> Result<(), Error> {
        *target = self.read_f64(Endianness::Big)?;

        Ok(())
    }

    /// Read a native endian f64 into a variable
    ///
    /// # Parameters
    ///     `target` - The variable to store the value in
    ///
    /// # Errors
    ///     io::Error during read
    ///     io::Error::UnexpectedEof when there isn't enough data in the stream
    fn read_into_ne_f64(&mut self, target: &mut f64) -> Result<(), Error> {
        *target = self.read_f64(Endianness::Big)?;

        Ok(())
    }

    // ------------------------------------------------------------------------------- Strings
    /// Read an fixed length string from the stream
    ///
    /// # Parameters
    ///     `size` - The size of the string to read from stream
    ///
    /// # Errors
    ///     io::Error                - during read
    ///     io::Error::UnexpectedEof - when there isn't enough data in the stream
    ///     io::Error::InvalidData   - When non-utf-8 data is read from stream
    ///
    /// # Returns
    ///     String value read from the stream
    fn read_fixed_size_string(&mut self, size: usize) -> Result<String, Error> {
        let mut buffer: Vec<u8> = vec![0u8; size];

        if self.read(&mut buffer)? != size {
            return Err(Error::from(ErrorKind::UnexpectedEof));
        }

        let string: String = {
            match String::from_utf8(buffer) {
                Ok(value) => value,
                Err(_) => {
                    return Err(Error::new(
                        ErrorKind::InvalidData,
                        "Unable to convert bytes to string.",
                    ))
                }
            }
        };

        Ok(string)
    }

    /// Read a fixed size string into a variable
    ///
    /// # Parameters
    ///     `size` - The size of the string to read from stream
    ///     `target` - The variable to store the value in
    ///
    /// # Errors
    ///     io::Error during read
    ///     io::Error::UnexpectedEof when there isn't enough data in the stream
    ///
    /// # Returns
    ///     u64 value read from the stream
    fn read_into_fixed_size_string(
        &mut self,
        size: usize,
        target: &mut String,
    ) -> Result<(), Error> {
        *target = self.read_fixed_size_string(size)?;

        Ok(())
    }

    /// Read a a null terminated string from the stream
    ///
    /// # Errors
    ///     io::Error during read
    ///     io::Error::UnexpectedEof when there isn't enough data in the stream
    ///
    /// # Returns
    ///     u64 value read from the stream
    fn read_null_terminated_string(&mut self) -> Result<String, Error> {
        let buffer = self.read_to_delim(0)?;
        Ok(String::from_utf8(buffer.to_vec()).or(Err(Error::from(ErrorKind::InvalidData)))?)
    }

    /// Read a null terminated string into a variable
    ///
    /// # Parameters
    ///     `target` - The variable to store the value in
    ///
    /// # Errors
    ///     io::Error during read
    ///     io::Error::UnexpectedEof when there isn't enough data in the stream
    fn read_into_null_terminated_string(&mut self, target: &mut String) -> Result<(), Error> {
        *target = self.read_null_terminated_string()?;
        Ok(())
    }

    /// Read a size prefixed string from the stream
    ///
    /// # Parameters
    ///     `endianness` - Endianness of the stream to read from
    ///
    /// # Errors
    ///     io::Error during read
    ///     io::Error::UnexpectedEof when there isn't enough data in the stream
    ///
    /// # Returns
    ///     u64 value read from the stream
    fn read_size_prefixed_string<PrefixT: PrimInt>(
        &mut self,
        endianness: Endianness,
    ) -> Result<String, Error> {
        let mut buffer: [u8; 8] = [0u8; std::mem::size_of::<usize>()];

        let size = match endianness {
            Endianness::Little => {
                self.read(&mut buffer[..std::mem::size_of::<PrefixT>()])?;
                usize::from_le_bytes(buffer)
            }
            Endianness::Big => {
                self.read(&mut buffer[8 - std::mem::size_of::<PrefixT>()..])?;
                usize::from_be_bytes(buffer)
            }
        };

        Ok(self.read_fixed_size_string(size)?)
    }

    /// Read a size prefixed string into a variable
    ///
    /// # Parameters
    ///     `endianness` - Endianness of the stream to read from
    ///     `target` - The variable to store the value in
    ///
    /// # Errors
    ///     io::Error during read
    ///     io::Error::UnexpectedEof when there isn't enough data in the stream
    fn read_into_size_prefixed_string<PrefixT: PrimInt>(
        &mut self,
        target: &mut String,
        endianness: Endianness,
    ) -> Result<(), Error> {
        *target = self.read_size_prefixed_string::<PrefixT>(endianness)?;

        Ok(())
    }
}
