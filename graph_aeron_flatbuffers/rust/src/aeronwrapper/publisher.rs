use crate::client::{to_c_string, AeronClient, AeronConfig, AeronError, AeronResult};
use libaeron_sys::{
    aeron_async_add_publication, aeron_async_add_publication_poll, aeron_async_add_publication_t,
    aeron_publication_close, aeron_publication_offer, aeron_publication_t,
    AERON_PUBLICATION_ADMIN_ACTION, AERON_PUBLICATION_BACK_PRESSURED, AERON_PUBLICATION_CLOSED,
    AERON_PUBLICATION_ERROR, AERON_PUBLICATION_MAX_POSITION_EXCEEDED,
    AERON_PUBLICATION_NOT_CONNECTED,
};
use log::info;
use std::{
    borrow::Cow,
    ffi::c_char,
    ptr,
    time::{Duration, Instant},
};

pub struct AeronPublisher {
    _client: AeronClient,
    publication: *mut aeron_publication_t,
}

impl AeronPublisher {
    pub fn new(config: &AeronConfig) -> AeronResult<Self> {
        let client = AeronClient::new()?;
        let mut async_publication = ptr::null_mut::<aeron_async_add_publication_t>();
        let mut publication = ptr::null_mut::<aeron_publication_t>();

        let uri_c_string = to_c_string(&config.uri)?;
        let uri_ptr: *const c_char = uri_c_string.as_ptr();

        info!(
            "[{}] {}",
            config.uri,
            stringify!(aeron_async_add_publication)
        );
        if unsafe {
            aeron_async_add_publication(
                &mut async_publication,
                client.aeron,
                uri_ptr,
                config.stream_id,
            )
        } < 0
        {
            return Err(AeronError::with_aeron_err(
                format!("{} failed", stringify!(aeron_async_add_publication)).as_str(),
            ));
        }

        info!(
            "[{}] {}",
            &config.uri,
            stringify!(aeron_async_add_publication_poll)
        );
        let timeout_duration = Duration::from_secs(30);
        let timeout = Instant::now() + timeout_duration;
        while publication.is_null() {
            if unsafe { aeron_async_add_publication_poll(&mut publication, async_publication) } < 0
            {
                return Err(AeronError::with_aeron_err(
                    format!("{} failed", stringify!(aeron_async_add_publication_poll)).as_str(),
                ));
            }
            if Instant::now() > timeout {
                return Err(AeronError::new(format!(
                    "{} failed: timed out after '{:?}'",
                    stringify!(aeron_async_add_publication_poll),
                    timeout_duration
                )));
            }
        }

        Ok(Self {
            _client: client,
            publication,
        })
    }

    pub fn publish(&mut self, message: Cow<[u8]>) -> AeronResult<()> {
        let message_ptr = message.as_ptr();
        let message_length = message.len();

        // todo - try claim https://aeroncookbook.com/cookbook-content/aeron-try-claim/ to reduce a copy
        let result = unsafe {
            aeron_publication_offer(
                self.publication,
                message_ptr,
                message_length,
                None,
                ptr::null_mut(),
            )
        };

        if result < 0 {
            let err_code: i32 = result.try_into().map_err(|err| {
                AeronError::new(format!(
                    "failed to convert publication error code '{}' to i32",
                    err
                ))
            })?;
            let error = match err_code {
                AERON_PUBLICATION_NOT_CONNECTED => "publication not connected",
                AERON_PUBLICATION_BACK_PRESSURED => "publication back pressured",
                AERON_PUBLICATION_ADMIN_ACTION => "publication admin action",
                AERON_PUBLICATION_CLOSED => "publication closed",
                AERON_PUBLICATION_MAX_POSITION_EXCEEDED => "publication max position exceeded",
                AERON_PUBLICATION_ERROR => "publication error",
                _ => "unknown publication error",
            };
            return Err(AeronError::new(format!(
                "publishing error: err='{}', err_code='{}'",
                error, err_code
            )));
        }
        Ok(())
    }
}

impl Drop for AeronPublisher {
    fn drop(&mut self) {
        info!("dropping {}...", stringify!(AeronPublisher));
        if !self.publication.is_null() {
            unsafe { aeron_publication_close(self.publication, None, ptr::null_mut()) };
        }
        info!("dropped {}", stringify!(AeronPublisher));
    }
}
