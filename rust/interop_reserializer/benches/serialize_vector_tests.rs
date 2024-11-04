use std::borrow::BorrowMut;

use generated_mod::common_serialize::serialize_vec;

#[cfg(test)]
pub mod serialize_vector_tests {
    use generated_mod::poc_types::{ BookUpdatePoc, BookUpdatePocCreate, BookUpdatePocRead };

    #[test] 
    pub fn test_bu() {
        let bu = BookUpdatePocCreate {
            bids: vec![1, 2, 3, 4],
            asks: vec![9, 8, 7, 6],
        };

        let mut buf: Vec<u8> = Vec::with_capacity(100);
        let pos = bu.write_to_buffer(&mut buf, 0);
        unsafe {
            buf.set_len(pos);
        }

        let buc = BookUpdatePocRead::from_buffer(&buf, 0);

        dbg!(buc.bids());
        dbg!(buc.asks());
    }

    /*

    #[test]
    pub fn test() {
        let vec: Vec<i64> = vec![1, 2, 3, 4];
        let mut buf: Vec<u8> = Vec::with_capacity(100);
        unsafe {
            buf.set_len(32);
        }

        let mut pos = 0;
        serialize(vec, &mut buf, pos);

        let x = get_bytes_as(&buf, 16);
        dbg!(x);
    }

    #[test]
    pub fn fn_test_raw() {
        let mut buf = Vec::with_capacity(500);
        let v: Vec<i64> = vec![1, 2, 3, 4];

        let mut pos = serialize_vec2(&v, &mut buf, 0);

        pos = 0;
        let v = deserialize_vec4::<i64>(&buf, &mut pos);
        // let v = deserialize_vec2(&buf, &mut pos);

        println!("VecLen: {}", v.len());
        // dbg!(v);
    }

    #[test]
    pub fn test_vec() {
        let mut buf = Vec::with_capacity(500);
        let v: Vec<i64> = vec![1, 2, 3, 4];

        let mut pos = serialize_vec2(&v, &mut buf, 0);
        println!("Pos: {}", pos);

        let v2: Vec<i64> = vec![5, 6, 7, 8];
        pos = serialize_vec(&v2, &mut buf, pos);

        let mut pos = 0;
        let res1: Vec<i64> = deserialize_vec2(&buf, &mut pos);

        dbg!(res1);

        let res2: Vec<i64> = deserialize_vec2(&buf, &mut pos);

        dbg!(res2);
    }

*/
}
