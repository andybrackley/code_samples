use core::fmt;
use libaeron_sys::{
    aeron_close, aeron_context_close, aeron_context_init, aeron_context_t, aeron_errmsg,
    aeron_init, aeron_start, aeron_t,
};
use log::info;
use std::{error::Error, ffi::CStr, ptr};

#[derive(Debug)]
pub struct AeronError(String);

impl AeronError {
    pub fn new(err_msg: String) -> Self {
        Self(err_msg)
    }

    pub fn with_aeron_err(err_msg: &str) -> Self {
        let aeron_err_msg = unsafe { CStr::from_ptr(aeron_errmsg()) }
            .to_str()
            .unwrap_or_else(|_| "failed to get Aeron err msg");
        Self(format!("{}: {}", err_msg, aeron_err_msg))
    }
}

impl fmt::Display for AeronError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Aeron error: {}", self.0)
    }
}

impl Error for AeronError {}

pub type AeronResult<T> = Result<T, AeronError>;

#[derive(Debug)]
pub struct AeronConfig {
    pub uri: String,
    pub stream_id: i32,
}

pub struct AeronClient {
    aeron_context: *mut aeron_context_t,
    pub aeron: *mut aeron_t,
}

impl AeronClient {
    pub fn new() -> AeronResult<Self> {
        let mut aeron_context = ptr::null_mut::<aeron_context_t>();
        let mut aeron = ptr::null_mut::<aeron_t>();
        info!("{}", stringify!(aeron_context_init));
        if unsafe { aeron_context_init(&mut aeron_context) } < 0 {
            return Err(AeronError::with_aeron_err(
                format!("{} failed", stringify!(aeron_context_init)).as_str(),
            ));
        }
        info!("{}", stringify!(aeron_init));
        if unsafe { aeron_init(&mut aeron, aeron_context) } < 0 {
            return Err(AeronError::with_aeron_err(
                format!("{} failed", stringify!(aeron_init)).as_str(),
            ));
        }
        info!("{}", stringify!(aeron_start));
        if unsafe { aeron_start(aeron) } < 0 {
            return Err(AeronError::with_aeron_err(
                format!("{} failed", stringify!(aeron_start)).as_str(),
            ));
        }
        Ok(Self {
            aeron_context,
            aeron,
        })
    }
}

impl Drop for AeronClient {
    fn drop(&mut self) {
        info!("dropping {}...", stringify!(AeronClient));
        if !self.aeron.is_null() {
            unsafe { aeron_close(self.aeron) };
        }
        if !self.aeron_context.is_null() {
            unsafe { aeron_context_close(self.aeron_context) };
        }
        info!("dropped {}", stringify!(AeronClient));
    }
}

pub fn to_c_string(s: &str) -> AeronResult<std::ffi::CString> {
    std::ffi::CString::new(s).map_err(|err| {
        AeronError::new(format!(
            "failed to create CString from string '{}'. Error: '{}'",
            s, err
        ))
    })
}
