#[cfg(test)]
pub mod serialize_vector_tests {
    use types_gen::generated::framework::common_deserialize::{deserialize_option, deserialize_scalar, deserialize_vec, Deserializable};
    use types_gen::generated::framework::common_serialize::{serialize_option, serialize_scalar, serialize_vec, Serializable};

    #[test]
    pub fn test_option() {
        #[derive(Debug, Copy, Clone)]
        struct S {
            a: i64,
            b: i32,
        }
        impl Serializable for S {
            fn serialize(&self, buffer: &mut [u8], pos: usize) -> usize {
                let mut pos = serialize_scalar(&self.a, buffer, pos);
                pos = serialize_scalar(&self.b, buffer, pos);
                pos
            }
        }

        impl<'a> Deserializable<'a> for S {
            fn deserialize(buffer: &'a [u8], pos: &mut usize) -> Self {
                S {
                    a: deserialize_scalar::<i64>(buffer, pos),
                    b: deserialize_scalar::<i32>(buffer, pos),
                }
            }
        }

        let v = Some(S { a: 10, b: 20 });

        let mut buf: Vec<u8> = Vec::with_capacity(100);
        let mut pos = serialize_option(&v, &mut buf, 0);

        unsafe {
            buf.set_len(pos);
        }

        pos = 0;
        let res = deserialize_option::<S>(&buf, &mut pos);
        dbg!(res);
    }

    // #[test]
    pub fn test_vec_of_vec() {
        let mut v = vec![vec![1, 2, 3, 4], vec![5, 6, 7, 8]];

        let mut buf: Vec<u8> = Vec::with_capacity(100);
        let mut pos = serialize_vec(&v, &mut buf, 0);

        unsafe {
            v.set_len(pos);
        }

        pos = 0;
        let res = deserialize_vec::<Vec<Vec<i32>>>(&buf, &mut pos);

        dbg!(res);
    }

    #[test]
    pub fn test_various_sizes() {
        #[derive(Debug, PartialEq, Eq)]
        struct S {
            a: i8,
            b: i16,
            c: i32,
            d: i64,
            e: i128,
        }

        let s1 = S {
            a: 1,
            b: 2,
            c: 3,
            d: 4,
            e: 5,
        };

        let mut buf: Vec<u8> = Vec::with_capacity(100);
        let mut pos = serialize_scalar(&s1.a, &mut buf, 0);
        pos = serialize_scalar(&s1.b, &mut buf, pos);
        pos = serialize_scalar(&s1.c, &mut buf, pos);
        pos = serialize_scalar(&s1.d, &mut buf, pos);
        pos = serialize_scalar(&s1.e, &mut buf, pos);
        unsafe {
            buf.set_len(pos);
        }

        let mut offset = 0;
        let s2 = S {
            a: deserialize_scalar::<i8>(&buf, &mut offset),
            b: deserialize_scalar::<i16>(&buf, &mut offset),
            c: deserialize_scalar::<i32>(&buf, &mut offset),
            d: deserialize_scalar::<i64>(&buf, &mut offset),
            e: deserialize_scalar::<i128>(&buf, &mut offset),
        };

        assert_eq!(s1, s2);
    }
}
