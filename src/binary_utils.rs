use std::io::{self, Error, ErrorKind, Seek};

pub trait BinaryUtils: Seek {
    fn ignore(&mut self, length: usize) -> io::Result<()>;
    fn next_alignment(&mut self, alignment: u64) -> io::Result<()>;
    fn tell(&mut self) -> io::Result<u64>;
    fn buffer_size(&mut self) -> io::Result<usize>;
}

impl<StreamT: Seek> BinaryUtils for StreamT {
    /// Skips `length` bytes in the stream.
    ///
    /// # Errors
    ///     io::Error::OutOfMemory   - Ignoring results in overflow of stream pos
    ///     io::Error::UnexpectedEof - Requested amount of bytes could not be skipped fully
    ///
    /// # Notes
    ///     When ErrorKind::FileTooLarge is in stable release it will
    ///     be used instead of io::Error::OutOfMemory.
    fn ignore(&mut self, length: usize) -> io::Result<()> {
        if length == 0 {
            return Ok(());
        }

        let calculated_offset = self
            .seek(std::io::SeekFrom::Current(0))?
            .checked_add(length as u64)
            .ok_or(Error::from(ErrorKind::OutOfMemory))?; // TODO: ErrorKind::FileTooLarge when stable - if ever

        if self.seek(std::io::SeekFrom::Current(length as i64))? != calculated_offset {
            return Err(Error::from(ErrorKind::UnexpectedEof));
        }

        Ok(())
    }

    /// Simple utility to wrap getting stream position
    ///
    /// # Errors
    ///     Same errors as std::io::Seek (negative offset, seeking failed)
    fn tell(&mut self) -> io::Result<u64> {
        Ok(self.seek(io::SeekFrom::Current(0))?)
    }

    /// For buffers with a fixed size (EG Byte Arrays/Files/ETC) return the
    /// size of the file. (This will not work for things like network streams)
    /// 
    /// This is equivalent to SeekFrom::End(0) then resetting the stream position
    ///
    /// # Errors
    ///     Same errors as std::io::Seek (negative offset, seeking failed)
    fn buffer_size(&mut self) -> io::Result<usize> {
        let current_position: u64 = self.tell()?;
        let stream_size: usize = self.seek(io::SeekFrom::End(0))? as usize;

        self.seek(io::SeekFrom::Start(current_position))?;

        Ok(stream_size)
    }

    /// Seeks the stream to the nearest `alignment` boundary.
    /// `alignment` must be non-zero
    ///
    /// # Errors
    ///     io::Error::OutOfMemory   - Next alignment results in overflow of stream pos
    ///     io::Error::InvalidInput  - Alignment passed was zero
    ///     io::Error::UnexpectedEof - Requested amount of bytes could not be skipped fully
    ///
    /// # Notes
    ///     When ErrorKind::FileTooLarge is in stable release it will
    ///     be used instead of io::Error::OutOfMemory.
    fn next_alignment(&mut self, alignment: u64) -> io::Result<()> {
        if alignment == 0 {
            return Err(Error::from(ErrorKind::InvalidInput));
        }

        let calculated_offset = self
            .seek(std::io::SeekFrom::Current(0))?
            .checked_next_multiple_of(alignment)
            .ok_or(Error::from(ErrorKind::OutOfMemory))?; // TODO: ErrorKind:FileTooLarge when stable - if ever

        if self.seek(std::io::SeekFrom::Start(calculated_offset))? != calculated_offset {
            return Err(Error::from(ErrorKind::UnexpectedEof));
        }

        Ok(())
    }
}
