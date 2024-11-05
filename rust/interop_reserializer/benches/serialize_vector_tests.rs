#[cfg(test)]
pub mod serialize_vector_tests {
    use generated_mod::{
        common_deserialize::deserialize_scalar,
        common_serialize::serialize_scalar,
    };

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
