use crate::*;

pub use encoder::EngineEncoder;
pub use decoder::EngineDecoder;

pub const ENCODED_LENGTH: usize = 10;

pub mod encoder {
    use super::*;

    #[derive(Debug, Default)]
    pub struct EngineEncoder<P> {
        parent: Option<P>,
        offset: usize,
    }

    impl<'a, P> Writer<'a> for EngineEncoder<P> where P: Writer<'a> + Default {
        #[inline]
        fn get_buf_mut(&mut self) -> &mut WriteBuf<'a> {
            if let Some(parent) = self.parent.as_mut() {
                parent.get_buf_mut()
            } else {
                panic!("parent was None")
            }
        }
    }

    impl<'a, P> EngineEncoder<P> where P: Writer<'a> + Default {
        pub fn wrap(mut self, parent: P, offset: usize) -> Self {
            self.parent = Some(parent);
            self.offset = offset;
            self
        }

        #[inline]
        pub fn parent(&mut self) -> SbeResult<P> {
            self.parent.take().ok_or(SbeErr::ParentNotSet)
        }

        /// primitive field 'capacity'
        /// - min value: 0
        /// - max value: 65534
        /// - null value: 65535
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 0
        /// - encodedLength: 2
        /// - version: 0
        #[inline]
        pub fn capacity(&mut self, value: u16) {
            let offset = self.offset;
            self.get_buf_mut().put_u16_at(offset, value);
        }

        /// primitive field 'numCylinders'
        /// - min value: 0
        /// - max value: 254
        /// - null value: 255
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 2
        /// - encodedLength: 1
        /// - version: 0
        #[inline]
        pub fn num_cylinders(&mut self, value: u8) {
            let offset = self.offset + 2;
            self.get_buf_mut().put_u8_at(offset, value);
        }

        // skipping CONSTANT maxRpm

        /// primitive array field 'manufacturerCode'
        /// - min value: 32
        /// - max value: 126
        /// - null value: 0
        /// - characterEncoding: US-ASCII
        /// - semanticType: null
        /// - encodedOffset: 3
        /// - encodedLength: 3
        /// - version: 0
        #[inline]
        pub fn manufacturer_code(&mut self, value: &[u8; 3]) {
            let offset = self.offset + 3;
            let buf = self.get_buf_mut();
            buf.put_bytes_at(offset, value);
        }

        // skipping CONSTANT fuel

        /// primitive field 'efficiency'
        /// - min value: 0
        /// - max value: 100
        /// - null value: -128
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 6
        /// - encodedLength: 1
        /// - version: 0
        #[inline]
        pub fn efficiency(&mut self, value: i8) {
            let offset = self.offset + 6;
            self.get_buf_mut().put_i8_at(offset, value);
        }

        /// REQUIRED enum
        #[inline]
        pub fn booster_enabled(&mut self, value: boolean_type::BooleanType) {
            let offset = self.offset + 7;
            self.get_buf_mut().put_u8_at(offset, value as u8)
        }

        /// COMPOSITE ENCODER
        #[inline]
        pub fn booster_encoder(self) -> booster_codec::BoosterEncoder<Self> {
            let offset = self.offset + 8;
            booster_codec::BoosterEncoder::default().wrap(self, offset)
        }

    }
} // end encoder mod 

pub mod decoder {
    use super::*;

    #[derive(Debug, Default)]
    pub struct EngineDecoder<P> {
        parent: Option<P>,
        offset: usize,
    }

    impl<'a, P> ActingVersion for EngineDecoder<P> where P: Reader<'a> + ActingVersion + Default {
        #[inline]
        fn acting_version(&self) -> u16 {
            self.parent.as_ref().unwrap().acting_version()
        }
    }

    impl<'a, P> Reader<'a> for EngineDecoder<P> where P: Reader<'a> + Default {
        #[inline]
        fn get_buf(&self) -> &ReadBuf<'a> {
            self.parent.as_ref().expect("parent missing").get_buf()
        }
    }

    impl<'a, P> EngineDecoder<P> where P: Reader<'a> + Default {
        pub fn wrap(mut self, parent: P, offset: usize) -> Self {
            self.parent = Some(parent);
            self.offset = offset;
            self
        }

        #[inline]
        pub fn parent(&mut self) -> SbeResult<P> {
            self.parent.take().ok_or(SbeErr::ParentNotSet)
        }

        /// primitive field - 'REQUIRED'
        #[inline]
        pub fn capacity(&self) -> u16 {
            self.get_buf().get_u16_at(self.offset)
        }

        /// primitive field - 'REQUIRED'
        #[inline]
        pub fn num_cylinders(&self) -> u8 {
            self.get_buf().get_u8_at(self.offset + 2)
        }

        /// CONSTANT 
        #[inline]
        pub fn max_rpm(&self) -> u16 {
            9000
        }

        #[inline]
        pub fn manufacturer_code(&self) -> [u8; 3] {
            let buf = self.get_buf();
            ReadBuf::get_bytes_at(buf.data, self.offset + 3)
        }

        /// CONSTANT 
        /// characterEncoding: 'US-ASCII'
        #[inline]
        pub fn fuel(&self) -> &'static [u8] {
            b"Petrol"
        }

        /// primitive field - 'REQUIRED'
        #[inline]
        pub fn efficiency(&self) -> i8 {
            self.get_buf().get_i8_at(self.offset + 6)
        }

        /// REQUIRED enum
        #[inline]
        pub fn booster_enabled(&self) -> boolean_type::BooleanType {
            self.get_buf().get_u8_at(self.offset + 7).into()
        }

        /// COMPOSITE DECODER
        #[inline]
        pub fn booster_decoder(self) -> booster_codec::BoosterDecoder<Self> {
            let offset = self.offset + 8;
            booster_codec::BoosterDecoder::default().wrap(self, offset)
        }

    }
} // end decoder mod 
