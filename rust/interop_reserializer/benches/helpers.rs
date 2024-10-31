use generated_mod::types_tests::BookUpdate;
use serde::{ Deserialize, Serialize };

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct BookUpdateSerializable {
    pub time: i64,
    pub timestamp_exch: Option<i64>,
    pub inst_id: i64,
    pub update_type: i64,
    pub bids: Vec<i64>,
    pub asks: Vec<i64>,
}

pub fn get_book_update_test_obj() -> BookUpdate {
    BookUpdate {
        time: 123,
        timestamp_exch: Some(456),
        inst_id: 789,
        update_type: 101112,
        bids: vec![1, 2, 3, 4, 5],
        asks: vec![9, 8, 7, 6, 5],
    }
}

pub fn get_book_update_serializable_test_obj() -> BookUpdateSerializable {
    BookUpdateSerializable {
        time: 123,
        timestamp_exch: Some(456),
        inst_id: 789,
        update_type: 101112,
        bids: vec![1, 2, 3, 4, 5],
        asks: vec![9, 8, 7, 6, 5],
    }
}

pub fn get_default_buffer() -> Vec<u8> {
    Vec::with_capacity(200)
}
