// Generated file, please don't edit manually.

impl LabelPair {
    pub fn new_() -> LabelPair {
        ::std::default::Default::default()
    }
    #[inline]
    pub fn has_name(&self) -> bool {
        self.name.is_some()
    }
    #[inline]
    pub fn clear_name(&mut self) {
        self.name = ::std::option::Option::None
    }
    #[inline]
    pub fn set_name(&mut self, v: std::string::String) {
        self.name = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_name(&self) -> &str {
        match self.name.as_ref() {
            Some(v) => v,
            None => "",
        }
    }
    #[inline]
    pub fn mut_name(&mut self) -> &mut std::string::String {
        if self.name.is_none() {
            self.name = ::std::option::Option::Some(std::string::String::default());
        }
        self.name.as_mut().unwrap()
    }
    #[inline]
    pub fn take_name(&mut self) -> std::string::String {
        self.name.take().unwrap_or_else(::std::string::String::new)
    }
    #[inline]
    pub fn has_value(&self) -> bool {
        self.value.is_some()
    }
    #[inline]
    pub fn clear_value(&mut self) {
        self.value = ::std::option::Option::None
    }
    #[inline]
    pub fn set_value(&mut self, v: std::string::String) {
        self.value = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_value(&self) -> &str {
        match self.value.as_ref() {
            Some(v) => v,
            None => "",
        }
    }
    #[inline]
    pub fn mut_value(&mut self) -> &mut std::string::String {
        if self.value.is_none() {
            self.value = ::std::option::Option::Some(std::string::String::default());
        }
        self.value.as_mut().unwrap()
    }
    #[inline]
    pub fn take_value(&mut self) -> std::string::String {
        self.value.take().unwrap_or_else(::std::string::String::new)
    }
}
impl ::protobuf::Clear for LabelPair {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for LabelPair {
    fn compute_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn get_cached_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn new() -> Self {
        Self::new_()
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
    }
    fn default_instance() -> &'static LabelPair {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: LabelPair = LabelPair::new_();
        }
        &*INSTANCE
    }
    fn is_initialized(&self) -> bool {
        true
    }
    fn merge_from(
        &mut self,
        _is: &mut ::protobuf::CodedInputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
    }
    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        unimplemented!();
    }
    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        unimplemented!();
    }
    fn write_to_bytes(&self) -> ::protobuf::ProtobufResult<Vec<u8>> {
        let mut buf = Vec::new();
        if let Err(_) = ::prost::Message::encode(self, &mut buf) {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(buf)
    }
    fn merge_from_bytes(&mut self, bytes: &[u8]) -> ::protobuf::ProtobufResult<()> {
        if let Err(_) = ::prost::Message::merge(self, bytes) {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(())
    }
}
impl Gauge {
    pub fn new_() -> Gauge {
        ::std::default::Default::default()
    }
    #[inline]
    pub fn has_value(&self) -> bool {
        self.value.is_some()
    }
    #[inline]
    pub fn clear_value(&mut self) {
        self.value = ::std::option::Option::None
    }
    #[inline]
    pub fn set_value(&mut self, v: f64) {
        self.value = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_value(&self) -> f64 {
        match self.value {
            Some(v) => v,
            None => 0.,
        }
    }
}
impl ::protobuf::Clear for Gauge {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for Gauge {
    fn compute_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn get_cached_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn new() -> Self {
        Self::new_()
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
    }
    fn default_instance() -> &'static Gauge {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: Gauge = Gauge::new_();
        }
        &*INSTANCE
    }
    fn is_initialized(&self) -> bool {
        true
    }
    fn merge_from(
        &mut self,
        _is: &mut ::protobuf::CodedInputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
    }
    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        unimplemented!();
    }
    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        unimplemented!();
    }
    fn write_to_bytes(&self) -> ::protobuf::ProtobufResult<Vec<u8>> {
        let mut buf = Vec::new();
        if let Err(_) = ::prost::Message::encode(self, &mut buf) {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(buf)
    }
    fn merge_from_bytes(&mut self, bytes: &[u8]) -> ::protobuf::ProtobufResult<()> {
        if let Err(_) = ::prost::Message::merge(self, bytes) {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(())
    }
}
impl Counter {
    pub fn new_() -> Counter {
        ::std::default::Default::default()
    }
    #[inline]
    pub fn has_value(&self) -> bool {
        self.value.is_some()
    }
    #[inline]
    pub fn clear_value(&mut self) {
        self.value = ::std::option::Option::None
    }
    #[inline]
    pub fn set_value(&mut self, v: f64) {
        self.value = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_value(&self) -> f64 {
        match self.value {
            Some(v) => v,
            None => 0.,
        }
    }
}
impl ::protobuf::Clear for Counter {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for Counter {
    fn compute_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn get_cached_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn new() -> Self {
        Self::new_()
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
    }
    fn default_instance() -> &'static Counter {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: Counter = Counter::new_();
        }
        &*INSTANCE
    }
    fn is_initialized(&self) -> bool {
        true
    }
    fn merge_from(
        &mut self,
        _is: &mut ::protobuf::CodedInputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
    }
    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        unimplemented!();
    }
    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        unimplemented!();
    }
    fn write_to_bytes(&self) -> ::protobuf::ProtobufResult<Vec<u8>> {
        let mut buf = Vec::new();
        if let Err(_) = ::prost::Message::encode(self, &mut buf) {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(buf)
    }
    fn merge_from_bytes(&mut self, bytes: &[u8]) -> ::protobuf::ProtobufResult<()> {
        if let Err(_) = ::prost::Message::merge(self, bytes) {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(())
    }
}
impl Quantile {
    pub fn new_() -> Quantile {
        ::std::default::Default::default()
    }
    #[inline]
    pub fn has_quantile(&self) -> bool {
        self.quantile.is_some()
    }
    #[inline]
    pub fn clear_quantile(&mut self) {
        self.quantile = ::std::option::Option::None
    }
    #[inline]
    pub fn set_quantile(&mut self, v: f64) {
        self.quantile = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_quantile(&self) -> f64 {
        match self.quantile {
            Some(v) => v,
            None => 0.,
        }
    }
    #[inline]
    pub fn has_value(&self) -> bool {
        self.value.is_some()
    }
    #[inline]
    pub fn clear_value(&mut self) {
        self.value = ::std::option::Option::None
    }
    #[inline]
    pub fn set_value(&mut self, v: f64) {
        self.value = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_value(&self) -> f64 {
        match self.value {
            Some(v) => v,
            None => 0.,
        }
    }
}
impl ::protobuf::Clear for Quantile {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for Quantile {
    fn compute_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn get_cached_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn new() -> Self {
        Self::new_()
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
    }
    fn default_instance() -> &'static Quantile {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: Quantile = Quantile::new_();
        }
        &*INSTANCE
    }
    fn is_initialized(&self) -> bool {
        true
    }
    fn merge_from(
        &mut self,
        _is: &mut ::protobuf::CodedInputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
    }
    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        unimplemented!();
    }
    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        unimplemented!();
    }
    fn write_to_bytes(&self) -> ::protobuf::ProtobufResult<Vec<u8>> {
        let mut buf = Vec::new();
        if let Err(_) = ::prost::Message::encode(self, &mut buf) {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(buf)
    }
    fn merge_from_bytes(&mut self, bytes: &[u8]) -> ::protobuf::ProtobufResult<()> {
        if let Err(_) = ::prost::Message::merge(self, bytes) {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(())
    }
}
impl Summary {
    pub fn new_() -> Summary {
        ::std::default::Default::default()
    }
    #[inline]
    pub fn has_sample_count(&self) -> bool {
        self.sample_count.is_some()
    }
    #[inline]
    pub fn clear_sample_count(&mut self) {
        self.sample_count = ::std::option::Option::None
    }
    #[inline]
    pub fn set_sample_count(&mut self, v: u64) {
        self.sample_count = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_sample_count(&self) -> u64 {
        match self.sample_count {
            Some(v) => v,
            None => 0,
        }
    }
    #[inline]
    pub fn has_sample_sum(&self) -> bool {
        self.sample_sum.is_some()
    }
    #[inline]
    pub fn clear_sample_sum(&mut self) {
        self.sample_sum = ::std::option::Option::None
    }
    #[inline]
    pub fn set_sample_sum(&mut self, v: f64) {
        self.sample_sum = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_sample_sum(&self) -> f64 {
        match self.sample_sum {
            Some(v) => v,
            None => 0.,
        }
    }
    #[inline]
    pub fn clear_quantile(&mut self) {
        self.quantile.clear();
    }
    #[inline]
    pub fn set_quantile(&mut self, v: ::std::vec::Vec<Quantile>) {
        self.quantile = v;
    }
    #[inline]
    pub fn get_quantile(&self) -> &::std::vec::Vec<Quantile> {
        &self.quantile
    }
    #[inline]
    pub fn mut_quantile(&mut self) -> &mut ::std::vec::Vec<Quantile> {
        &mut self.quantile
    }
    #[inline]
    pub fn take_quantile(&mut self) -> ::std::vec::Vec<Quantile> {
        ::std::mem::replace(&mut self.quantile, ::std::vec::Vec::new())
    }
}
impl ::protobuf::Clear for Summary {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for Summary {
    fn compute_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn get_cached_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn new() -> Self {
        Self::new_()
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
    }
    fn default_instance() -> &'static Summary {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: Summary = Summary::new_();
        }
        &*INSTANCE
    }
    fn is_initialized(&self) -> bool {
        true
    }
    fn merge_from(
        &mut self,
        _is: &mut ::protobuf::CodedInputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
    }
    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        unimplemented!();
    }
    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        unimplemented!();
    }
    fn write_to_bytes(&self) -> ::protobuf::ProtobufResult<Vec<u8>> {
        let mut buf = Vec::new();
        if let Err(_) = ::prost::Message::encode(self, &mut buf) {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(buf)
    }
    fn merge_from_bytes(&mut self, bytes: &[u8]) -> ::protobuf::ProtobufResult<()> {
        if let Err(_) = ::prost::Message::merge(self, bytes) {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(())
    }
}
impl Untyped {
    pub fn new_() -> Untyped {
        ::std::default::Default::default()
    }
    #[inline]
    pub fn has_value(&self) -> bool {
        self.value.is_some()
    }
    #[inline]
    pub fn clear_value(&mut self) {
        self.value = ::std::option::Option::None
    }
    #[inline]
    pub fn set_value(&mut self, v: f64) {
        self.value = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_value(&self) -> f64 {
        match self.value {
            Some(v) => v,
            None => 0.,
        }
    }
}
impl ::protobuf::Clear for Untyped {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for Untyped {
    fn compute_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn get_cached_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn new() -> Self {
        Self::new_()
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
    }
    fn default_instance() -> &'static Untyped {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: Untyped = Untyped::new_();
        }
        &*INSTANCE
    }
    fn is_initialized(&self) -> bool {
        true
    }
    fn merge_from(
        &mut self,
        _is: &mut ::protobuf::CodedInputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
    }
    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        unimplemented!();
    }
    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        unimplemented!();
    }
    fn write_to_bytes(&self) -> ::protobuf::ProtobufResult<Vec<u8>> {
        let mut buf = Vec::new();
        if let Err(_) = ::prost::Message::encode(self, &mut buf) {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(buf)
    }
    fn merge_from_bytes(&mut self, bytes: &[u8]) -> ::protobuf::ProtobufResult<()> {
        if let Err(_) = ::prost::Message::merge(self, bytes) {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(())
    }
}
impl Histogram {
    pub fn new_() -> Histogram {
        ::std::default::Default::default()
    }
    #[inline]
    pub fn has_sample_count(&self) -> bool {
        self.sample_count.is_some()
    }
    #[inline]
    pub fn clear_sample_count(&mut self) {
        self.sample_count = ::std::option::Option::None
    }
    #[inline]
    pub fn set_sample_count(&mut self, v: u64) {
        self.sample_count = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_sample_count(&self) -> u64 {
        match self.sample_count {
            Some(v) => v,
            None => 0,
        }
    }
    #[inline]
    pub fn has_sample_sum(&self) -> bool {
        self.sample_sum.is_some()
    }
    #[inline]
    pub fn clear_sample_sum(&mut self) {
        self.sample_sum = ::std::option::Option::None
    }
    #[inline]
    pub fn set_sample_sum(&mut self, v: f64) {
        self.sample_sum = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_sample_sum(&self) -> f64 {
        match self.sample_sum {
            Some(v) => v,
            None => 0.,
        }
    }
    #[inline]
    pub fn clear_bucket(&mut self) {
        self.bucket.clear();
    }
    #[inline]
    pub fn set_bucket(&mut self, v: ::std::vec::Vec<Bucket>) {
        self.bucket = v;
    }
    #[inline]
    pub fn get_bucket(&self) -> &::std::vec::Vec<Bucket> {
        &self.bucket
    }
    #[inline]
    pub fn mut_bucket(&mut self) -> &mut ::std::vec::Vec<Bucket> {
        &mut self.bucket
    }
    #[inline]
    pub fn take_bucket(&mut self) -> ::std::vec::Vec<Bucket> {
        ::std::mem::replace(&mut self.bucket, ::std::vec::Vec::new())
    }
}
impl ::protobuf::Clear for Histogram {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for Histogram {
    fn compute_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn get_cached_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn new() -> Self {
        Self::new_()
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
    }
    fn default_instance() -> &'static Histogram {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: Histogram = Histogram::new_();
        }
        &*INSTANCE
    }
    fn is_initialized(&self) -> bool {
        true
    }
    fn merge_from(
        &mut self,
        _is: &mut ::protobuf::CodedInputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
    }
    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        unimplemented!();
    }
    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        unimplemented!();
    }
    fn write_to_bytes(&self) -> ::protobuf::ProtobufResult<Vec<u8>> {
        let mut buf = Vec::new();
        if let Err(_) = ::prost::Message::encode(self, &mut buf) {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(buf)
    }
    fn merge_from_bytes(&mut self, bytes: &[u8]) -> ::protobuf::ProtobufResult<()> {
        if let Err(_) = ::prost::Message::merge(self, bytes) {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(())
    }
}
impl Bucket {
    pub fn new_() -> Bucket {
        ::std::default::Default::default()
    }
    #[inline]
    pub fn has_cumulative_count(&self) -> bool {
        self.cumulative_count.is_some()
    }
    #[inline]
    pub fn clear_cumulative_count(&mut self) {
        self.cumulative_count = ::std::option::Option::None
    }
    #[inline]
    pub fn set_cumulative_count(&mut self, v: u64) {
        self.cumulative_count = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_cumulative_count(&self) -> u64 {
        match self.cumulative_count {
            Some(v) => v,
            None => 0,
        }
    }
    #[inline]
    pub fn has_upper_bound(&self) -> bool {
        self.upper_bound.is_some()
    }
    #[inline]
    pub fn clear_upper_bound(&mut self) {
        self.upper_bound = ::std::option::Option::None
    }
    #[inline]
    pub fn set_upper_bound(&mut self, v: f64) {
        self.upper_bound = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_upper_bound(&self) -> f64 {
        match self.upper_bound {
            Some(v) => v,
            None => 0.,
        }
    }
}
impl ::protobuf::Clear for Bucket {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for Bucket {
    fn compute_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn get_cached_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn new() -> Self {
        Self::new_()
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
    }
    fn default_instance() -> &'static Bucket {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: Bucket = Bucket::new_();
        }
        &*INSTANCE
    }
    fn is_initialized(&self) -> bool {
        true
    }
    fn merge_from(
        &mut self,
        _is: &mut ::protobuf::CodedInputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
    }
    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        unimplemented!();
    }
    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        unimplemented!();
    }
    fn write_to_bytes(&self) -> ::protobuf::ProtobufResult<Vec<u8>> {
        let mut buf = Vec::new();
        if let Err(_) = ::prost::Message::encode(self, &mut buf) {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(buf)
    }
    fn merge_from_bytes(&mut self, bytes: &[u8]) -> ::protobuf::ProtobufResult<()> {
        if let Err(_) = ::prost::Message::merge(self, bytes) {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(())
    }
}
impl Metric {
    pub fn new_() -> Metric {
        ::std::default::Default::default()
    }
    #[inline]
    pub fn clear_label(&mut self) {
        self.label.clear();
    }
    #[inline]
    pub fn set_label(&mut self, v: ::std::vec::Vec<LabelPair>) {
        self.label = v;
    }
    #[inline]
    pub fn get_label(&self) -> &::std::vec::Vec<LabelPair> {
        &self.label
    }
    #[inline]
    pub fn mut_label(&mut self) -> &mut ::std::vec::Vec<LabelPair> {
        &mut self.label
    }
    #[inline]
    pub fn take_label(&mut self) -> ::std::vec::Vec<LabelPair> {
        ::std::mem::replace(&mut self.label, ::std::vec::Vec::new())
    }
    #[inline]
    pub fn has_gauge(&self) -> bool {
        self.gauge.is_some()
    }
    #[inline]
    pub fn clear_gauge(&mut self) {
        self.gauge = ::std::option::Option::None
    }
    #[inline]
    pub fn set_gauge(&mut self, v: Gauge) {
        self.gauge = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_gauge(&self) -> &Gauge {
        match self.gauge.as_ref() {
            Some(v) => v,
            None => <Gauge as ::protobuf::Message>::default_instance(),
        }
    }
    #[inline]
    pub fn mut_gauge(&mut self) -> &mut Gauge {
        if self.gauge.is_none() {
            self.gauge = ::std::option::Option::Some(Gauge::default());
        }
        self.gauge.as_mut().unwrap()
    }
    #[inline]
    pub fn take_gauge(&mut self) -> Gauge {
        self.gauge.take().unwrap_or_else(Gauge::default)
    }
    #[inline]
    pub fn has_counter(&self) -> bool {
        self.counter.is_some()
    }
    #[inline]
    pub fn clear_counter(&mut self) {
        self.counter = ::std::option::Option::None
    }
    #[inline]
    pub fn set_counter(&mut self, v: Counter) {
        self.counter = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_counter(&self) -> &Counter {
        match self.counter.as_ref() {
            Some(v) => v,
            None => <Counter as ::protobuf::Message>::default_instance(),
        }
    }
    #[inline]
    pub fn mut_counter(&mut self) -> &mut Counter {
        if self.counter.is_none() {
            self.counter = ::std::option::Option::Some(Counter::default());
        }
        self.counter.as_mut().unwrap()
    }
    #[inline]
    pub fn take_counter(&mut self) -> Counter {
        self.counter.take().unwrap_or_else(Counter::default)
    }
    #[inline]
    pub fn has_summary(&self) -> bool {
        self.summary.is_some()
    }
    #[inline]
    pub fn clear_summary(&mut self) {
        self.summary = ::std::option::Option::None
    }
    #[inline]
    pub fn set_summary(&mut self, v: Summary) {
        self.summary = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_summary(&self) -> &Summary {
        match self.summary.as_ref() {
            Some(v) => v,
            None => <Summary as ::protobuf::Message>::default_instance(),
        }
    }
    #[inline]
    pub fn mut_summary(&mut self) -> &mut Summary {
        if self.summary.is_none() {
            self.summary = ::std::option::Option::Some(Summary::default());
        }
        self.summary.as_mut().unwrap()
    }
    #[inline]
    pub fn take_summary(&mut self) -> Summary {
        self.summary.take().unwrap_or_else(Summary::default)
    }
    #[inline]
    pub fn has_untyped(&self) -> bool {
        self.untyped.is_some()
    }
    #[inline]
    pub fn clear_untyped(&mut self) {
        self.untyped = ::std::option::Option::None
    }
    #[inline]
    pub fn set_untyped(&mut self, v: Untyped) {
        self.untyped = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_untyped(&self) -> &Untyped {
        match self.untyped.as_ref() {
            Some(v) => v,
            None => <Untyped as ::protobuf::Message>::default_instance(),
        }
    }
    #[inline]
    pub fn mut_untyped(&mut self) -> &mut Untyped {
        if self.untyped.is_none() {
            self.untyped = ::std::option::Option::Some(Untyped::default());
        }
        self.untyped.as_mut().unwrap()
    }
    #[inline]
    pub fn take_untyped(&mut self) -> Untyped {
        self.untyped.take().unwrap_or_else(Untyped::default)
    }
    #[inline]
    pub fn has_histogram(&self) -> bool {
        self.histogram.is_some()
    }
    #[inline]
    pub fn clear_histogram(&mut self) {
        self.histogram = ::std::option::Option::None
    }
    #[inline]
    pub fn set_histogram(&mut self, v: Histogram) {
        self.histogram = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_histogram(&self) -> &Histogram {
        match self.histogram.as_ref() {
            Some(v) => v,
            None => <Histogram as ::protobuf::Message>::default_instance(),
        }
    }
    #[inline]
    pub fn mut_histogram(&mut self) -> &mut Histogram {
        if self.histogram.is_none() {
            self.histogram = ::std::option::Option::Some(Histogram::default());
        }
        self.histogram.as_mut().unwrap()
    }
    #[inline]
    pub fn take_histogram(&mut self) -> Histogram {
        self.histogram.take().unwrap_or_else(Histogram::default)
    }
    #[inline]
    pub fn has_timestamp_ms(&self) -> bool {
        self.timestamp_ms.is_some()
    }
    #[inline]
    pub fn clear_timestamp_ms(&mut self) {
        self.timestamp_ms = ::std::option::Option::None
    }
    #[inline]
    pub fn set_timestamp_ms(&mut self, v: i64) {
        self.timestamp_ms = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_timestamp_ms(&self) -> i64 {
        match self.timestamp_ms {
            Some(v) => v,
            None => 0,
        }
    }
}
impl ::protobuf::Clear for Metric {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for Metric {
    fn compute_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn get_cached_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn new() -> Self {
        Self::new_()
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
    }
    fn default_instance() -> &'static Metric {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: Metric = Metric::new_();
        }
        &*INSTANCE
    }
    fn is_initialized(&self) -> bool {
        true
    }
    fn merge_from(
        &mut self,
        _is: &mut ::protobuf::CodedInputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
    }
    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        unimplemented!();
    }
    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        unimplemented!();
    }
    fn write_to_bytes(&self) -> ::protobuf::ProtobufResult<Vec<u8>> {
        let mut buf = Vec::new();
        if let Err(_) = ::prost::Message::encode(self, &mut buf) {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(buf)
    }
    fn merge_from_bytes(&mut self, bytes: &[u8]) -> ::protobuf::ProtobufResult<()> {
        if let Err(_) = ::prost::Message::merge(self, bytes) {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(())
    }
}
impl MetricFamily {
    pub fn new_() -> MetricFamily {
        ::std::default::Default::default()
    }
    #[inline]
    pub fn has_name(&self) -> bool {
        self.name.is_some()
    }
    #[inline]
    pub fn clear_name(&mut self) {
        self.name = ::std::option::Option::None
    }
    #[inline]
    pub fn set_name(&mut self, v: std::string::String) {
        self.name = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_name(&self) -> &str {
        match self.name.as_ref() {
            Some(v) => v,
            None => "",
        }
    }
    #[inline]
    pub fn mut_name(&mut self) -> &mut std::string::String {
        if self.name.is_none() {
            self.name = ::std::option::Option::Some(std::string::String::default());
        }
        self.name.as_mut().unwrap()
    }
    #[inline]
    pub fn take_name(&mut self) -> std::string::String {
        self.name.take().unwrap_or_else(::std::string::String::new)
    }
    #[inline]
    pub fn has_help(&self) -> bool {
        self.help.is_some()
    }
    #[inline]
    pub fn clear_help(&mut self) {
        self.help = ::std::option::Option::None
    }
    #[inline]
    pub fn set_help(&mut self, v: std::string::String) {
        self.help = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_help(&self) -> &str {
        match self.help.as_ref() {
            Some(v) => v,
            None => "",
        }
    }
    #[inline]
    pub fn mut_help(&mut self) -> &mut std::string::String {
        if self.help.is_none() {
            self.help = ::std::option::Option::Some(std::string::String::default());
        }
        self.help.as_mut().unwrap()
    }
    #[inline]
    pub fn take_help(&mut self) -> std::string::String {
        self.help.take().unwrap_or_else(::std::string::String::new)
    }
    #[inline]
    pub fn has_field_type(&self) -> bool {
        self.r#type.is_some()
    }
    #[inline]
    pub fn clear_field_type(&mut self) {
        self.r#type = ::std::option::Option::None
    }
    #[inline]
    pub fn set_field_type_(&mut self, v: MetricType) {
        self.r#type =
            ::std::option::Option::Some(unsafe { ::std::mem::transmute::<MetricType, i32>(v) });
    }
    #[inline]
    pub fn get_field_type(&self) -> MetricType {
        unsafe {
            ::std::mem::transmute::<i32, MetricType>(match self.r#type {
                Some(v) => v,
                None => 0,
            })
        }
    }
    #[inline]
    pub fn clear_metric(&mut self) {
        self.metric.clear();
    }
    #[inline]
    pub fn set_metric(&mut self, v: ::std::vec::Vec<Metric>) {
        self.metric = v;
    }
    #[inline]
    pub fn get_metric(&self) -> &::std::vec::Vec<Metric> {
        &self.metric
    }
    #[inline]
    pub fn mut_metric(&mut self) -> &mut ::std::vec::Vec<Metric> {
        &mut self.metric
    }
    #[inline]
    pub fn take_metric(&mut self) -> ::std::vec::Vec<Metric> {
        ::std::mem::replace(&mut self.metric, ::std::vec::Vec::new())
    }
}
impl ::protobuf::Clear for MetricFamily {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for MetricFamily {
    fn compute_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn get_cached_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn new() -> Self {
        Self::new_()
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
    }
    fn default_instance() -> &'static MetricFamily {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: MetricFamily = MetricFamily::new_();
        }
        &*INSTANCE
    }
    fn is_initialized(&self) -> bool {
        true
    }
    fn merge_from(
        &mut self,
        _is: &mut ::protobuf::CodedInputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
    }
    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        unimplemented!();
    }
    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        unimplemented!();
    }
    fn write_to_bytes(&self) -> ::protobuf::ProtobufResult<Vec<u8>> {
        let mut buf = Vec::new();
        if let Err(_) = ::prost::Message::encode(self, &mut buf) {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(buf)
    }
    fn merge_from_bytes(&mut self, bytes: &[u8]) -> ::protobuf::ProtobufResult<()> {
        if let Err(_) = ::prost::Message::merge(self, bytes) {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(())
    }
}
impl MetricType {
    pub fn values() -> &'static [Self] {
        static VALUES: &'static [MetricType] = &[
            MetricType::Counter,
            MetricType::Gauge,
            MetricType::Summary,
            MetricType::Untyped,
            MetricType::Histogram,
        ];
        VALUES
    }
}
