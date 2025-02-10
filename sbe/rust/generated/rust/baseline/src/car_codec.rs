use crate::*;

pub use decoder::CarDecoder;
pub use encoder::CarEncoder;

pub const SBE_BLOCK_LENGTH: u16 = 45;
pub const SBE_TEMPLATE_ID: u16 = 1;
pub const SBE_SCHEMA_ID: u16 = 1;
pub const SBE_SCHEMA_VERSION: u16 = 0;
pub const SBE_SEMANTIC_VERSION: &str = "5.2";

pub mod encoder {
    use super::*;
    use message_header_codec::*;

    #[derive(Debug, Default)]
    pub struct CarEncoder<'a> {
        buf: WriteBuf<'a>,
        initial_offset: usize,
        offset: usize,
        limit: usize,
    }

    impl<'a> Writer<'a> for CarEncoder<'a> {
        #[inline]
        fn get_buf_mut(&mut self) -> &mut WriteBuf<'a> {
            &mut self.buf
        }
    }

    impl<'a> Encoder<'a> for CarEncoder<'a> {
        #[inline]
        fn get_limit(&self) -> usize {
            self.limit
        }

        #[inline]
        fn set_limit(&mut self, limit: usize) {
            self.limit = limit;
        }
    }

    impl<'a> CarEncoder<'a> {
        pub fn wrap(mut self, buf: WriteBuf<'a>, offset: usize) -> Self {
            let limit = offset + SBE_BLOCK_LENGTH as usize;
            self.buf = buf;
            self.initial_offset = offset;
            self.offset = offset;
            self.limit = limit;
            self
        }

        #[inline]
        pub fn encoded_length(&self) -> usize {
            self.limit - self.offset
        }

        pub fn header(self, offset: usize) -> MessageHeaderEncoder<Self> {
            let mut header = MessageHeaderEncoder::default().wrap(self, offset);
            header.block_length(SBE_BLOCK_LENGTH);
            header.template_id(SBE_TEMPLATE_ID);
            header.schema_id(SBE_SCHEMA_ID);
            header.version(SBE_SCHEMA_VERSION);
            header
        }

        /// primitive field 'serialNumber'
        /// - min value: 0
        /// - max value: -2
        /// - null value: -1
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 0
        /// - encodedLength: 8
        /// - version: 0
        #[inline]
        pub fn serial_number(&mut self, value: u64) {
            let offset = self.offset;
            self.get_buf_mut().put_u64_at(offset, value);
        }

        /// primitive field 'modelYear'
        /// - min value: 0
        /// - max value: 65534
        /// - null value: 65535
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 8
        /// - encodedLength: 2
        /// - version: 0
        #[inline]
        pub fn model_year(&mut self, value: u16) {
            let offset = self.offset + 8;
            self.get_buf_mut().put_u16_at(offset, value);
        }

        /// REQUIRED enum
        #[inline]
        pub fn available(&mut self, value: boolean_type::BooleanType) {
            let offset = self.offset + 10;
            self.get_buf_mut().put_u8_at(offset, value as u8)
        }

        /// REQUIRED enum
        #[inline]
        pub fn code(&mut self, value: model::Model) {
            let offset = self.offset + 11;
            self.get_buf_mut().put_u8_at(offset, value as u8)
        }

        /// primitive array field 'someNumbers'
        /// - min value: 0
        /// - max value: 4294967294
        /// - null value: 4294967295
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 12
        /// - encodedLength: 16
        /// - version: 0
        #[inline]
        pub fn some_numbers(&mut self, value: &[u32; 4]) {
            let offset = self.offset + 12;
            let buf = self.get_buf_mut();
            buf.put_u32_at(offset, value[0]);
            buf.put_u32_at(offset + 4, value[1]);
            buf.put_u32_at(offset + 8, value[2]);
            buf.put_u32_at(offset + 12, value[3]);
        }

        /// primitive array field 'vehicleCode'
        /// - min value: 32
        /// - max value: 126
        /// - null value: 0
        /// - characterEncoding: ASCII
        /// - semanticType: null
        /// - encodedOffset: 28
        /// - encodedLength: 6
        /// - version: 0
        #[inline]
        pub fn vehicle_code(&mut self, value: &[u8; 6]) {
            let offset = self.offset + 28;
            let buf = self.get_buf_mut();
            buf.put_bytes_at(offset, value);
        }

        #[inline]
        pub fn extras(&mut self, value: optional_extras::OptionalExtras) {
            let offset = self.offset + 34;
            self.get_buf_mut().put_u8_at(offset, value.0)
        }

        // skipping CONSTANT enum 'discountedModel'

        /// COMPOSITE ENCODER
        #[inline]
        pub fn engine_encoder(self) -> engine_codec::EngineEncoder<Self> {
            let offset = self.offset + 35;
            engine_codec::EngineEncoder::default().wrap(self, offset)
        }

        /// GROUP ENCODER (id=10)
        #[inline]
        pub fn fuel_figures_encoder(self, count: u16, fuel_figures_encoder: FuelFiguresEncoder<Self>) -> FuelFiguresEncoder<Self> {
            fuel_figures_encoder.wrap(self, count)
        }

        /// GROUP ENCODER (id=13)
        #[inline]
        pub fn performance_figures_encoder(self, count: u16, performance_figures_encoder: PerformanceFiguresEncoder<Self>) -> PerformanceFiguresEncoder<Self> {
            performance_figures_encoder.wrap(self, count)
        }

        /// VAR_DATA ENCODER - character encoding: 'UTF-8'
        #[inline]
        pub fn manufacturer(&mut self, value: &str) {
            let limit = self.get_limit();
            let data_length = value.len();
            self.set_limit(limit + 4 + data_length);
            self.get_buf_mut().put_u32_at(limit, data_length as u32);
            self.get_buf_mut().put_slice_at(limit + 4, value.as_bytes());
        }

        /// VAR_DATA ENCODER - character encoding: 'UTF-8'
        #[inline]
        pub fn model(&mut self, value: &str) {
            let limit = self.get_limit();
            let data_length = value.len();
            self.set_limit(limit + 4 + data_length);
            self.get_buf_mut().put_u32_at(limit, data_length as u32);
            self.get_buf_mut().put_slice_at(limit + 4, value.as_bytes());
        }

        /// VAR_DATA ENCODER - character encoding: 'ASCII'
        #[inline]
        pub fn activation_code(&mut self, value: &[u8]) {
            let limit = self.get_limit();
            let data_length = value.len();
            self.set_limit(limit + 4 + data_length);
            self.get_buf_mut().put_u32_at(limit, data_length as u32);
            self.get_buf_mut().put_slice_at(limit + 4, value);
        }

    }

    #[derive(Debug, Default)]
    pub struct FuelFiguresEncoder<P> {
        parent: Option<P>,
        count: u16,
        index: usize,
        offset: usize,
        initial_limit: usize,
    }

    impl<'a, P> Writer<'a> for FuelFiguresEncoder<P> where P: Writer<'a> + Default {
        #[inline]
        fn get_buf_mut(&mut self) -> &mut WriteBuf<'a> {
            if let Some(parent) = self.parent.as_mut() {
                parent.get_buf_mut()
            } else {
                panic!("parent was None")
            }
        }
    }

    impl<'a, P> Encoder<'a> for FuelFiguresEncoder<P> where P: Encoder<'a> + Default {
        #[inline]
        fn get_limit(&self) -> usize {
            self.parent.as_ref().expect("parent missing").get_limit()
        }

        #[inline]
        fn set_limit(&mut self, limit: usize) {
            self.parent.as_mut().expect("parent missing").set_limit(limit);
        }
    }

    impl<'a, P> FuelFiguresEncoder<P> where P: Encoder<'a> + Default {
        #[inline]
        pub fn wrap(
            mut self,
            mut parent: P,
            count: u16,
        ) -> Self {
            let initial_limit = parent.get_limit();
            parent.set_limit(initial_limit + 4);
            parent.get_buf_mut().put_u16_at(initial_limit, Self::block_length());
            parent.get_buf_mut().put_u16_at(initial_limit + 2, count);
            self.parent = Some(parent);
            self.count = count;
            self.index = usize::MAX;
            self.offset = usize::MAX;
            self.initial_limit = initial_limit;
            self
        }

        #[inline]
        pub fn block_length() -> u16 {
            6
        }

        #[inline]
        pub fn parent(&mut self) -> SbeResult<P> {
            self.parent.take().ok_or(SbeErr::ParentNotSet)
        }

        /// will return Some(current index) when successful otherwise None
        #[inline]
        pub fn advance(&mut self) -> SbeResult<Option<usize>> {
            let index = self.index.wrapping_add(1);
            if index >= self.count as usize {
                return Ok(None);
            }
            if let Some(parent) = self.parent.as_mut() {
                self.offset = parent.get_limit();
                parent.set_limit(self.offset + Self::block_length() as usize);
                self.index = index;
                Ok(Some(index))
            } else {
                Err(SbeErr::ParentNotSet)
            }
        }

        /// primitive field 'speed'
        /// - min value: 0
        /// - max value: 65534
        /// - null value: 65535
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 0
        /// - encodedLength: 2
        /// - version: 0
        #[inline]
        pub fn speed(&mut self, value: u16) {
            let offset = self.offset;
            self.get_buf_mut().put_u16_at(offset, value);
        }

        /// primitive field 'mpg'
        /// - min value: -3.4028234663852886E38
        /// - max value: 3.4028234663852886E38
        /// - null value: NaN
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 2
        /// - encodedLength: 4
        /// - version: 0
        #[inline]
        pub fn mpg(&mut self, value: f32) {
            let offset = self.offset + 2;
            self.get_buf_mut().put_f32_at(offset, value);
        }

        /// VAR_DATA ENCODER - character encoding: 'ASCII'
        #[inline]
        pub fn usage_description(&mut self, value: &[u8]) {
            let limit = self.get_limit();
            let data_length = value.len();
            self.set_limit(limit + 4 + data_length);
            self.get_buf_mut().put_u32_at(limit, data_length as u32);
            self.get_buf_mut().put_slice_at(limit + 4, value);
        }

    }

    #[derive(Debug, Default)]
    pub struct PerformanceFiguresEncoder<P> {
        parent: Option<P>,
        count: u16,
        index: usize,
        offset: usize,
        initial_limit: usize,
    }

    impl<'a, P> Writer<'a> for PerformanceFiguresEncoder<P> where P: Writer<'a> + Default {
        #[inline]
        fn get_buf_mut(&mut self) -> &mut WriteBuf<'a> {
            if let Some(parent) = self.parent.as_mut() {
                parent.get_buf_mut()
            } else {
                panic!("parent was None")
            }
        }
    }

    impl<'a, P> Encoder<'a> for PerformanceFiguresEncoder<P> where P: Encoder<'a> + Default {
        #[inline]
        fn get_limit(&self) -> usize {
            self.parent.as_ref().expect("parent missing").get_limit()
        }

        #[inline]
        fn set_limit(&mut self, limit: usize) {
            self.parent.as_mut().expect("parent missing").set_limit(limit);
        }
    }

    impl<'a, P> PerformanceFiguresEncoder<P> where P: Encoder<'a> + Default {
        #[inline]
        pub fn wrap(
            mut self,
            mut parent: P,
            count: u16,
        ) -> Self {
            let initial_limit = parent.get_limit();
            parent.set_limit(initial_limit + 4);
            parent.get_buf_mut().put_u16_at(initial_limit, Self::block_length());
            parent.get_buf_mut().put_u16_at(initial_limit + 2, count);
            self.parent = Some(parent);
            self.count = count;
            self.index = usize::MAX;
            self.offset = usize::MAX;
            self.initial_limit = initial_limit;
            self
        }

        #[inline]
        pub fn block_length() -> u16 {
            1
        }

        #[inline]
        pub fn parent(&mut self) -> SbeResult<P> {
            self.parent.take().ok_or(SbeErr::ParentNotSet)
        }

        /// will return Some(current index) when successful otherwise None
        #[inline]
        pub fn advance(&mut self) -> SbeResult<Option<usize>> {
            let index = self.index.wrapping_add(1);
            if index >= self.count as usize {
                return Ok(None);
            }
            if let Some(parent) = self.parent.as_mut() {
                self.offset = parent.get_limit();
                parent.set_limit(self.offset + Self::block_length() as usize);
                self.index = index;
                Ok(Some(index))
            } else {
                Err(SbeErr::ParentNotSet)
            }
        }

        /// primitive field 'octaneRating'
        /// - min value: 90
        /// - max value: 110
        /// - null value: 255
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 0
        /// - encodedLength: 1
        /// - version: 0
        #[inline]
        pub fn octane_rating(&mut self, value: u8) {
            let offset = self.offset;
            self.get_buf_mut().put_u8_at(offset, value);
        }

        /// GROUP ENCODER (id=15)
        #[inline]
        pub fn acceleration_encoder(self, count: u16, acceleration_encoder: AccelerationEncoder<Self>) -> AccelerationEncoder<Self> {
            acceleration_encoder.wrap(self, count)
        }

    }

    #[derive(Debug, Default)]
    pub struct AccelerationEncoder<P> {
        parent: Option<P>,
        count: u16,
        index: usize,
        offset: usize,
        initial_limit: usize,
    }

    impl<'a, P> Writer<'a> for AccelerationEncoder<P> where P: Writer<'a> + Default {
        #[inline]
        fn get_buf_mut(&mut self) -> &mut WriteBuf<'a> {
            if let Some(parent) = self.parent.as_mut() {
                parent.get_buf_mut()
            } else {
                panic!("parent was None")
            }
        }
    }

    impl<'a, P> Encoder<'a> for AccelerationEncoder<P> where P: Encoder<'a> + Default {
        #[inline]
        fn get_limit(&self) -> usize {
            self.parent.as_ref().expect("parent missing").get_limit()
        }

        #[inline]
        fn set_limit(&mut self, limit: usize) {
            self.parent.as_mut().expect("parent missing").set_limit(limit);
        }
    }

    impl<'a, P> AccelerationEncoder<P> where P: Encoder<'a> + Default {
        #[inline]
        pub fn wrap(
            mut self,
            mut parent: P,
            count: u16,
        ) -> Self {
            let initial_limit = parent.get_limit();
            parent.set_limit(initial_limit + 4);
            parent.get_buf_mut().put_u16_at(initial_limit, Self::block_length());
            parent.get_buf_mut().put_u16_at(initial_limit + 2, count);
            self.parent = Some(parent);
            self.count = count;
            self.index = usize::MAX;
            self.offset = usize::MAX;
            self.initial_limit = initial_limit;
            self
        }

        #[inline]
        pub fn block_length() -> u16 {
            6
        }

        #[inline]
        pub fn parent(&mut self) -> SbeResult<P> {
            self.parent.take().ok_or(SbeErr::ParentNotSet)
        }

        /// will return Some(current index) when successful otherwise None
        #[inline]
        pub fn advance(&mut self) -> SbeResult<Option<usize>> {
            let index = self.index.wrapping_add(1);
            if index >= self.count as usize {
                return Ok(None);
            }
            if let Some(parent) = self.parent.as_mut() {
                self.offset = parent.get_limit();
                parent.set_limit(self.offset + Self::block_length() as usize);
                self.index = index;
                Ok(Some(index))
            } else {
                Err(SbeErr::ParentNotSet)
            }
        }

        /// primitive field 'mph'
        /// - min value: 0
        /// - max value: 65534
        /// - null value: 65535
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 0
        /// - encodedLength: 2
        /// - version: 0
        #[inline]
        pub fn mph(&mut self, value: u16) {
            let offset = self.offset;
            self.get_buf_mut().put_u16_at(offset, value);
        }

        /// primitive field 'seconds'
        /// - min value: -3.4028234663852886E38
        /// - max value: 3.4028234663852886E38
        /// - null value: NaN
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 2
        /// - encodedLength: 4
        /// - version: 0
        #[inline]
        pub fn seconds(&mut self, value: f32) {
            let offset = self.offset + 2;
            self.get_buf_mut().put_f32_at(offset, value);
        }

    }

} // end encoder

pub mod decoder {
    use super::*;
    use message_header_codec::*;

    #[derive(Clone, Copy, Debug, Default)]
    pub struct CarDecoder<'a> {
        buf: ReadBuf<'a>,
        initial_offset: usize,
        offset: usize,
        limit: usize,
        pub acting_block_length: u16,
        pub acting_version: u16,
    }

    impl<'a> ActingVersion for CarDecoder<'a> {
        #[inline]
        fn acting_version(&self) -> u16 {
            self.acting_version
        }
    }

    impl<'a> Reader<'a> for CarDecoder<'a> {
        #[inline]
        fn get_buf(&self) -> &ReadBuf<'a> {
            &self.buf
        }
    }

    impl<'a> Decoder<'a> for CarDecoder<'a> {
        #[inline]
        fn get_limit(&self) -> usize {
            self.limit
        }

        #[inline]
        fn set_limit(&mut self, limit: usize) {
            self.limit = limit;
        }
    }

    impl<'a> CarDecoder<'a> {
        pub fn wrap(
            mut self,
            buf: ReadBuf<'a>,
            offset: usize,
            acting_block_length: u16,
            acting_version: u16,
        ) -> Self {
            let limit = offset + acting_block_length as usize;
            self.buf = buf;
            self.initial_offset = offset;
            self.offset = offset;
            self.limit = limit;
            self.acting_block_length = acting_block_length;
            self.acting_version = acting_version;
            self
        }

        #[inline]
        pub fn encoded_length(&self) -> usize {
            self.limit - self.offset
        }

        pub fn header(self, mut header: MessageHeaderDecoder<ReadBuf<'a>>, offset: usize) -> Self {
            debug_assert_eq!(SBE_TEMPLATE_ID, header.template_id());
            let acting_block_length = header.block_length();
            let acting_version = header.version();

            self.wrap(
                header.parent().unwrap(),
                offset + message_header_codec::ENCODED_LENGTH,
                acting_block_length,
                acting_version,
            )
        }

        /// primitive field - 'REQUIRED'
        #[inline]
        pub fn serial_number(&self) -> u64 {
            self.get_buf().get_u64_at(self.offset)
        }

        /// primitive field - 'REQUIRED'
        #[inline]
        pub fn model_year(&self) -> u16 {
            self.get_buf().get_u16_at(self.offset + 8)
        }

        /// REQUIRED enum
        #[inline]
        pub fn available(&self) -> boolean_type::BooleanType {
            self.get_buf().get_u8_at(self.offset + 10).into()
        }

        /// REQUIRED enum
        #[inline]
        pub fn code(&self) -> model::Model {
            self.get_buf().get_u8_at(self.offset + 11).into()
        }

        #[inline]
        pub fn some_numbers(&self) -> [u32; 4] {
            let buf = self.get_buf();
            [
                buf.get_u32_at(self.offset + 12),
                buf.get_u32_at(self.offset + 12 + 4),
                buf.get_u32_at(self.offset + 12 + 8),
                buf.get_u32_at(self.offset + 12 + 12),
            ]
        }

        #[inline]
        pub fn vehicle_code(&self) -> [u8; 6] {
            let buf = self.get_buf();
            ReadBuf::get_bytes_at(buf.data, self.offset + 28)
        }

        /// BIT SET DECODER
        #[inline]
        pub fn extras(&self) -> optional_extras::OptionalExtras {
            optional_extras::OptionalExtras::new(self.get_buf().get_u8_at(self.offset + 34))
        }

        /// CONSTANT enum
        #[inline]
        pub fn discounted_model(&self) -> model::Model {
            model::Model::C
        }

        /// COMPOSITE DECODER
        #[inline]
        pub fn engine_decoder(self) -> engine_codec::EngineDecoder<Self> {
            let offset = self.offset + 35;
            engine_codec::EngineDecoder::default().wrap(self, offset)
        }

        /// GROUP DECODER (id=10)
        #[inline]
        pub fn fuel_figures_decoder(self) -> FuelFiguresDecoder<Self> {
            FuelFiguresDecoder::default().wrap(self)
        }

        /// GROUP DECODER (id=13)
        #[inline]
        pub fn performance_figures_decoder(self) -> PerformanceFiguresDecoder<Self> {
            PerformanceFiguresDecoder::default().wrap(self)
        }

        /// VAR_DATA DECODER - character encoding: 'UTF-8'
        #[inline]
        pub fn manufacturer_decoder(&mut self) -> (usize, usize) {
            let offset = self.get_limit();
            let data_length = self.get_buf().get_u32_at(offset) as usize;
            self.set_limit(offset + 4 + data_length);
            (offset + 4, data_length)
        }

        #[inline]
        pub fn manufacturer_slice(&'a self, coordinates: (usize, usize)) -> &'a [u8] {
            debug_assert!(self.get_limit() >= coordinates.0 + coordinates.1);
            self.get_buf().get_slice_at(coordinates.0, coordinates.1)
        }

        /// VAR_DATA DECODER - character encoding: 'UTF-8'
        #[inline]
        pub fn model_decoder(&mut self) -> (usize, usize) {
            let offset = self.get_limit();
            let data_length = self.get_buf().get_u32_at(offset) as usize;
            self.set_limit(offset + 4 + data_length);
            (offset + 4, data_length)
        }

        #[inline]
        pub fn model_slice(&'a self, coordinates: (usize, usize)) -> &'a [u8] {
            debug_assert!(self.get_limit() >= coordinates.0 + coordinates.1);
            self.get_buf().get_slice_at(coordinates.0, coordinates.1)
        }

        /// VAR_DATA DECODER - character encoding: 'ASCII'
        #[inline]
        pub fn activation_code_decoder(&mut self) -> (usize, usize) {
            let offset = self.get_limit();
            let data_length = self.get_buf().get_u32_at(offset) as usize;
            self.set_limit(offset + 4 + data_length);
            (offset + 4, data_length)
        }

        #[inline]
        pub fn activation_code_slice(&'a self, coordinates: (usize, usize)) -> &'a [u8] {
            debug_assert!(self.get_limit() >= coordinates.0 + coordinates.1);
            self.get_buf().get_slice_at(coordinates.0, coordinates.1)
        }

    }

    #[derive(Debug, Default)]
    pub struct FuelFiguresDecoder<P> {
        parent: Option<P>,
        block_length: u16,
        count: u16,
        index: usize,
        offset: usize,
    }

    impl<'a, P> ActingVersion for FuelFiguresDecoder<P> where P: Reader<'a> + ActingVersion + Default {
        #[inline]
        fn acting_version(&self) -> u16 {
            self.parent.as_ref().unwrap().acting_version()
        }
    }

    impl<'a, P> Reader<'a> for FuelFiguresDecoder<P> where P: Reader<'a> + Default {
        #[inline]
        fn get_buf(&self) -> &ReadBuf<'a> {
            self.parent.as_ref().expect("parent missing").get_buf()
        }
    }

    impl<'a, P> Decoder<'a> for FuelFiguresDecoder<P> where P: Decoder<'a> + ActingVersion + Default {
        #[inline]
        fn get_limit(&self) -> usize {
            self.parent.as_ref().expect("parent missing").get_limit()
        }

        #[inline]
        fn set_limit(&mut self, limit: usize) {
            self.parent.as_mut().expect("parent missing").set_limit(limit);
        }
    }

    impl<'a, P> FuelFiguresDecoder<P> where P: Decoder<'a> + ActingVersion + Default {
        pub fn wrap(
            mut self,
            mut parent: P,
        ) -> Self {
            let initial_offset = parent.get_limit();
            let block_length = parent.get_buf().get_u16_at(initial_offset);
            let count = parent.get_buf().get_u16_at(initial_offset + 2);
            parent.set_limit(initial_offset + 4);
            self.parent = Some(parent);
            self.block_length = block_length;
            self.count = count;
            self.index = usize::MAX;
            self.offset = 0;
            self
        }

        /// group token - Token{signal=BEGIN_GROUP, name='fuelFigures', referencedName='null', description='null', packageName='null', id=10, version=0, deprecated=0, encodedLength=6, offset=45, componentTokenCount=18, encoding=Encoding{presence=REQUIRED, primitiveType=null, byteOrder=LITTLE_ENDIAN, minValue=null, maxValue=null, nullValue=null, constValue=null, characterEncoding='null', epoch='null', timeUnit=null, semanticType='null'}}
        #[inline]
        pub fn parent(&mut self) -> SbeResult<P> {
            self.parent.take().ok_or(SbeErr::ParentNotSet)
        }

        #[inline]
        pub fn acting_version(&mut self) -> u16 {
            self.parent.as_ref().unwrap().acting_version()
        }

        #[inline]
        pub fn count(&self) -> u16 {
            self.count
        }

        /// will return Some(current index) when successful otherwise None
        pub fn advance(&mut self) -> SbeResult<Option<usize>> {
            let index = self.index.wrapping_add(1);
            if index >= self.count as usize {
                 return Ok(None);
            }
            if let Some(parent) = self.parent.as_mut() {
                self.offset = parent.get_limit();
                parent.set_limit(self.offset + self.block_length as usize);
                self.index = index;
                Ok(Some(index))
            } else {
                Err(SbeErr::ParentNotSet)
            }
        }

        /// primitive field - 'REQUIRED'
        #[inline]
        pub fn speed(&self) -> u16 {
            self.get_buf().get_u16_at(self.offset)
        }

        /// primitive field - 'REQUIRED'
        #[inline]
        pub fn mpg(&self) -> f32 {
            self.get_buf().get_f32_at(self.offset + 2)
        }

        /// VAR_DATA DECODER - character encoding: 'ASCII'
        #[inline]
        pub fn usage_description_decoder(&mut self) -> (usize, usize) {
            let offset = self.parent.as_ref().expect("parent missing").get_limit();
            let data_length = self.get_buf().get_u32_at(offset) as usize;
            self.parent.as_mut().unwrap().set_limit(offset + 4 + data_length);
            (offset + 4, data_length)
        }

        #[inline]
        pub fn usage_description_slice(&'a self, coordinates: (usize, usize)) -> &'a [u8] {
            debug_assert!(self.get_limit() >= coordinates.0 + coordinates.1);
            self.get_buf().get_slice_at(coordinates.0, coordinates.1)
        }

    }

    #[derive(Debug, Default)]
    pub struct PerformanceFiguresDecoder<P> {
        parent: Option<P>,
        block_length: u16,
        count: u16,
        index: usize,
        offset: usize,
    }

    impl<'a, P> ActingVersion for PerformanceFiguresDecoder<P> where P: Reader<'a> + ActingVersion + Default {
        #[inline]
        fn acting_version(&self) -> u16 {
            self.parent.as_ref().unwrap().acting_version()
        }
    }

    impl<'a, P> Reader<'a> for PerformanceFiguresDecoder<P> where P: Reader<'a> + Default {
        #[inline]
        fn get_buf(&self) -> &ReadBuf<'a> {
            self.parent.as_ref().expect("parent missing").get_buf()
        }
    }

    impl<'a, P> Decoder<'a> for PerformanceFiguresDecoder<P> where P: Decoder<'a> + ActingVersion + Default {
        #[inline]
        fn get_limit(&self) -> usize {
            self.parent.as_ref().expect("parent missing").get_limit()
        }

        #[inline]
        fn set_limit(&mut self, limit: usize) {
            self.parent.as_mut().expect("parent missing").set_limit(limit);
        }
    }

    impl<'a, P> PerformanceFiguresDecoder<P> where P: Decoder<'a> + ActingVersion + Default {
        pub fn wrap(
            mut self,
            mut parent: P,
        ) -> Self {
            let initial_offset = parent.get_limit();
            let block_length = parent.get_buf().get_u16_at(initial_offset);
            let count = parent.get_buf().get_u16_at(initial_offset + 2);
            parent.set_limit(initial_offset + 4);
            self.parent = Some(parent);
            self.block_length = block_length;
            self.count = count;
            self.index = usize::MAX;
            self.offset = 0;
            self
        }

        /// group token - Token{signal=BEGIN_GROUP, name='performanceFigures', referencedName='null', description='null', packageName='null', id=13, version=0, deprecated=0, encodedLength=1, offset=-1, componentTokenCount=21, encoding=Encoding{presence=REQUIRED, primitiveType=null, byteOrder=LITTLE_ENDIAN, minValue=null, maxValue=null, nullValue=null, constValue=null, characterEncoding='null', epoch='null', timeUnit=null, semanticType='null'}}
        #[inline]
        pub fn parent(&mut self) -> SbeResult<P> {
            self.parent.take().ok_or(SbeErr::ParentNotSet)
        }

        #[inline]
        pub fn acting_version(&mut self) -> u16 {
            self.parent.as_ref().unwrap().acting_version()
        }

        #[inline]
        pub fn count(&self) -> u16 {
            self.count
        }

        /// will return Some(current index) when successful otherwise None
        pub fn advance(&mut self) -> SbeResult<Option<usize>> {
            let index = self.index.wrapping_add(1);
            if index >= self.count as usize {
                 return Ok(None);
            }
            if let Some(parent) = self.parent.as_mut() {
                self.offset = parent.get_limit();
                parent.set_limit(self.offset + self.block_length as usize);
                self.index = index;
                Ok(Some(index))
            } else {
                Err(SbeErr::ParentNotSet)
            }
        }

        /// primitive field - 'REQUIRED'
        #[inline]
        pub fn octane_rating(&self) -> u8 {
            self.get_buf().get_u8_at(self.offset)
        }

        /// GROUP DECODER (id=15)
        #[inline]
        pub fn acceleration_decoder(self) -> AccelerationDecoder<Self> {
            AccelerationDecoder::default().wrap(self)
        }

    }

    #[derive(Debug, Default)]
    pub struct AccelerationDecoder<P> {
        parent: Option<P>,
        block_length: u16,
        count: u16,
        index: usize,
        offset: usize,
    }

    impl<'a, P> ActingVersion for AccelerationDecoder<P> where P: Reader<'a> + ActingVersion + Default {
        #[inline]
        fn acting_version(&self) -> u16 {
            self.parent.as_ref().unwrap().acting_version()
        }
    }

    impl<'a, P> Reader<'a> for AccelerationDecoder<P> where P: Reader<'a> + Default {
        #[inline]
        fn get_buf(&self) -> &ReadBuf<'a> {
            self.parent.as_ref().expect("parent missing").get_buf()
        }
    }

    impl<'a, P> Decoder<'a> for AccelerationDecoder<P> where P: Decoder<'a> + ActingVersion + Default {
        #[inline]
        fn get_limit(&self) -> usize {
            self.parent.as_ref().expect("parent missing").get_limit()
        }

        #[inline]
        fn set_limit(&mut self, limit: usize) {
            self.parent.as_mut().expect("parent missing").set_limit(limit);
        }
    }

    impl<'a, P> AccelerationDecoder<P> where P: Decoder<'a> + ActingVersion + Default {
        pub fn wrap(
            mut self,
            mut parent: P,
        ) -> Self {
            let initial_offset = parent.get_limit();
            let block_length = parent.get_buf().get_u16_at(initial_offset);
            let count = parent.get_buf().get_u16_at(initial_offset + 2);
            parent.set_limit(initial_offset + 4);
            self.parent = Some(parent);
            self.block_length = block_length;
            self.count = count;
            self.index = usize::MAX;
            self.offset = 0;
            self
        }

        /// group token - Token{signal=BEGIN_GROUP, name='acceleration', referencedName='null', description='null', packageName='null', id=15, version=0, deprecated=0, encodedLength=6, offset=1, componentTokenCount=12, encoding=Encoding{presence=REQUIRED, primitiveType=null, byteOrder=LITTLE_ENDIAN, minValue=null, maxValue=null, nullValue=null, constValue=null, characterEncoding='null', epoch='null', timeUnit=null, semanticType='null'}}
        #[inline]
        pub fn parent(&mut self) -> SbeResult<P> {
            self.parent.take().ok_or(SbeErr::ParentNotSet)
        }

        #[inline]
        pub fn acting_version(&mut self) -> u16 {
            self.parent.as_ref().unwrap().acting_version()
        }

        #[inline]
        pub fn count(&self) -> u16 {
            self.count
        }

        /// will return Some(current index) when successful otherwise None
        pub fn advance(&mut self) -> SbeResult<Option<usize>> {
            let index = self.index.wrapping_add(1);
            if index >= self.count as usize {
                 return Ok(None);
            }
            if let Some(parent) = self.parent.as_mut() {
                self.offset = parent.get_limit();
                parent.set_limit(self.offset + self.block_length as usize);
                self.index = index;
                Ok(Some(index))
            } else {
                Err(SbeErr::ParentNotSet)
            }
        }

        /// primitive field - 'REQUIRED'
        #[inline]
        pub fn mph(&self) -> u16 {
            self.get_buf().get_u16_at(self.offset)
        }

        /// primitive field - 'REQUIRED'
        #[inline]
        pub fn seconds(&self) -> f32 {
            self.get_buf().get_f32_at(self.offset + 2)
        }

    }

} // end decoder

