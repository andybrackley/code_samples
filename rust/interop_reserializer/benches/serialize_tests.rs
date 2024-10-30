#[cfg(test)]
pub mod generated_serializer_tests {
    use generated_mod::types_tests::BookUpdate;

    #[test]
    pub fn t() {
        let to_serialize = BookUpdate {
            time: 32,
            timestamp_exch: None,
            inst_id: 64,
            update_type: 128,
            bids: Vec::new(),
            asks: Vec::new(),
        };

        let mut buffer: Vec<u8> = Vec::new();
        buffer.reserve(1024);

        let new_pos = to_serialize.serialize_into(&mut buffer, 0);
        println!("Written: '{new_pos}' bytes");

        let from_buffer = BookUpdate::deserialize_from(&buffer, 0);
        match from_buffer {
            Ok((obj, pos)) => {
                println!("Read: '{}' bytes. Obj=[{:#?}]", pos, obj);
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}
