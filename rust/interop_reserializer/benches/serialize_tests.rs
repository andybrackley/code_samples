#[cfg(test)]
pub mod generated_serializer_tests {
    const OUTPUT_DIR: &str = ".\\serialized\\";

    use std::fs::File;
    use std::io::{ self, Write };
    use generated_mod::common_deserialize::{ deserialize_scalar, deserialize_vec };
    use generated_mod::common_serialize::{ serialize_scalar, serialize_vec };
    use generated_mod::common_types::RawArray;
    use generated_mod::types_tests::BookUpdate;

    fn write_buffer_to_file(filepath: &str, buffer: &Vec<u8>) -> io::Result<()> {
        let mut file = File::create(filepath)?;
        file.write_all(buffer)?;
        Ok(())
    }

    #[test]
    pub fn test_raw_array() {
        let arr = RawArray::from_vec(vec![1, 2, 3, 4]);
        let as_vec = arr.to_vec();
        dbg!(as_vec);
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
            bids: vec![1, 2, 3, 4, 5, 6],
            asks: vec![9, 8, 7, 6, 5],
        };

        let mut buffer: Vec<u8> = Vec::with_capacity(200);
        let new_pos = to_serialize.serialize_into(&mut buffer, 0);
        unsafe {
            buffer.set_len(new_pos);
        }

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

    fn get_book_update_test_obj() -> BookUpdate {
        BookUpdate {
            time: 123,
            timestamp_exch: Some(456),
            inst_id: 789,
            update_type: 101112,
            bids: vec![1, 2, 3, 4, 5],
            asks: vec![9, 8, 7, 6, 5],
        }
    }

    #[test]
    pub fn test_bench() {
        let mut buf: Vec<u8> = Vec::with_capacity(200);
        let obj = get_book_update_test_obj();
        let pos = obj.serialize_into(&mut buf, 0);
        unsafe {
            buf.set_len(pos);
        }

        let mut pos = 0;
        let _ = BookUpdate::deserialize_from(&buf, pos).unwrap();

        println!("Test Completed");
    }

    #[test]
    pub fn test_alignments() {
        #[derive(Debug, PartialEq, Eq)]
        struct S {
            a: i32,
            b: i32,
            c: i64,
            d: i128,
            v: Vec<i32>,
        }

        let s1 = S {
            a: 1,
            b: 2,
            c: 3,
            d: 4,
            v: vec![9, 8, 7, 6, 5],
        };

        let mut buf: Vec<u8> = Vec::with_capacity(200);
        let _ = unsafe { buf.align_to::<i8>() };

        let mut pos = serialize_scalar(&s1.a, &mut buf, 0);
        pos = serialize_scalar(&s1.b, &mut buf, pos);
        pos = serialize_scalar(&s1.c, &mut buf, pos);
        pos = serialize_scalar(&s1.d, &mut buf, pos);
        pos = serialize_vec(&s1.v, &mut buf, pos);

        let mut offset = 0;
        let s2 = S {
            a: deserialize_scalar::<i32>(&buf, &mut offset),
            b: deserialize_scalar::<i32>(&buf, &mut offset),
            c: deserialize_scalar::<i64>(&buf, &mut offset),
            d: deserialize_scalar::<i128>(&buf, &mut offset),
            v: deserialize_vec(&buf, &mut offset).to_vec(),
        };

        assert_eq!(s1, s2);
    }
}
