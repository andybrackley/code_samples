use std::ffi::CString;


extern crate aeron;

pub struct Settings {
    dir_prefix: String,
    channel: CString,
    stream_id: i32
}

impl Settings {
    pub fn new() -> Self {
        Self {
            dir_prefix: String::new(),
            channel: CString::new("aeron::ipc").expect("Invalid string for channel"),
            stream_id: 1
        }
    }
}

// See:
//    https://github.com/UnitedTraders/aeron-rs/blob/master/examples/basic_publisher.rs
//    https://github.com/UnitedTraders/aeron-rs/blob/master/examples/basic_subscriber.rs

pub fn dump_connection_status() {

}

pub mod connection {
    use aeron;
    use aeron::context::Context;
    use aeron::utils::errors::AeronError;
    use super::Settings;

    pub fn create_context(settings: &Settings) -> Context {
        let mut context = Context::new();
        if !settings.dir_prefix.is_empty() {
            context.set_aeron_dir(settings.dir_prefix.clone());
        }

        return context;
    }

    pub fn connect(settings: &Settings) -> Result<aeron::aeron::Aeron, AeronError> {
        let context = create_context(settings);
        let connection = aeron::aeron::Aeron::new(context);
        return connection;
    }
}

pub mod publisher {
    use std::sync::{Arc, Mutex};
    use super::Settings;

    pub fn get_by_id(connection: &mut aeron::aeron::Aeron, pub_id: i64) -> Arc<Mutex<aeron::publication::Publication>> {
        let mut publication = connection.find_publication(pub_id);
        while(publication.is_err()) {
            std::thread::yield_now();
            publication = connection.find_publication(pub_id);
        }

        return publication.unwrap();
    }

    pub fn create(connection: &mut aeron::aeron::Aeron, settings: &Settings) -> Arc<Mutex<aeron::publication::Publication>> {
        let pub_id = connection.add_publication(settings.channel.clone(), settings.stream_id).expect("Error adding publication");
        let publisher = get_by_id(connection, pub_id);
        return publisher;
    }
}

pub mod subscriber {
    use std::sync::{Arc, Mutex};
    use aeron::{concurrent::{atomic_buffer::AtomicBuffer, logbuffer::header::Header, strategies::SleepingIdleStrategy, strategies::Strategy}, utils::types::Index};

    use super::Settings;

    pub fn get_by_id(connection: &mut aeron::aeron::Aeron, sub_id: i64) -> Arc<Mutex<aeron::subscription::Subscription>> {
        let mut subscription = connection.find_subscription(sub_id);
        while(subscription.is_err()) {
            std::thread::yield_now();
            subscription = connection.find_subscription(sub_id);
        }

        return subscription.unwrap();
    }

    pub fn create(connection: &mut aeron::aeron::Aeron, settings: &Settings) -> Arc<Mutex<aeron::subscription::Subscription>> {
        let sub_id = connection.add_subscription(settings.channel.clone(), settings.stream_id).expect("Failed to add subscription");
        let subscription = get_by_id(connection, sub_id);
        return subscription;
    }

    pub fn handle(subscription: &Arc<Mutex<aeron::subscription::Subscription>>, handler: &mut impl FnMut(&AtomicBuffer, Index, Index, &Header)) {
        let fragment_limit = 10;
        let idle_strategy = SleepingIdleStrategy::new(1000);

        while(true) {
            let fragments_read = subscription.lock().unwrap().poll(handler, fragment_limit);
            idle_strategy.idle_opt(fragments_read);
        }
    }
}