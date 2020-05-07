use embedded_hal::blocking::i2c;
use generic_array::{ArrayLength, GenericArray};
use heapless::{consts::U64, Vec};

pub(crate) struct MockI2C<Bufferlength>
where
    Bufferlength: ArrayLength<u8>,
{
    buffer: GenericArray<u8, Bufferlength>,
    pub(crate) address_written: Option<u8>,
    pub(crate) bytes_written: Vec<u8, U64>,
}

impl<Bufferlength> MockI2C<Bufferlength>
where
    Bufferlength: ArrayLength<u8>,
{
    pub(crate) fn new(buffer: GenericArray<u8, Bufferlength>) -> Self {
        MockI2C {
            buffer,
            address_written: None,
            bytes_written: Vec::new(),
        }
    }
}

impl<Bufferlength> i2c::WriteRead for MockI2C<Bufferlength>
where
    Bufferlength: ArrayLength<u8>,
{
    type Error = ();
    fn write_read(
        &mut self,
        address: u8,
        bytes: &[u8],
        buffer: &mut [u8],
    ) -> Result<(), Self::Error> {
        self.address_written = Some(address);
        self.bytes_written.extend_from_slice(bytes).unwrap();
        self.buffer.swap_with_slice(buffer);
        Ok(())
    }
}

impl<Bufferlength> i2c::Write for MockI2C<Bufferlength>
where
    Bufferlength: ArrayLength<u8>,
{
    type Error = ();

    fn write(&mut self, addr: u8, bytes: &[u8]) -> Result<(), Self::Error> {
        self.address_written = Some(addr);
        self.bytes_written.extend_from_slice(bytes).unwrap();
        Ok(())
    }
}
