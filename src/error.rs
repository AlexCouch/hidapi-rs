// **************************************************************************
// Copyright (c) 2018 Roland Ruckerbauer All Rights Reserved.
//
// This file is part of hidapi-rs, based on hidapi-rs by Osspial
// **************************************************************************
//
// This file should be considered as part of a common module where the different backends implement the common module.
// This error module provides a common interface for setting and getting errors. However, setting errors should be called externally (via hidapi-rs child modules (aka children))
// and getting an error should be called internally. This abstract module should only provide an interface for children to easily
// invoke a panic with the error type and error message without having to define their own error messages and formatting. This error module should format
// all the error messages for the children in a way that provides all the necessary information needed to debug the use of hidapi-rs.
// This is why the error module should be abstracted for specific backends to implement their own error handling since different backends rely on different
// HID api's (mac -> IOHidManager; Windows -> setupapi -> linux -> hidraw) which provide different types and handle devices differently.
// - Alex Couch, 2019, May 14

use cfg_if::cfg_if;
use failure::{Compat, Error};

/// TODO: Abstract the ApiResult so that backends can have their own dedicated result type.
pub type ApiResult<T> = Result<T, ErrorEnum>;

/// TODO: Work on a better error handling system. Get rid of enum maybe?<br>
/// TODO: Abstract the error handling system into a subapi.<br>
/// TODO: Work on create a common module that different platforms can inplement for different backends. Including the error system.<br>
///     Panics should be internally called when someone calls ApiError::set_error.<br>
///     If someone calls set_error, the unwrapping should be handled internally so that the caller just needs to call a single function.<br>
/// -Alex, 2019, May 14
#[derive(Debug, Fail)]
pub enum ErrorEnum {
    #[fail(display = "hidapi error: {}", message)]
    HidApiError { message: String },

    #[fail(
        display = "hidapi error: (could not get error message), caused by: {}",
        cause
    )]
    HidApiErrorEmptyWithCause {
        #[cause]
        cause: Compat<Error>,
    },

    #[fail(display = "hidapi error: (could not get error message)")]
    HidApiErrorEmpty,

    #[cfg(any(
        feature = "linux-static-hidraw",
        feature = "linux-static-libusb",
        feature = "linux-shared-hidraw",
        feature = "linux-shared-libusb"
    ))]
    #[fail(display = "failed converting {:#X} to rust char", wide_char)]
    FromWideCharError { wide_char: wchar_t },

    #[fail(display = "Failed to initialize hidapi (maybe initialized before?)")]
    InitializationError,

    #[fail(display = "Failed opening hid device")]
    OpenHidDeviceError,

    #[fail(display = "Invalid data: size can not be 0")]
    InvalidZeroSizeData,

    #[fail(
        display = "Failed to send all data: only sent {} out of {} bytes",
        sent, all
    )]
    IncompleteSendError { sent: usize, all: usize },

    #[fail(display = "Can not set blocking mode to '{}'", mode)]
    SetBlockingModeError { mode: &'static str },
}

pub trait ApiError{
    /// Set an error to be unwrapped in an internal panic!.
    /// 
    /// @param error The error type to throw
    fn set_error(error: ErrorEnum);

    /// Get the error that was set by something and then unwrap in an internal panic!.
    /// 
    /// @return An option of whether an error was set or not. Some(error) or None.
    fn get_error() -> Option<ErrorEnum>;
}