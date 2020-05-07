//! Generic I2C interface
//!
//! # Example
//!
//! ```
//! use generic_array::{GenericArray, arr, typenum::consts::U2};
//! use i2c_interface::I2cInterface;
//! use embedded_hal::blocking::i2c;
//!
//! # struct MockRegister {
//! #     address: u8,
//! #     value: [u8; 2],
//! # }
//! #
//! # struct MockI2C {
//! #     register: MockRegister,
//! # }
//! #
//! # impl MockI2C {
//! #     fn new() -> Self {
//! #         MockI2C {
//! #             register: MockRegister {
//! #                 address: 0,
//! #                 value: [0; 2],
//! #             },
//! #         }
//! #     }
//! # }
//! #
//! # impl i2c::WriteRead for MockI2C {
//! #     type Error = ();
//! #
//! #     fn write_read(
//! #         &mut self,
//! #         address: u8,
//! #         bytes: &[u8],
//! #         buffer: &mut [u8],
//! #     ) -> Result<(), Self::Error> {
//! #         if self.register.address == bytes[0] {
//! #             for (i, item) in self.register.value.iter().enumerate() {
//! #                 buffer[i] = *item;
//! #             }
//! #         }
//! #         Ok(())
//! #     }
//! # }
//! #
//! # impl i2c::Write for MockI2C {
//! #     type Error = ();
//! #
//! #     fn write(&mut self, addr: u8, bytes: &[u8]) -> Result<(), Self::Error> {
//! #         self.register.address = bytes[0];
//! #         for (i, item) in self.register.value.iter_mut().enumerate() {
//! #             *item = bytes[i + 1];
//! #         }
//! #         Ok(())
//! #     }
//! # }
//! #
//! let device_address = 0x01;
//! let register = 0x02;
//! let value = arr![u8; 0x03, 0x04];
//!
//! # let mock_i2c = MockI2C::new();
//! # let mut i2c_interface = I2cInterface {
//! #     address: device_address,
//! #     i2c: mock_i2c,
//! # };
//! i2c_interface.write_register(register, value).unwrap();
//! let reading: GenericArray<u8, U2> = i2c_interface.read_register(register).unwrap();
//!
//! assert_eq!(reading, value);
//! ```

#![no_std]

use core::{mem, ops::Add};
use embedded_hal as hal;
use generic_array::{typenum::bit::B1, typenum::operator_aliases::Add1, ArrayLength, GenericArray};

pub use crate::hal::blocking::i2c;
pub use generic_array;

#[derive(Debug)]
/// Describes I2C interface
pub struct I2cInterface<I2C> {
    /// Slave device I2C
    pub i2c: I2C,

    /// Slave device address
    pub address: u8,
}

impl<I2C> I2cInterface<I2C> {
    /// Read bytes from register
    pub fn read_register<N, Err>(
        &mut self,
        register: impl Into<u8>,
    ) -> Result<GenericArray<u8, N>, Err>
    where
        I2C: i2c::WriteRead<Error = Err>,
        N: ArrayLength<u8>,
    {
        let mut buff: GenericArray<u8, N> = unsafe { mem::zeroed() };
        self.i2c
            .write_read(self.address, &[register.into()], &mut buff)?;
        Ok(buff)
    }

    /// Write bytes to register
    pub fn write_register<N, Err>(
        &mut self,
        register: impl Into<u8>,
        bytes: GenericArray<u8, N>,
    ) -> Result<(), Err>
    where
        I2C: i2c::Write<Error = Err>,
        N: ArrayLength<u8> + Add<B1>,
        Add1<N>: ArrayLength<u8>,
    {
        let mut payload: GenericArray<u8, Add1<N>> = unsafe { mem::zeroed() };
        payload[0] = register.into();
        for (i, item) in bytes.iter().enumerate() {
            payload[i + 1] = *item;
        }
        self.i2c.write(self.address, &payload)
    }
}
