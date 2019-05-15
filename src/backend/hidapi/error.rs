use failure;
use cfg_if::cfg_if;
use crate::error;

/// TODO: Work on a better error handling system. Get rid of enum maybe? See crate::error.rs -Alex
cfg_if!{
    if #[cfg(feature = "linux-rust-hidraw")]{
        pub type HidResult<T> = Result<T, HidError>;

        pub enum HidapiErrorEnum {
            CommonErrors(ApiErrorEnum),
            #[fail(display = "Udev error: {}", udev_e)]
            UdevError { udev_e: libudev::Error },

            #[fail(display = "Nix error: {}", nix_e)]
            NixError { nix_e: nix::Error },
        }

        pub trait ResultExt<T> {
            /// Convert any Result<T, E> into Result<T, HidError {E}>
            fn convert(self) -> Result<T, HidError>;
        }

        cfg_if!{
            if #[cfg(feature = "linux-rust-hidraw")] {
                impl<T> ResultExt<T> for Result<T, libudev::Error> {
                    fn convert(self) -> Result<T, HidError> {
                        self.map_err(|udev_e| HidError::UdevError { udev_e })
                    }
                }
                impl<T> ResultExt<T> for Result<T, nix::Error> {
                    fn convert(self) -> Result<T, HidError> {
                        self.map_err(|nix_e| HidError::NixError { nix_e })
                    }
                }
            }
        }
    }
}