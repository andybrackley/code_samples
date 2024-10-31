const OUTPUT_DIR: &str = ".\\serialized\\";

#[cfg(test)]
pub mod generated_serializer_tests {
    use std::fs::File;
    use std::io::{ self, Write };

    use generated_mod::types_tests::BookUpdate;

    use crate::OUTPUT_DIR;

    fn write_buffer_to_file(filepath: &str, buffer: &Vec<u8>) -> io::Result<()> {
        let mut file = File::create(filepath)?;
        file.write_all(buffer)?;
        Ok(())
    }

    #[test]
    pub fn bookupdate_serialization_test() {
        let filename = "BookUpdate_rust.bin";
        let filepath = format!("{}{}", OUTPUT_DIR, filename);

        let to_serialize = BookUpdate {
            time: 32,
            timestamp_exch: Some(999),
            inst_id: 64,
            update_type: 128,
            bids: vec![1, 2, 3, 4, 5],
            asks: vec![9, 8, 7, 6, 5],
        };

        let mut buffer: Vec<u8> = Vec::with_capacity(200);

        let new_pos = to_serialize.serialize_into(&mut buffer, 0);
        println!("Written: '{new_pos}' bytes");
        write_buffer_to_file(&filepath, &buffer).unwrap();

        let from_buffer = BookUpdate::deserialize_from(&buffer, 0);
        match &from_buffer {
            Ok((obj, pos)) => {
                println!("Read: '{}' bytes. Obj=[{:#?}]", pos, obj);
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }

        assert_eq!(to_serialize, from_buffer.unwrap().0);
    }
}