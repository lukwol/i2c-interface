mod mock;

#[cfg(test)]
mod i2c_interface {
    use super::mock::MockI2C;
    use generic_array::{
        arr,
        typenum::consts::{U2, U64},
        GenericArray,
    };
    use heapless::Vec;
    use i2c_interface::I2cInterface;

    #[test]
    fn reading_register() {
        let device_address = 2;
        let register_address = 3;
        let payload = arr![u8; 42, 43];

        let mut interface = I2cInterface {
            address: device_address,
            i2c: MockI2C::new(payload),
        };
        let read: GenericArray<u8, U2> = interface.read_register(register_address).unwrap();
        let mock_i2c = interface.i2c;

        assert_eq!(read, payload);
        assert_eq!(mock_i2c.address_written, Some(device_address));
        assert_eq!(
            mock_i2c.bytes_written,
            Vec::<u8, U64>::from_slice(&[register_address]).unwrap()
        );
    }
}
