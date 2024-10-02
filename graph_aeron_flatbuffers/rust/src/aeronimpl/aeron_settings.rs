use std::ffi::CString;


extern crate aeron;

#[derive(Clone)]
pub struct Settings {
    dir_prefix: String,
    channel: String,
    stream_id: i32,
}

impl Settings {
    pub fn new(dir_prefix: String) -> Self {
        Self {
            dir_prefix: dir_prefix,
            // channel: CString::new("aeron:ipc").expect("Invalid string for channel"),
            channel: String::from("aeron:ipc"),
            stream_id: 1,
        }
    }
}

// See:
//    https://github.com/UnitedTraders/aeron-rs/blob/master/examples/basic_publisher.rs
//    https://github.com/UnitedTraders/aeron-rs/blob/master/examples/basic_subscriber.rs

pub fn dump_connection_status() {

}

fn str_to_c(val: &str) -> CString {
    CString::new(val).expect("Error converting str to CString")
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

        println!("Using CnC file: {}", context.cnc_file_name());
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
    use super::{str_to_c, Settings};

    pub fn get_by_id(connection: &mut aeron::aeron::Aeron, pub_id: i64) -> Arc<Mutex<aeron::publication::Publication>> {
        let mut publication = connection.find_publication(pub_id);
        while publication.is_err() {
            std::thread::yield_now();
            publication = connection.find_publication(pub_id);
        }

        return publication.unwrap();
    }

    pub fn create(connection: &mut aeron::aeron::Aeron, settings: &Settings) -> Arc<Mutex<aeron::publication::Publication>> {
        let pub_id = connection.add_publication(str_to_c(&settings.channel), settings.stream_id).expect("Error adding publication");
        let publisher = get_by_id(connection, pub_id);
        return publisher;
    }
}

pub mod subscriber {
    use std::sync::{atomic::{AtomicBool, Ordering}, Arc, Mutex};
    use aeron::{concurrent::{atomic_buffer::AtomicBuffer, logbuffer::header::Header, strategies::SleepingIdleStrategy, strategies::Strategy}, utils::types::Index};

    use super::{str_to_c, Settings};

    pub fn get_by_id(connection: &mut aeron::aeron::Aeron, sub_id: i64) -> Arc<Mutex<aeron::subscription::Subscription>> {
        let mut subscription = connection.find_subscription(sub_id);
        while(subscription.is_err()) {
            std::thread::yield_now();
            subscription = connection.find_subscription(sub_id);
        }

        return subscription.unwrap();
    }

    pub fn create(connection: &mut aeron::aeron::Aeron, settings: &Settings) -> Arc<Mutex<aeron::subscription::Subscription>> {
        let sub_id = connection.add_subscription(str_to_c(&settings.channel), settings.stream_id).expect("Failed to add subscription");
        let subscription = get_by_id(connection, sub_id);
        return subscription;
    }

    pub fn handle(subscription: &Arc<Mutex<aeron::subscription::Subscription>>, handler: &mut impl FnMut(&AtomicBuffer, Index, Index, &Header), is_running: Arc<AtomicBool>) {
        let fragment_limit = 10;
        let idle_strategy = SleepingIdleStrategy::new(1000);

        while is_running.load(Ordering::Relaxed) {
            let fragments_read = subscription.lock().unwrap().poll(handler, fragment_limit);
            idle_strategy.idle_opt(fragments_read);
        }
    }
}