use crate::adapters::usbs::usb_errors::UsbError;

pub trait UsbConnection {

    fn open(&mut self) -> Result<(), UsbError>;

    fn close(&mut self);

    fn write(
        &mut self,
        endpoint: u8,
        data: &[u8],
    ) -> Result<usize, UsbError>;

    fn read(
        &mut self,
        endpoint: u8,
        data: &mut [u8],
    ) -> Result<usize, UsbError>;
}


#[cfg(test)]
mod tests {

    use super::*;

    struct FakeUsbConnection {
        opened: bool,
    }

    impl FakeUsbConnection {

        fn new() -> Self {
            Self {
                opened: false,
            }
        }
    }

    impl UsbConnection for FakeUsbConnection {

        fn open(&mut self) -> Result<(), UsbError> {
            self.opened = true;
            Ok(())
        }

        fn close(&mut self) {
            self.opened = false;
        }

        fn write(
            &mut self,
            _endpoint: u8,
            data: &[u8],
        ) -> Result<usize, UsbError> {

            Ok(data.len())
        }

        fn read(
            &mut self,
            _endpoint: u8,
            data: &mut [u8],
        ) -> Result<usize, UsbError> {

            for byte in data.iter_mut() {
                *byte = 0;
            }

            Ok(data.len())
        }
    }


    #[test]
    fn opens_connection() {

        let mut connection =
            FakeUsbConnection::new();

        let result =
            connection.open();

        assert!(result.is_ok());
        assert!(connection.opened);
    }


    #[test]
    fn closes_connection() {

        let mut connection =
            FakeUsbConnection::new();

        connection.open().unwrap();
        connection.close();

        assert!(!connection.opened);
    }


    #[test]
    fn writes_data() {

        let mut connection =
            FakeUsbConnection::new();

        connection.open().unwrap();

        let data = [1, 2, 3, 4];

        let result =
            connection.write(
                0x01,
                &data,
            );

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 4);
    }


    #[test]
    fn reads_data() {

        let mut connection =
            FakeUsbConnection::new();

        connection.open().unwrap();

        let mut data = [1, 2, 3, 4];

        let result =
            connection.read(
                0x81,
                &mut data,
            );

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 4);

        assert_eq!(
            data,
            [0, 0, 0, 0]
        );
    }
}