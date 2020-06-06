#[derive(Debug)]
pub enum Error {
    DBus(dbus::Error),
    UnsupportedMethod,
    UnsupportedDevice,
}

impl From<dbus::Error> for Error {
    fn from(error: dbus::Error) -> Self {
        Error::DBus(error)
    }
}
