use crate::client::{to_c_string, AeronClient, AeronConfig, AeronError, AeronResult};
use libaeron_sys::{
    aeron_async_add_subscription, aeron_async_add_subscription_poll,
    aeron_async_add_subscription_t, aeron_fragment_assembler_create,
    aeron_fragment_assembler_delete, aeron_fragment_assembler_t, aeron_fragment_handler_t,
    aeron_header_t, aeron_subscription_close, aeron_subscription_poll, aeron_subscription_t,
};
use log::info;
use std::{
    ffi::{c_char, c_void},
    mem::transmute,
    ptr,
    time::{Duration, Instant},
};

#[derive(Debug)]
struct PollValue<'a> {
    value: Option<&'a [u8]>,
}

struct AeronReciever {
    _client: AeronClient,
    subscription: *mut aeron_subscription_t,
}

impl AeronReciever {
    pub fn new(config: &AeronConfig) -> AeronResult<Self> {
        let client = AeronClient::new()?;
        let mut async_subscription = ptr::null_mut::<aeron_async_add_subscription_t>();
        let mut subscription = ptr::null_mut::<aeron_subscription_t>();

        let uri_c_string = to_c_string(&config.uri)?;
        let uri_ptr: *const c_char = uri_c_string.as_ptr();

        info!(
            "[{}] {}",
            &config.uri,
            stringify!(aeron_async_add_subscription)
        );
        if unsafe {
            aeron_async_add_subscription(
                &mut async_subscription,
                client.aeron,
                uri_ptr,
                config.stream_id,
                None,
                ptr::null_mut(),
                None,
                ptr::null_mut(),
            )
        } < 0
        {
            return Err(AeronError::with_aeron_err(
                format!("{} failed", stringify!(aeron_async_add_subscription)).as_str(),
            ));
        }

        info!(
            "[{}] {}",
            &config.uri,
            stringify!(aeron_async_add_subscription_poll)
        );
        let timeout_duration = Duration::from_secs(30);
        let timeout = Instant::now() + timeout_duration;
        while subscription.is_null() {
            if unsafe { aeron_async_add_subscription_poll(&mut subscription, async_subscription) }
                < 0
            {
                return Err(AeronError::with_aeron_err(
                    format!("{} failed", stringify!(aeron_async_add_subscription_poll)).as_str(),
                ));
            }
            if Instant::now() > timeout {
                return Err(AeronError::new(format!(
                    "{} failed: timed out after '{:?}'",
                    stringify!(aeron_async_add_subscription_poll),
                    timeout_duration
                )));
            }
        }

        Ok(Self {
            _client: client,
            subscription,
        })
    }
}

impl Drop for AeronReciever {
    fn drop(&mut self) {
        info!("dropping {}...", stringify!(AeronReciever));
        if !self.subscription.is_null() {
            unsafe { aeron_subscription_close(self.subscription, None, ptr::null_mut()) };
        }
        info!("dropped {}", stringify!(AeronReciever));
    }
}

pub struct AeronPoller {
    receiver: AeronReciever,
    fragment_handler: aeron_fragment_handler_t,
    fragment_assembler: *mut aeron_fragment_assembler_t,
}

impl AeronPoller {
    pub fn new(config: &AeronConfig) -> AeronResult<Self> {
        let receiver = AeronReciever::new(config)?;
        let mut fragment_assembler = ptr::null_mut::<aeron_fragment_assembler_t>();

        extern "C" fn fragment_handler(
            clientd: *mut c_void,
            buffer: *const u8,
            length: usize,
            _header: *mut aeron_header_t,
        ) {
            let data = unsafe { &mut *(clientd as *mut PollValue) };
            // todo - do we need to copy the buffer? clone? std::ptr::copy_nonoverlapping?
            data.value = Some(unsafe { std::slice::from_raw_parts(buffer, length) });
        }

        if unsafe {
            aeron_fragment_assembler_create(
                &mut fragment_assembler,
                Some(fragment_handler),
                ptr::null_mut(),
            )
        } < 0
        {
            return Err(AeronError::with_aeron_err(
                "failed to create fragment assembler",
            ));
        }

        Ok(Self {
            receiver,
            fragment_handler: Some(fragment_handler),
            fragment_assembler,
        })
    }

    pub fn poll<'a>(&self) -> AeronResult<Option<&'a [u8]>> {
        let mut poll_value: PollValue = PollValue { value: None };
        let clientd = &mut poll_value as *mut PollValue;

        // take 1 fragment at a time, if that fragment completes the message then the callback will be called
        loop {
            let fragments_read = unsafe {
                aeron_subscription_poll(
                    self.receiver.subscription,
                    self.fragment_handler,
                    clientd as *mut c_void,
                    1,
                )
            };
            if fragments_read < 0 {
                return Err(AeronError::with_aeron_err(
                    format!("{} failed", stringify!(aeron_subscription_poll)).as_str(),
                ));
            }
            if fragments_read == 0 || poll_value.value.is_some() {
                break;
            }
        }
        Ok(poll_value.value)
    }
}

impl Drop for AeronPoller {
    fn drop(&mut self) {
        info!("dropping {}...", stringify!(AeronPoller));
        if !self.fragment_assembler.is_null() {
            unsafe { aeron_fragment_assembler_delete(self.fragment_assembler) };
        }
        info!("dropped {}", stringify!(AeronPoller));
    }
}

pub struct AeronSubscriber {
    receiver: AeronReciever,
    fragment_assembler: *mut aeron_fragment_assembler_t,
}

impl AeronSubscriber {
    pub fn new(config: &AeronConfig) -> AeronResult<Self> {
        let receiver = AeronReciever::new(config)?;
        let fragment_assembler = ptr::null_mut::<aeron_fragment_assembler_t>();

        Ok(Self {
            receiver,
            fragment_assembler,
        })
    }

    pub fn subscribe<TCallback, TIdleStrat, TIsSub>(
        &mut self,
        func: TCallback,
        idle_strategy: TIdleStrat,
        is_subscribed: TIsSub,
    ) -> AeronResult<()>
    where
        TCallback: FnMut(&[u8]),
        TIdleStrat: Fn(i32),
        TIsSub: Fn() -> bool,
    {
        info!("subscribing...");
        let mut fragment_assembler = ptr::null_mut::<aeron_fragment_assembler_t>();

        extern "C" fn fragment_handler(
            clientd: *mut c_void,
            buffer: *const u8,
            length: usize,
            _header: *mut aeron_header_t,
        ) {
            let data = unsafe { std::slice::from_raw_parts(buffer, length) };
            // todo - use transmute_copy(&clientd) instead of transmute(clientd)?
            let func: &mut Box<dyn FnMut(&[u8])> = unsafe { transmute(clientd) };
            func(data)
        }

        if unsafe {
            aeron_fragment_assembler_create(
                &mut fragment_assembler,
                Some(fragment_handler),
                ptr::null_mut(),
            )
        } < 0
        {
            return Err(AeronError::with_aeron_err(
                "failed to create fragment assembler",
            ));
        }

        // trait object with a stable address
        let func = Box::new(func) as Box<dyn FnMut(&[u8])>;
        // thin pointer
        let func = Box::new(func);
        // raw pointer
        let clientd = Box::into_raw(func);

        loop {
            let fragments_read = unsafe {
                aeron_subscription_poll(
                    self.receiver.subscription,
                    Some(fragment_handler),
                    clientd as *mut c_void,
                    10,
                )
            };

            idle_strategy(fragments_read);
            if !is_subscribed() {
                info!("unsubscribing...");
                break;
            }
        }

        Ok(())
    }
}

impl Drop for AeronSubscriber {
    fn drop(&mut self) {
        info!("dropping {}...", stringify!(AeronSubscriber));
        if !self.fragment_assembler.is_null() {
            unsafe { aeron_fragment_assembler_delete(self.fragment_assembler) };
        }
        info!("dropped {}", stringify!(AeronSubscriber));
    }
}
