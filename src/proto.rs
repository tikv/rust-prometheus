// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(PartialEq,Clone,Default)]
pub struct LabelPair {
    // message fields
    name: ::protobuf::SingularField<::std::string::String>,
    value: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for LabelPair {}

impl LabelPair {
    pub fn new() -> LabelPair {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static LabelPair {
        static mut instance: ::protobuf::lazy::Lazy<LabelPair> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const LabelPair,
        };
        unsafe {
            instance.get(LabelPair::new)
        }
    }

    // optional string name = 1;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    pub fn has_name(&self) -> bool {
        self.name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        if self.name.is_none() {
            self.name.set_default();
        };
        self.name.as_mut().unwrap()
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        self.name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_name(&self) -> &str {
        match self.name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.name
    }

    fn mut_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.name
    }

    // optional string value = 2;

    pub fn clear_value(&mut self) {
        self.value.clear();
    }

    pub fn has_value(&self) -> bool {
        self.value.is_some()
    }

    // Param is passed by value, moved
    pub fn set_value(&mut self, v: ::std::string::String) {
        self.value = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_value(&mut self) -> &mut ::std::string::String {
        if self.value.is_none() {
            self.value.set_default();
        };
        self.value.as_mut().unwrap()
    }

    // Take field
    pub fn take_value(&mut self) -> ::std::string::String {
        self.value.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_value(&self) -> &str {
        match self.value.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_value_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.value
    }

    fn mut_value_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.value
    }
}

impl ::protobuf::Message for LabelPair {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.name)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.value)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.name.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        };
        if let Some(v) = self.value.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.name.as_ref() {
            os.write_string(1, &v)?;
        };
        if let Some(v) = self.value.as_ref() {
            os.write_string(2, &v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for LabelPair {
    fn new() -> LabelPair {
        LabelPair::new()
    }

    fn descriptor_static(_: ::std::option::Option<LabelPair>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    LabelPair::get_name_for_reflect,
                    LabelPair::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "value",
                    LabelPair::get_value_for_reflect,
                    LabelPair::mut_value_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<LabelPair>(
                    "LabelPair",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for LabelPair {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_value();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for LabelPair {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for LabelPair {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Gauge {
    // message fields
    value: ::std::option::Option<f64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Gauge {}

impl Gauge {
    pub fn new() -> Gauge {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Gauge {
        static mut instance: ::protobuf::lazy::Lazy<Gauge> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Gauge,
        };
        unsafe {
            instance.get(Gauge::new)
        }
    }

    // optional double value = 1;

    pub fn clear_value(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_value(&self) -> bool {
        self.value.is_some()
    }

    // Param is passed by value, moved
    pub fn set_value(&mut self, v: f64) {
        self.value = ::std::option::Option::Some(v);
    }

    pub fn get_value(&self) -> f64 {
        self.value.unwrap_or(0.)
    }

    fn get_value_for_reflect(&self) -> &::std::option::Option<f64> {
        &self.value
    }

    fn mut_value_for_reflect(&mut self) -> &mut ::std::option::Option<f64> {
        &mut self.value
    }
}

impl ::protobuf::Message for Gauge {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_double()?;
                    self.value = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.value {
            my_size += 9;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.value {
            os.write_double(1, v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Gauge {
    fn new() -> Gauge {
        Gauge::new()
    }

    fn descriptor_static(_: ::std::option::Option<Gauge>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                    "value",
                    Gauge::get_value_for_reflect,
                    Gauge::mut_value_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Gauge>(
                    "Gauge",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Gauge {
    fn clear(&mut self) {
        self.clear_value();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Gauge {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Gauge {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Counter {
    // message fields
    value: ::std::option::Option<f64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Counter {}

impl Counter {
    pub fn new() -> Counter {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Counter {
        static mut instance: ::protobuf::lazy::Lazy<Counter> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Counter,
        };
        unsafe {
            instance.get(Counter::new)
        }
    }

    // optional double value = 1;

    pub fn clear_value(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_value(&self) -> bool {
        self.value.is_some()
    }

    // Param is passed by value, moved
    pub fn set_value(&mut self, v: f64) {
        self.value = ::std::option::Option::Some(v);
    }

    pub fn get_value(&self) -> f64 {
        self.value.unwrap_or(0.)
    }

    fn get_value_for_reflect(&self) -> &::std::option::Option<f64> {
        &self.value
    }

    fn mut_value_for_reflect(&mut self) -> &mut ::std::option::Option<f64> {
        &mut self.value
    }
}

impl ::protobuf::Message for Counter {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_double()?;
                    self.value = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.value {
            my_size += 9;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.value {
            os.write_double(1, v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Counter {
    fn new() -> Counter {
        Counter::new()
    }

    fn descriptor_static(_: ::std::option::Option<Counter>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                    "value",
                    Counter::get_value_for_reflect,
                    Counter::mut_value_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Counter>(
                    "Counter",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Counter {
    fn clear(&mut self) {
        self.clear_value();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Counter {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Counter {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Quantile {
    // message fields
    quantile: ::std::option::Option<f64>,
    value: ::std::option::Option<f64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Quantile {}

impl Quantile {
    pub fn new() -> Quantile {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Quantile {
        static mut instance: ::protobuf::lazy::Lazy<Quantile> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Quantile,
        };
        unsafe {
            instance.get(Quantile::new)
        }
    }

    // optional double quantile = 1;

    pub fn clear_quantile(&mut self) {
        self.quantile = ::std::option::Option::None;
    }

    pub fn has_quantile(&self) -> bool {
        self.quantile.is_some()
    }

    // Param is passed by value, moved
    pub fn set_quantile(&mut self, v: f64) {
        self.quantile = ::std::option::Option::Some(v);
    }

    pub fn get_quantile(&self) -> f64 {
        self.quantile.unwrap_or(0.)
    }

    fn get_quantile_for_reflect(&self) -> &::std::option::Option<f64> {
        &self.quantile
    }

    fn mut_quantile_for_reflect(&mut self) -> &mut ::std::option::Option<f64> {
        &mut self.quantile
    }

    // optional double value = 2;

    pub fn clear_value(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_value(&self) -> bool {
        self.value.is_some()
    }

    // Param is passed by value, moved
    pub fn set_value(&mut self, v: f64) {
        self.value = ::std::option::Option::Some(v);
    }

    pub fn get_value(&self) -> f64 {
        self.value.unwrap_or(0.)
    }

    fn get_value_for_reflect(&self) -> &::std::option::Option<f64> {
        &self.value
    }

    fn mut_value_for_reflect(&mut self) -> &mut ::std::option::Option<f64> {
        &mut self.value
    }
}

impl ::protobuf::Message for Quantile {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_double()?;
                    self.quantile = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_double()?;
                    self.value = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.quantile {
            my_size += 9;
        };
        if let Some(v) = self.value {
            my_size += 9;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.quantile {
            os.write_double(1, v)?;
        };
        if let Some(v) = self.value {
            os.write_double(2, v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Quantile {
    fn new() -> Quantile {
        Quantile::new()
    }

    fn descriptor_static(_: ::std::option::Option<Quantile>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                    "quantile",
                    Quantile::get_quantile_for_reflect,
                    Quantile::mut_quantile_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                    "value",
                    Quantile::get_value_for_reflect,
                    Quantile::mut_value_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Quantile>(
                    "Quantile",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Quantile {
    fn clear(&mut self) {
        self.clear_quantile();
        self.clear_value();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Quantile {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Quantile {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Summary {
    // message fields
    sample_count: ::std::option::Option<u64>,
    sample_sum: ::std::option::Option<f64>,
    quantile: ::protobuf::RepeatedField<Quantile>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Summary {}

impl Summary {
    pub fn new() -> Summary {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Summary {
        static mut instance: ::protobuf::lazy::Lazy<Summary> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Summary,
        };
        unsafe {
            instance.get(Summary::new)
        }
    }

    // optional uint64 sample_count = 1;

    pub fn clear_sample_count(&mut self) {
        self.sample_count = ::std::option::Option::None;
    }

    pub fn has_sample_count(&self) -> bool {
        self.sample_count.is_some()
    }

    // Param is passed by value, moved
    pub fn set_sample_count(&mut self, v: u64) {
        self.sample_count = ::std::option::Option::Some(v);
    }

    pub fn get_sample_count(&self) -> u64 {
        self.sample_count.unwrap_or(0)
    }

    fn get_sample_count_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.sample_count
    }

    fn mut_sample_count_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.sample_count
    }

    // optional double sample_sum = 2;

    pub fn clear_sample_sum(&mut self) {
        self.sample_sum = ::std::option::Option::None;
    }

    pub fn has_sample_sum(&self) -> bool {
        self.sample_sum.is_some()
    }

    // Param is passed by value, moved
    pub fn set_sample_sum(&mut self, v: f64) {
        self.sample_sum = ::std::option::Option::Some(v);
    }

    pub fn get_sample_sum(&self) -> f64 {
        self.sample_sum.unwrap_or(0.)
    }

    fn get_sample_sum_for_reflect(&self) -> &::std::option::Option<f64> {
        &self.sample_sum
    }

    fn mut_sample_sum_for_reflect(&mut self) -> &mut ::std::option::Option<f64> {
        &mut self.sample_sum
    }

    // repeated .io.prometheus.client.Quantile quantile = 3;

    pub fn clear_quantile(&mut self) {
        self.quantile.clear();
    }

    // Param is passed by value, moved
    pub fn set_quantile(&mut self, v: ::protobuf::RepeatedField<Quantile>) {
        self.quantile = v;
    }

    // Mutable pointer to the field.
    pub fn mut_quantile(&mut self) -> &mut ::protobuf::RepeatedField<Quantile> {
        &mut self.quantile
    }

    // Take field
    pub fn take_quantile(&mut self) -> ::protobuf::RepeatedField<Quantile> {
        ::std::mem::replace(&mut self.quantile, ::protobuf::RepeatedField::new())
    }

    pub fn get_quantile(&self) -> &[Quantile] {
        &self.quantile
    }

    fn get_quantile_for_reflect(&self) -> &::protobuf::RepeatedField<Quantile> {
        &self.quantile
    }

    fn mut_quantile_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<Quantile> {
        &mut self.quantile
    }
}

impl ::protobuf::Message for Summary {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.sample_count = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_double()?;
                    self.sample_sum = ::std::option::Option::Some(tmp);
                },
                3 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.quantile)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.sample_count {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.sample_sum {
            my_size += 9;
        };
        for value in &self.quantile {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.sample_count {
            os.write_uint64(1, v)?;
        };
        if let Some(v) = self.sample_sum {
            os.write_double(2, v)?;
        };
        for v in &self.quantile {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Summary {
    fn new() -> Summary {
        Summary::new()
    }

    fn descriptor_static(_: ::std::option::Option<Summary>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "sample_count",
                    Summary::get_sample_count_for_reflect,
                    Summary::mut_sample_count_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                    "sample_sum",
                    Summary::get_sample_sum_for_reflect,
                    Summary::mut_sample_sum_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Quantile>>(
                    "quantile",
                    Summary::get_quantile_for_reflect,
                    Summary::mut_quantile_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Summary>(
                    "Summary",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Summary {
    fn clear(&mut self) {
        self.clear_sample_count();
        self.clear_sample_sum();
        self.clear_quantile();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Summary {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Summary {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Untyped {
    // message fields
    value: ::std::option::Option<f64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Untyped {}

impl Untyped {
    pub fn new() -> Untyped {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Untyped {
        static mut instance: ::protobuf::lazy::Lazy<Untyped> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Untyped,
        };
        unsafe {
            instance.get(Untyped::new)
        }
    }

    // optional double value = 1;

    pub fn clear_value(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_value(&self) -> bool {
        self.value.is_some()
    }

    // Param is passed by value, moved
    pub fn set_value(&mut self, v: f64) {
        self.value = ::std::option::Option::Some(v);
    }

    pub fn get_value(&self) -> f64 {
        self.value.unwrap_or(0.)
    }

    fn get_value_for_reflect(&self) -> &::std::option::Option<f64> {
        &self.value
    }

    fn mut_value_for_reflect(&mut self) -> &mut ::std::option::Option<f64> {
        &mut self.value
    }
}

impl ::protobuf::Message for Untyped {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_double()?;
                    self.value = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.value {
            my_size += 9;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.value {
            os.write_double(1, v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Untyped {
    fn new() -> Untyped {
        Untyped::new()
    }

    fn descriptor_static(_: ::std::option::Option<Untyped>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                    "value",
                    Untyped::get_value_for_reflect,
                    Untyped::mut_value_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Untyped>(
                    "Untyped",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Untyped {
    fn clear(&mut self) {
        self.clear_value();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Untyped {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Untyped {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Histogram {
    // message fields
    sample_count: ::std::option::Option<u64>,
    sample_sum: ::std::option::Option<f64>,
    bucket: ::protobuf::RepeatedField<Bucket>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Histogram {}

impl Histogram {
    pub fn new() -> Histogram {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Histogram {
        static mut instance: ::protobuf::lazy::Lazy<Histogram> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Histogram,
        };
        unsafe {
            instance.get(Histogram::new)
        }
    }

    // optional uint64 sample_count = 1;

    pub fn clear_sample_count(&mut self) {
        self.sample_count = ::std::option::Option::None;
    }

    pub fn has_sample_count(&self) -> bool {
        self.sample_count.is_some()
    }

    // Param is passed by value, moved
    pub fn set_sample_count(&mut self, v: u64) {
        self.sample_count = ::std::option::Option::Some(v);
    }

    pub fn get_sample_count(&self) -> u64 {
        self.sample_count.unwrap_or(0)
    }

    fn get_sample_count_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.sample_count
    }

    fn mut_sample_count_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.sample_count
    }

    // optional double sample_sum = 2;

    pub fn clear_sample_sum(&mut self) {
        self.sample_sum = ::std::option::Option::None;
    }

    pub fn has_sample_sum(&self) -> bool {
        self.sample_sum.is_some()
    }

    // Param is passed by value, moved
    pub fn set_sample_sum(&mut self, v: f64) {
        self.sample_sum = ::std::option::Option::Some(v);
    }

    pub fn get_sample_sum(&self) -> f64 {
        self.sample_sum.unwrap_or(0.)
    }

    fn get_sample_sum_for_reflect(&self) -> &::std::option::Option<f64> {
        &self.sample_sum
    }

    fn mut_sample_sum_for_reflect(&mut self) -> &mut ::std::option::Option<f64> {
        &mut self.sample_sum
    }

    // repeated .io.prometheus.client.Bucket bucket = 3;

    pub fn clear_bucket(&mut self) {
        self.bucket.clear();
    }

    // Param is passed by value, moved
    pub fn set_bucket(&mut self, v: ::protobuf::RepeatedField<Bucket>) {
        self.bucket = v;
    }

    // Mutable pointer to the field.
    pub fn mut_bucket(&mut self) -> &mut ::protobuf::RepeatedField<Bucket> {
        &mut self.bucket
    }

    // Take field
    pub fn take_bucket(&mut self) -> ::protobuf::RepeatedField<Bucket> {
        ::std::mem::replace(&mut self.bucket, ::protobuf::RepeatedField::new())
    }

    pub fn get_bucket(&self) -> &[Bucket] {
        &self.bucket
    }

    fn get_bucket_for_reflect(&self) -> &::protobuf::RepeatedField<Bucket> {
        &self.bucket
    }

    fn mut_bucket_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<Bucket> {
        &mut self.bucket
    }
}

impl ::protobuf::Message for Histogram {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.sample_count = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_double()?;
                    self.sample_sum = ::std::option::Option::Some(tmp);
                },
                3 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.bucket)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.sample_count {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.sample_sum {
            my_size += 9;
        };
        for value in &self.bucket {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.sample_count {
            os.write_uint64(1, v)?;
        };
        if let Some(v) = self.sample_sum {
            os.write_double(2, v)?;
        };
        for v in &self.bucket {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Histogram {
    fn new() -> Histogram {
        Histogram::new()
    }

    fn descriptor_static(_: ::std::option::Option<Histogram>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "sample_count",
                    Histogram::get_sample_count_for_reflect,
                    Histogram::mut_sample_count_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                    "sample_sum",
                    Histogram::get_sample_sum_for_reflect,
                    Histogram::mut_sample_sum_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Bucket>>(
                    "bucket",
                    Histogram::get_bucket_for_reflect,
                    Histogram::mut_bucket_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Histogram>(
                    "Histogram",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Histogram {
    fn clear(&mut self) {
        self.clear_sample_count();
        self.clear_sample_sum();
        self.clear_bucket();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Histogram {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Histogram {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Bucket {
    // message fields
    cumulative_count: ::std::option::Option<u64>,
    upper_bound: ::std::option::Option<f64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Bucket {}

impl Bucket {
    pub fn new() -> Bucket {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Bucket {
        static mut instance: ::protobuf::lazy::Lazy<Bucket> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Bucket,
        };
        unsafe {
            instance.get(Bucket::new)
        }
    }

    // optional uint64 cumulative_count = 1;

    pub fn clear_cumulative_count(&mut self) {
        self.cumulative_count = ::std::option::Option::None;
    }

    pub fn has_cumulative_count(&self) -> bool {
        self.cumulative_count.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cumulative_count(&mut self, v: u64) {
        self.cumulative_count = ::std::option::Option::Some(v);
    }

    pub fn get_cumulative_count(&self) -> u64 {
        self.cumulative_count.unwrap_or(0)
    }

    fn get_cumulative_count_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.cumulative_count
    }

    fn mut_cumulative_count_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.cumulative_count
    }

    // optional double upper_bound = 2;

    pub fn clear_upper_bound(&mut self) {
        self.upper_bound = ::std::option::Option::None;
    }

    pub fn has_upper_bound(&self) -> bool {
        self.upper_bound.is_some()
    }

    // Param is passed by value, moved
    pub fn set_upper_bound(&mut self, v: f64) {
        self.upper_bound = ::std::option::Option::Some(v);
    }

    pub fn get_upper_bound(&self) -> f64 {
        self.upper_bound.unwrap_or(0.)
    }

    fn get_upper_bound_for_reflect(&self) -> &::std::option::Option<f64> {
        &self.upper_bound
    }

    fn mut_upper_bound_for_reflect(&mut self) -> &mut ::std::option::Option<f64> {
        &mut self.upper_bound
    }
}

impl ::protobuf::Message for Bucket {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.cumulative_count = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_double()?;
                    self.upper_bound = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.cumulative_count {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.upper_bound {
            my_size += 9;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.cumulative_count {
            os.write_uint64(1, v)?;
        };
        if let Some(v) = self.upper_bound {
            os.write_double(2, v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Bucket {
    fn new() -> Bucket {
        Bucket::new()
    }

    fn descriptor_static(_: ::std::option::Option<Bucket>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "cumulative_count",
                    Bucket::get_cumulative_count_for_reflect,
                    Bucket::mut_cumulative_count_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                    "upper_bound",
                    Bucket::get_upper_bound_for_reflect,
                    Bucket::mut_upper_bound_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Bucket>(
                    "Bucket",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Bucket {
    fn clear(&mut self) {
        self.clear_cumulative_count();
        self.clear_upper_bound();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Bucket {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Bucket {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Metric {
    // message fields
    label: ::protobuf::RepeatedField<LabelPair>,
    gauge: ::protobuf::SingularPtrField<Gauge>,
    counter: ::protobuf::SingularPtrField<Counter>,
    summary: ::protobuf::SingularPtrField<Summary>,
    untyped: ::protobuf::SingularPtrField<Untyped>,
    histogram: ::protobuf::SingularPtrField<Histogram>,
    timestamp_ms: ::std::option::Option<i64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Metric {}

impl Metric {
    pub fn new() -> Metric {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Metric {
        static mut instance: ::protobuf::lazy::Lazy<Metric> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Metric,
        };
        unsafe {
            instance.get(Metric::new)
        }
    }

    // repeated .io.prometheus.client.LabelPair label = 1;

    pub fn clear_label(&mut self) {
        self.label.clear();
    }

    // Param is passed by value, moved
    pub fn set_label(&mut self, v: ::protobuf::RepeatedField<LabelPair>) {
        self.label = v;
    }

    // Mutable pointer to the field.
    pub fn mut_label(&mut self) -> &mut ::protobuf::RepeatedField<LabelPair> {
        &mut self.label
    }

    // Take field
    pub fn take_label(&mut self) -> ::protobuf::RepeatedField<LabelPair> {
        ::std::mem::replace(&mut self.label, ::protobuf::RepeatedField::new())
    }

    pub fn get_label(&self) -> &[LabelPair] {
        &self.label
    }

    fn get_label_for_reflect(&self) -> &::protobuf::RepeatedField<LabelPair> {
        &self.label
    }

    fn mut_label_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<LabelPair> {
        &mut self.label
    }

    // optional .io.prometheus.client.Gauge gauge = 2;

    pub fn clear_gauge(&mut self) {
        self.gauge.clear();
    }

    pub fn has_gauge(&self) -> bool {
        self.gauge.is_some()
    }

    // Param is passed by value, moved
    pub fn set_gauge(&mut self, v: Gauge) {
        self.gauge = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_gauge(&mut self) -> &mut Gauge {
        if self.gauge.is_none() {
            self.gauge.set_default();
        };
        self.gauge.as_mut().unwrap()
    }

    // Take field
    pub fn take_gauge(&mut self) -> Gauge {
        self.gauge.take().unwrap_or_else(|| Gauge::new())
    }

    pub fn get_gauge(&self) -> &Gauge {
        self.gauge.as_ref().unwrap_or_else(|| Gauge::default_instance())
    }

    fn get_gauge_for_reflect(&self) -> &::protobuf::SingularPtrField<Gauge> {
        &self.gauge
    }

    fn mut_gauge_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Gauge> {
        &mut self.gauge
    }

    // optional .io.prometheus.client.Counter counter = 3;

    pub fn clear_counter(&mut self) {
        self.counter.clear();
    }

    pub fn has_counter(&self) -> bool {
        self.counter.is_some()
    }

    // Param is passed by value, moved
    pub fn set_counter(&mut self, v: Counter) {
        self.counter = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_counter(&mut self) -> &mut Counter {
        if self.counter.is_none() {
            self.counter.set_default();
        };
        self.counter.as_mut().unwrap()
    }

    // Take field
    pub fn take_counter(&mut self) -> Counter {
        self.counter.take().unwrap_or_else(|| Counter::new())
    }

    pub fn get_counter(&self) -> &Counter {
        self.counter.as_ref().unwrap_or_else(|| Counter::default_instance())
    }

    fn get_counter_for_reflect(&self) -> &::protobuf::SingularPtrField<Counter> {
        &self.counter
    }

    fn mut_counter_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Counter> {
        &mut self.counter
    }

    // optional .io.prometheus.client.Summary summary = 4;

    pub fn clear_summary(&mut self) {
        self.summary.clear();
    }

    pub fn has_summary(&self) -> bool {
        self.summary.is_some()
    }

    // Param is passed by value, moved
    pub fn set_summary(&mut self, v: Summary) {
        self.summary = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_summary(&mut self) -> &mut Summary {
        if self.summary.is_none() {
            self.summary.set_default();
        };
        self.summary.as_mut().unwrap()
    }

    // Take field
    pub fn take_summary(&mut self) -> Summary {
        self.summary.take().unwrap_or_else(|| Summary::new())
    }

    pub fn get_summary(&self) -> &Summary {
        self.summary.as_ref().unwrap_or_else(|| Summary::default_instance())
    }

    fn get_summary_for_reflect(&self) -> &::protobuf::SingularPtrField<Summary> {
        &self.summary
    }

    fn mut_summary_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Summary> {
        &mut self.summary
    }

    // optional .io.prometheus.client.Untyped untyped = 5;

    pub fn clear_untyped(&mut self) {
        self.untyped.clear();
    }

    pub fn has_untyped(&self) -> bool {
        self.untyped.is_some()
    }

    // Param is passed by value, moved
    pub fn set_untyped(&mut self, v: Untyped) {
        self.untyped = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_untyped(&mut self) -> &mut Untyped {
        if self.untyped.is_none() {
            self.untyped.set_default();
        };
        self.untyped.as_mut().unwrap()
    }

    // Take field
    pub fn take_untyped(&mut self) -> Untyped {
        self.untyped.take().unwrap_or_else(|| Untyped::new())
    }

    pub fn get_untyped(&self) -> &Untyped {
        self.untyped.as_ref().unwrap_or_else(|| Untyped::default_instance())
    }

    fn get_untyped_for_reflect(&self) -> &::protobuf::SingularPtrField<Untyped> {
        &self.untyped
    }

    fn mut_untyped_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Untyped> {
        &mut self.untyped
    }

    // optional .io.prometheus.client.Histogram histogram = 7;

    pub fn clear_histogram(&mut self) {
        self.histogram.clear();
    }

    pub fn has_histogram(&self) -> bool {
        self.histogram.is_some()
    }

    // Param is passed by value, moved
    pub fn set_histogram(&mut self, v: Histogram) {
        self.histogram = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_histogram(&mut self) -> &mut Histogram {
        if self.histogram.is_none() {
            self.histogram.set_default();
        };
        self.histogram.as_mut().unwrap()
    }

    // Take field
    pub fn take_histogram(&mut self) -> Histogram {
        self.histogram.take().unwrap_or_else(|| Histogram::new())
    }

    pub fn get_histogram(&self) -> &Histogram {
        self.histogram.as_ref().unwrap_or_else(|| Histogram::default_instance())
    }

    fn get_histogram_for_reflect(&self) -> &::protobuf::SingularPtrField<Histogram> {
        &self.histogram
    }

    fn mut_histogram_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Histogram> {
        &mut self.histogram
    }

    // optional int64 timestamp_ms = 6;

    pub fn clear_timestamp_ms(&mut self) {
        self.timestamp_ms = ::std::option::Option::None;
    }

    pub fn has_timestamp_ms(&self) -> bool {
        self.timestamp_ms.is_some()
    }

    // Param is passed by value, moved
    pub fn set_timestamp_ms(&mut self, v: i64) {
        self.timestamp_ms = ::std::option::Option::Some(v);
    }

    pub fn get_timestamp_ms(&self) -> i64 {
        self.timestamp_ms.unwrap_or(0)
    }

    fn get_timestamp_ms_for_reflect(&self) -> &::std::option::Option<i64> {
        &self.timestamp_ms
    }

    fn mut_timestamp_ms_for_reflect(&mut self) -> &mut ::std::option::Option<i64> {
        &mut self.timestamp_ms
    }
}

impl ::protobuf::Message for Metric {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.label)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.gauge)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.counter)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.summary)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.untyped)?;
                },
                7 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.histogram)?;
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int64()?;
                    self.timestamp_ms = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.label {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.gauge.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.counter.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.summary.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.untyped.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.histogram.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.timestamp_ms {
            my_size += ::protobuf::rt::value_size(6, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.label {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.gauge.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.counter.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.summary.as_ref() {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.untyped.as_ref() {
            os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.histogram.as_ref() {
            os.write_tag(7, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.timestamp_ms {
            os.write_int64(6, v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Metric {
    fn new() -> Metric {
        Metric::new()
    }

    fn descriptor_static(_: ::std::option::Option<Metric>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<LabelPair>>(
                    "label",
                    Metric::get_label_for_reflect,
                    Metric::mut_label_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Gauge>>(
                    "gauge",
                    Metric::get_gauge_for_reflect,
                    Metric::mut_gauge_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Counter>>(
                    "counter",
                    Metric::get_counter_for_reflect,
                    Metric::mut_counter_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Summary>>(
                    "summary",
                    Metric::get_summary_for_reflect,
                    Metric::mut_summary_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Untyped>>(
                    "untyped",
                    Metric::get_untyped_for_reflect,
                    Metric::mut_untyped_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Histogram>>(
                    "histogram",
                    Metric::get_histogram_for_reflect,
                    Metric::mut_histogram_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "timestamp_ms",
                    Metric::get_timestamp_ms_for_reflect,
                    Metric::mut_timestamp_ms_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Metric>(
                    "Metric",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Metric {
    fn clear(&mut self) {
        self.clear_label();
        self.clear_gauge();
        self.clear_counter();
        self.clear_summary();
        self.clear_untyped();
        self.clear_histogram();
        self.clear_timestamp_ms();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Metric {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Metric {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct MetricFamily {
    // message fields
    name: ::protobuf::SingularField<::std::string::String>,
    help: ::protobuf::SingularField<::std::string::String>,
    field_type: ::std::option::Option<MetricType>,
    metric: ::protobuf::RepeatedField<Metric>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for MetricFamily {}

impl MetricFamily {
    pub fn new() -> MetricFamily {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static MetricFamily {
        static mut instance: ::protobuf::lazy::Lazy<MetricFamily> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const MetricFamily,
        };
        unsafe {
            instance.get(MetricFamily::new)
        }
    }

    // optional string name = 1;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    pub fn has_name(&self) -> bool {
        self.name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        if self.name.is_none() {
            self.name.set_default();
        };
        self.name.as_mut().unwrap()
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        self.name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_name(&self) -> &str {
        match self.name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.name
    }

    fn mut_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.name
    }

    // optional string help = 2;

    pub fn clear_help(&mut self) {
        self.help.clear();
    }

    pub fn has_help(&self) -> bool {
        self.help.is_some()
    }

    // Param is passed by value, moved
    pub fn set_help(&mut self, v: ::std::string::String) {
        self.help = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_help(&mut self) -> &mut ::std::string::String {
        if self.help.is_none() {
            self.help.set_default();
        };
        self.help.as_mut().unwrap()
    }

    // Take field
    pub fn take_help(&mut self) -> ::std::string::String {
        self.help.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_help(&self) -> &str {
        match self.help.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_help_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.help
    }

    fn mut_help_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.help
    }

    // optional .io.prometheus.client.MetricType type = 3;

    pub fn clear_field_type(&mut self) {
        self.field_type = ::std::option::Option::None;
    }

    pub fn has_field_type(&self) -> bool {
        self.field_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_field_type(&mut self, v: MetricType) {
        self.field_type = ::std::option::Option::Some(v);
    }

    pub fn get_field_type(&self) -> MetricType {
        self.field_type.unwrap_or(MetricType::COUNTER)
    }

    fn get_field_type_for_reflect(&self) -> &::std::option::Option<MetricType> {
        &self.field_type
    }

    fn mut_field_type_for_reflect(&mut self) -> &mut ::std::option::Option<MetricType> {
        &mut self.field_type
    }

    // repeated .io.prometheus.client.Metric metric = 4;

    pub fn clear_metric(&mut self) {
        self.metric.clear();
    }

    // Param is passed by value, moved
    pub fn set_metric(&mut self, v: ::protobuf::RepeatedField<Metric>) {
        self.metric = v;
    }

    // Mutable pointer to the field.
    pub fn mut_metric(&mut self) -> &mut ::protobuf::RepeatedField<Metric> {
        &mut self.metric
    }

    // Take field
    pub fn take_metric(&mut self) -> ::protobuf::RepeatedField<Metric> {
        ::std::mem::replace(&mut self.metric, ::protobuf::RepeatedField::new())
    }

    pub fn get_metric(&self) -> &[Metric] {
        &self.metric
    }

    fn get_metric_for_reflect(&self) -> &::protobuf::RepeatedField<Metric> {
        &self.metric
    }

    fn mut_metric_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<Metric> {
        &mut self.metric
    }
}

impl ::protobuf::Message for MetricFamily {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.name)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.help)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_enum()?;
                    self.field_type = ::std::option::Option::Some(tmp);
                },
                4 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.metric)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.name.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        };
        if let Some(v) = self.help.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        };
        if let Some(v) = self.field_type {
            my_size += ::protobuf::rt::enum_size(3, v);
        };
        for value in &self.metric {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.name.as_ref() {
            os.write_string(1, &v)?;
        };
        if let Some(v) = self.help.as_ref() {
            os.write_string(2, &v)?;
        };
        if let Some(v) = self.field_type {
            os.write_enum(3, v.value())?;
        };
        for v in &self.metric {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for MetricFamily {
    fn new() -> MetricFamily {
        MetricFamily::new()
    }

    fn descriptor_static(_: ::std::option::Option<MetricFamily>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    MetricFamily::get_name_for_reflect,
                    MetricFamily::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "help",
                    MetricFamily::get_help_for_reflect,
                    MetricFamily::mut_help_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<MetricType>>(
                    "type",
                    MetricFamily::get_field_type_for_reflect,
                    MetricFamily::mut_field_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Metric>>(
                    "metric",
                    MetricFamily::get_metric_for_reflect,
                    MetricFamily::mut_metric_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<MetricFamily>(
                    "MetricFamily",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for MetricFamily {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_help();
        self.clear_field_type();
        self.clear_metric();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for MetricFamily {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for MetricFamily {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum MetricType {
    COUNTER = 0,
    GAUGE = 1,
    SUMMARY = 2,
    UNTYPED = 3,
    HISTOGRAM = 4,
}

impl ::protobuf::ProtobufEnum for MetricType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<MetricType> {
        match value {
            0 => ::std::option::Option::Some(MetricType::COUNTER),
            1 => ::std::option::Option::Some(MetricType::GAUGE),
            2 => ::std::option::Option::Some(MetricType::SUMMARY),
            3 => ::std::option::Option::Some(MetricType::UNTYPED),
            4 => ::std::option::Option::Some(MetricType::HISTOGRAM),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [MetricType] = &[
            MetricType::COUNTER,
            MetricType::GAUGE,
            MetricType::SUMMARY,
            MetricType::UNTYPED,
            MetricType::HISTOGRAM,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<MetricType>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("MetricType", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for MetricType {
}

impl ::protobuf::reflect::ProtobufValue for MetricType {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x13, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2f, 0x6d, 0x65, 0x74, 0x72, 0x69, 0x63, 0x73, 0x2e,
    0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x14, 0x69, 0x6f, 0x2e, 0x70, 0x72, 0x6f, 0x6d, 0x65, 0x74,
    0x68, 0x65, 0x75, 0x73, 0x2e, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x22, 0x35, 0x0a, 0x09, 0x4c,
    0x61, 0x62, 0x65, 0x6c, 0x50, 0x61, 0x69, 0x72, 0x12, 0x12, 0x0a, 0x04, 0x6e, 0x61, 0x6d, 0x65,
    0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x12, 0x14, 0x0a, 0x05,
    0x76, 0x61, 0x6c, 0x75, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x05, 0x76, 0x61, 0x6c,
    0x75, 0x65, 0x22, 0x1d, 0x0a, 0x05, 0x47, 0x61, 0x75, 0x67, 0x65, 0x12, 0x14, 0x0a, 0x05, 0x76,
    0x61, 0x6c, 0x75, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x01, 0x52, 0x05, 0x76, 0x61, 0x6c, 0x75,
    0x65, 0x22, 0x1f, 0x0a, 0x07, 0x43, 0x6f, 0x75, 0x6e, 0x74, 0x65, 0x72, 0x12, 0x14, 0x0a, 0x05,
    0x76, 0x61, 0x6c, 0x75, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x01, 0x52, 0x05, 0x76, 0x61, 0x6c,
    0x75, 0x65, 0x22, 0x3c, 0x0a, 0x08, 0x51, 0x75, 0x61, 0x6e, 0x74, 0x69, 0x6c, 0x65, 0x12, 0x1a,
    0x0a, 0x08, 0x71, 0x75, 0x61, 0x6e, 0x74, 0x69, 0x6c, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x01,
    0x52, 0x08, 0x71, 0x75, 0x61, 0x6e, 0x74, 0x69, 0x6c, 0x65, 0x12, 0x14, 0x0a, 0x05, 0x76, 0x61,
    0x6c, 0x75, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x01, 0x52, 0x05, 0x76, 0x61, 0x6c, 0x75, 0x65,
    0x22, 0x87, 0x01, 0x0a, 0x07, 0x53, 0x75, 0x6d, 0x6d, 0x61, 0x72, 0x79, 0x12, 0x21, 0x0a, 0x0c,
    0x73, 0x61, 0x6d, 0x70, 0x6c, 0x65, 0x5f, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x18, 0x01, 0x20, 0x01,
    0x28, 0x04, 0x52, 0x0b, 0x73, 0x61, 0x6d, 0x70, 0x6c, 0x65, 0x43, 0x6f, 0x75, 0x6e, 0x74, 0x12,
    0x1d, 0x0a, 0x0a, 0x73, 0x61, 0x6d, 0x70, 0x6c, 0x65, 0x5f, 0x73, 0x75, 0x6d, 0x18, 0x02, 0x20,
    0x01, 0x28, 0x01, 0x52, 0x09, 0x73, 0x61, 0x6d, 0x70, 0x6c, 0x65, 0x53, 0x75, 0x6d, 0x12, 0x3a,
    0x0a, 0x08, 0x71, 0x75, 0x61, 0x6e, 0x74, 0x69, 0x6c, 0x65, 0x18, 0x03, 0x20, 0x03, 0x28, 0x0b,
    0x32, 0x1e, 0x2e, 0x69, 0x6f, 0x2e, 0x70, 0x72, 0x6f, 0x6d, 0x65, 0x74, 0x68, 0x65, 0x75, 0x73,
    0x2e, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x2e, 0x51, 0x75, 0x61, 0x6e, 0x74, 0x69, 0x6c, 0x65,
    0x52, 0x08, 0x71, 0x75, 0x61, 0x6e, 0x74, 0x69, 0x6c, 0x65, 0x22, 0x1f, 0x0a, 0x07, 0x55, 0x6e,
    0x74, 0x79, 0x70, 0x65, 0x64, 0x12, 0x14, 0x0a, 0x05, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x18, 0x01,
    0x20, 0x01, 0x28, 0x01, 0x52, 0x05, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x22, 0x83, 0x01, 0x0a, 0x09,
    0x48, 0x69, 0x73, 0x74, 0x6f, 0x67, 0x72, 0x61, 0x6d, 0x12, 0x21, 0x0a, 0x0c, 0x73, 0x61, 0x6d,
    0x70, 0x6c, 0x65, 0x5f, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x18, 0x01, 0x20, 0x01, 0x28, 0x04, 0x52,
    0x0b, 0x73, 0x61, 0x6d, 0x70, 0x6c, 0x65, 0x43, 0x6f, 0x75, 0x6e, 0x74, 0x12, 0x1d, 0x0a, 0x0a,
    0x73, 0x61, 0x6d, 0x70, 0x6c, 0x65, 0x5f, 0x73, 0x75, 0x6d, 0x18, 0x02, 0x20, 0x01, 0x28, 0x01,
    0x52, 0x09, 0x73, 0x61, 0x6d, 0x70, 0x6c, 0x65, 0x53, 0x75, 0x6d, 0x12, 0x34, 0x0a, 0x06, 0x62,
    0x75, 0x63, 0x6b, 0x65, 0x74, 0x18, 0x03, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x1c, 0x2e, 0x69, 0x6f,
    0x2e, 0x70, 0x72, 0x6f, 0x6d, 0x65, 0x74, 0x68, 0x65, 0x75, 0x73, 0x2e, 0x63, 0x6c, 0x69, 0x65,
    0x6e, 0x74, 0x2e, 0x42, 0x75, 0x63, 0x6b, 0x65, 0x74, 0x52, 0x06, 0x62, 0x75, 0x63, 0x6b, 0x65,
    0x74, 0x22, 0x54, 0x0a, 0x06, 0x42, 0x75, 0x63, 0x6b, 0x65, 0x74, 0x12, 0x29, 0x0a, 0x10, 0x63,
    0x75, 0x6d, 0x75, 0x6c, 0x61, 0x74, 0x69, 0x76, 0x65, 0x5f, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x18,
    0x01, 0x20, 0x01, 0x28, 0x04, 0x52, 0x0f, 0x63, 0x75, 0x6d, 0x75, 0x6c, 0x61, 0x74, 0x69, 0x76,
    0x65, 0x43, 0x6f, 0x75, 0x6e, 0x74, 0x12, 0x1f, 0x0a, 0x0b, 0x75, 0x70, 0x70, 0x65, 0x72, 0x5f,
    0x62, 0x6f, 0x75, 0x6e, 0x64, 0x18, 0x02, 0x20, 0x01, 0x28, 0x01, 0x52, 0x0a, 0x75, 0x70, 0x70,
    0x65, 0x72, 0x42, 0x6f, 0x75, 0x6e, 0x64, 0x22, 0xff, 0x02, 0x0a, 0x06, 0x4d, 0x65, 0x74, 0x72,
    0x69, 0x63, 0x12, 0x35, 0x0a, 0x05, 0x6c, 0x61, 0x62, 0x65, 0x6c, 0x18, 0x01, 0x20, 0x03, 0x28,
    0x0b, 0x32, 0x1f, 0x2e, 0x69, 0x6f, 0x2e, 0x70, 0x72, 0x6f, 0x6d, 0x65, 0x74, 0x68, 0x65, 0x75,
    0x73, 0x2e, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x2e, 0x4c, 0x61, 0x62, 0x65, 0x6c, 0x50, 0x61,
    0x69, 0x72, 0x52, 0x05, 0x6c, 0x61, 0x62, 0x65, 0x6c, 0x12, 0x31, 0x0a, 0x05, 0x67, 0x61, 0x75,
    0x67, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1b, 0x2e, 0x69, 0x6f, 0x2e, 0x70, 0x72,
    0x6f, 0x6d, 0x65, 0x74, 0x68, 0x65, 0x75, 0x73, 0x2e, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x2e,
    0x47, 0x61, 0x75, 0x67, 0x65, 0x52, 0x05, 0x67, 0x61, 0x75, 0x67, 0x65, 0x12, 0x37, 0x0a, 0x07,
    0x63, 0x6f, 0x75, 0x6e, 0x74, 0x65, 0x72, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1d, 0x2e,
    0x69, 0x6f, 0x2e, 0x70, 0x72, 0x6f, 0x6d, 0x65, 0x74, 0x68, 0x65, 0x75, 0x73, 0x2e, 0x63, 0x6c,
    0x69, 0x65, 0x6e, 0x74, 0x2e, 0x43, 0x6f, 0x75, 0x6e, 0x74, 0x65, 0x72, 0x52, 0x07, 0x63, 0x6f,
    0x75, 0x6e, 0x74, 0x65, 0x72, 0x12, 0x37, 0x0a, 0x07, 0x73, 0x75, 0x6d, 0x6d, 0x61, 0x72, 0x79,
    0x18, 0x04, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1d, 0x2e, 0x69, 0x6f, 0x2e, 0x70, 0x72, 0x6f, 0x6d,
    0x65, 0x74, 0x68, 0x65, 0x75, 0x73, 0x2e, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x2e, 0x53, 0x75,
    0x6d, 0x6d, 0x61, 0x72, 0x79, 0x52, 0x07, 0x73, 0x75, 0x6d, 0x6d, 0x61, 0x72, 0x79, 0x12, 0x37,
    0x0a, 0x07, 0x75, 0x6e, 0x74, 0x79, 0x70, 0x65, 0x64, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0b, 0x32,
    0x1d, 0x2e, 0x69, 0x6f, 0x2e, 0x70, 0x72, 0x6f, 0x6d, 0x65, 0x74, 0x68, 0x65, 0x75, 0x73, 0x2e,
    0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x2e, 0x55, 0x6e, 0x74, 0x79, 0x70, 0x65, 0x64, 0x52, 0x07,
    0x75, 0x6e, 0x74, 0x79, 0x70, 0x65, 0x64, 0x12, 0x3d, 0x0a, 0x09, 0x68, 0x69, 0x73, 0x74, 0x6f,
    0x67, 0x72, 0x61, 0x6d, 0x18, 0x07, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1f, 0x2e, 0x69, 0x6f, 0x2e,
    0x70, 0x72, 0x6f, 0x6d, 0x65, 0x74, 0x68, 0x65, 0x75, 0x73, 0x2e, 0x63, 0x6c, 0x69, 0x65, 0x6e,
    0x74, 0x2e, 0x48, 0x69, 0x73, 0x74, 0x6f, 0x67, 0x72, 0x61, 0x6d, 0x52, 0x09, 0x68, 0x69, 0x73,
    0x74, 0x6f, 0x67, 0x72, 0x61, 0x6d, 0x12, 0x21, 0x0a, 0x0c, 0x74, 0x69, 0x6d, 0x65, 0x73, 0x74,
    0x61, 0x6d, 0x70, 0x5f, 0x6d, 0x73, 0x18, 0x06, 0x20, 0x01, 0x28, 0x03, 0x52, 0x0b, 0x74, 0x69,
    0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d, 0x70, 0x4d, 0x73, 0x22, 0xa2, 0x01, 0x0a, 0x0c, 0x4d, 0x65,
    0x74, 0x72, 0x69, 0x63, 0x46, 0x61, 0x6d, 0x69, 0x6c, 0x79, 0x12, 0x12, 0x0a, 0x04, 0x6e, 0x61,
    0x6d, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x12, 0x12,
    0x0a, 0x04, 0x68, 0x65, 0x6c, 0x70, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x04, 0x68, 0x65,
    0x6c, 0x70, 0x12, 0x34, 0x0a, 0x04, 0x74, 0x79, 0x70, 0x65, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0e,
    0x32, 0x20, 0x2e, 0x69, 0x6f, 0x2e, 0x70, 0x72, 0x6f, 0x6d, 0x65, 0x74, 0x68, 0x65, 0x75, 0x73,
    0x2e, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x2e, 0x4d, 0x65, 0x74, 0x72, 0x69, 0x63, 0x54, 0x79,
    0x70, 0x65, 0x52, 0x04, 0x74, 0x79, 0x70, 0x65, 0x12, 0x34, 0x0a, 0x06, 0x6d, 0x65, 0x74, 0x72,
    0x69, 0x63, 0x18, 0x04, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x1c, 0x2e, 0x69, 0x6f, 0x2e, 0x70, 0x72,
    0x6f, 0x6d, 0x65, 0x74, 0x68, 0x65, 0x75, 0x73, 0x2e, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x2e,
    0x4d, 0x65, 0x74, 0x72, 0x69, 0x63, 0x52, 0x06, 0x6d, 0x65, 0x74, 0x72, 0x69, 0x63, 0x2a, 0x4d,
    0x0a, 0x0a, 0x4d, 0x65, 0x74, 0x72, 0x69, 0x63, 0x54, 0x79, 0x70, 0x65, 0x12, 0x0b, 0x0a, 0x07,
    0x43, 0x4f, 0x55, 0x4e, 0x54, 0x45, 0x52, 0x10, 0x00, 0x12, 0x09, 0x0a, 0x05, 0x47, 0x41, 0x55,
    0x47, 0x45, 0x10, 0x01, 0x12, 0x0b, 0x0a, 0x07, 0x53, 0x55, 0x4d, 0x4d, 0x41, 0x52, 0x59, 0x10,
    0x02, 0x12, 0x0b, 0x0a, 0x07, 0x55, 0x4e, 0x54, 0x59, 0x50, 0x45, 0x44, 0x10, 0x03, 0x12, 0x0d,
    0x0a, 0x09, 0x48, 0x49, 0x53, 0x54, 0x4f, 0x47, 0x52, 0x41, 0x4d, 0x10, 0x04, 0x42, 0x16, 0x0a,
    0x14, 0x69, 0x6f, 0x2e, 0x70, 0x72, 0x6f, 0x6d, 0x65, 0x74, 0x68, 0x65, 0x75, 0x73, 0x2e, 0x63,
    0x6c, 0x69, 0x65, 0x6e, 0x74, 0x4a, 0xf3, 0x17, 0x0a, 0x06, 0x12, 0x04, 0x0d, 0x00, 0x50, 0x01,
    0x0a, 0xbc, 0x04, 0x0a, 0x01, 0x0c, 0x12, 0x03, 0x0d, 0x00, 0x12, 0x32, 0xb1, 0x04, 0x20, 0x43,
    0x6f, 0x70, 0x79, 0x72, 0x69, 0x67, 0x68, 0x74, 0x20, 0x32, 0x30, 0x31, 0x33, 0x20, 0x50, 0x72,
    0x6f, 0x6d, 0x65, 0x74, 0x68, 0x65, 0x75, 0x73, 0x20, 0x54, 0x65, 0x61, 0x6d, 0x0a, 0x20, 0x4c,
    0x69, 0x63, 0x65, 0x6e, 0x73, 0x65, 0x64, 0x20, 0x75, 0x6e, 0x64, 0x65, 0x72, 0x20, 0x74, 0x68,
    0x65, 0x20, 0x41, 0x70, 0x61, 0x63, 0x68, 0x65, 0x20, 0x4c, 0x69, 0x63, 0x65, 0x6e, 0x73, 0x65,
    0x2c, 0x20, 0x56, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x20, 0x32, 0x2e, 0x30, 0x20, 0x28, 0x74,
    0x68, 0x65, 0x20, 0x22, 0x4c, 0x69, 0x63, 0x65, 0x6e, 0x73, 0x65, 0x22, 0x29, 0x3b, 0x0a, 0x20,
    0x79, 0x6f, 0x75, 0x20, 0x6d, 0x61, 0x79, 0x20, 0x6e, 0x6f, 0x74, 0x20, 0x75, 0x73, 0x65, 0x20,
    0x74, 0x68, 0x69, 0x73, 0x20, 0x66, 0x69, 0x6c, 0x65, 0x20, 0x65, 0x78, 0x63, 0x65, 0x70, 0x74,
    0x20, 0x69, 0x6e, 0x20, 0x63, 0x6f, 0x6d, 0x70, 0x6c, 0x69, 0x61, 0x6e, 0x63, 0x65, 0x20, 0x77,
    0x69, 0x74, 0x68, 0x20, 0x74, 0x68, 0x65, 0x20, 0x4c, 0x69, 0x63, 0x65, 0x6e, 0x73, 0x65, 0x2e,
    0x0a, 0x20, 0x59, 0x6f, 0x75, 0x20, 0x6d, 0x61, 0x79, 0x20, 0x6f, 0x62, 0x74, 0x61, 0x69, 0x6e,
    0x20, 0x61, 0x20, 0x63, 0x6f, 0x70, 0x79, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x4c,
    0x69, 0x63, 0x65, 0x6e, 0x73, 0x65, 0x20, 0x61, 0x74, 0x0a, 0x0a, 0x20, 0x68, 0x74, 0x74, 0x70,
    0x3a, 0x2f, 0x2f, 0x77, 0x77, 0x77, 0x2e, 0x61, 0x70, 0x61, 0x63, 0x68, 0x65, 0x2e, 0x6f, 0x72,
    0x67, 0x2f, 0x6c, 0x69, 0x63, 0x65, 0x6e, 0x73, 0x65, 0x73, 0x2f, 0x4c, 0x49, 0x43, 0x45, 0x4e,
    0x53, 0x45, 0x2d, 0x32, 0x2e, 0x30, 0x0a, 0x0a, 0x20, 0x55, 0x6e, 0x6c, 0x65, 0x73, 0x73, 0x20,
    0x72, 0x65, 0x71, 0x75, 0x69, 0x72, 0x65, 0x64, 0x20, 0x62, 0x79, 0x20, 0x61, 0x70, 0x70, 0x6c,
    0x69, 0x63, 0x61, 0x62, 0x6c, 0x65, 0x20, 0x6c, 0x61, 0x77, 0x20, 0x6f, 0x72, 0x20, 0x61, 0x67,
    0x72, 0x65, 0x65, 0x64, 0x20, 0x74, 0x6f, 0x20, 0x69, 0x6e, 0x20, 0x77, 0x72, 0x69, 0x74, 0x69,
    0x6e, 0x67, 0x2c, 0x20, 0x73, 0x6f, 0x66, 0x74, 0x77, 0x61, 0x72, 0x65, 0x0a, 0x20, 0x64, 0x69,
    0x73, 0x74, 0x72, 0x69, 0x62, 0x75, 0x74, 0x65, 0x64, 0x20, 0x75, 0x6e, 0x64, 0x65, 0x72, 0x20,
    0x74, 0x68, 0x65, 0x20, 0x4c, 0x69, 0x63, 0x65, 0x6e, 0x73, 0x65, 0x20, 0x69, 0x73, 0x20, 0x64,
    0x69, 0x73, 0x74, 0x72, 0x69, 0x62, 0x75, 0x74, 0x65, 0x64, 0x20, 0x6f, 0x6e, 0x20, 0x61, 0x6e,
    0x20, 0x22, 0x41, 0x53, 0x20, 0x49, 0x53, 0x22, 0x20, 0x42, 0x41, 0x53, 0x49, 0x53, 0x2c, 0x0a,
    0x20, 0x57, 0x49, 0x54, 0x48, 0x4f, 0x55, 0x54, 0x20, 0x57, 0x41, 0x52, 0x52, 0x41, 0x4e, 0x54,
    0x49, 0x45, 0x53, 0x20, 0x4f, 0x52, 0x20, 0x43, 0x4f, 0x4e, 0x44, 0x49, 0x54, 0x49, 0x4f, 0x4e,
    0x53, 0x20, 0x4f, 0x46, 0x20, 0x41, 0x4e, 0x59, 0x20, 0x4b, 0x49, 0x4e, 0x44, 0x2c, 0x20, 0x65,
    0x69, 0x74, 0x68, 0x65, 0x72, 0x20, 0x65, 0x78, 0x70, 0x72, 0x65, 0x73, 0x73, 0x20, 0x6f, 0x72,
    0x20, 0x69, 0x6d, 0x70, 0x6c, 0x69, 0x65, 0x64, 0x2e, 0x0a, 0x20, 0x53, 0x65, 0x65, 0x20, 0x74,
    0x68, 0x65, 0x20, 0x4c, 0x69, 0x63, 0x65, 0x6e, 0x73, 0x65, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x74,
    0x68, 0x65, 0x20, 0x73, 0x70, 0x65, 0x63, 0x69, 0x66, 0x69, 0x63, 0x20, 0x6c, 0x61, 0x6e, 0x67,
    0x75, 0x61, 0x67, 0x65, 0x20, 0x67, 0x6f, 0x76, 0x65, 0x72, 0x6e, 0x69, 0x6e, 0x67, 0x20, 0x70,
    0x65, 0x72, 0x6d, 0x69, 0x73, 0x73, 0x69, 0x6f, 0x6e, 0x73, 0x20, 0x61, 0x6e, 0x64, 0x0a, 0x20,
    0x6c, 0x69, 0x6d, 0x69, 0x74, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x20, 0x75, 0x6e, 0x64, 0x65,
    0x72, 0x20, 0x74, 0x68, 0x65, 0x20, 0x4c, 0x69, 0x63, 0x65, 0x6e, 0x73, 0x65, 0x2e, 0x0a, 0x0a,
    0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x0f, 0x08, 0x1c, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03,
    0x10, 0x00, 0x2d, 0x0a, 0x0b, 0x0a, 0x04, 0x08, 0xe7, 0x07, 0x00, 0x12, 0x03, 0x10, 0x00, 0x2d,
    0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x03, 0x10, 0x07, 0x13, 0x0a, 0x0d,
    0x0a, 0x06, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12, 0x03, 0x10, 0x07, 0x13, 0x0a, 0x0e, 0x0a,
    0x07, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x10, 0x07, 0x13, 0x0a, 0x0c, 0x0a,
    0x05, 0x08, 0xe7, 0x07, 0x00, 0x07, 0x12, 0x03, 0x10, 0x16, 0x2c, 0x0a, 0x0a, 0x0a, 0x02, 0x04,
    0x00, 0x12, 0x04, 0x12, 0x00, 0x15, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03,
    0x12, 0x08, 0x11, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x13, 0x02, 0x1c,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04, 0x12, 0x03, 0x13, 0x02, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x13, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x13, 0x12, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x00, 0x03, 0x12, 0x03, 0x13, 0x1a, 0x1b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01,
    0x12, 0x03, 0x14, 0x02, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x04, 0x12, 0x03,
    0x14, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x05, 0x12, 0x03, 0x14, 0x0b,
    0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x14, 0x12, 0x17, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x14, 0x1a, 0x1b, 0x0a, 0x0a, 0x0a,
    0x02, 0x05, 0x00, 0x12, 0x04, 0x17, 0x00, 0x1d, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x05, 0x00, 0x01,
    0x12, 0x03, 0x17, 0x05, 0x0f, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x00, 0x12, 0x03, 0x18,
    0x02, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x18, 0x02, 0x09,
    0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x00, 0x02, 0x12, 0x03, 0x18, 0x0f, 0x10, 0x0a, 0x0b,
    0x0a, 0x04, 0x05, 0x00, 0x02, 0x01, 0x12, 0x03, 0x19, 0x02, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x05,
    0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x19, 0x02, 0x07, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02,
    0x01, 0x02, 0x12, 0x03, 0x19, 0x0f, 0x10, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x02, 0x12,
    0x03, 0x1a, 0x02, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x1a,
    0x02, 0x09, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x02, 0x02, 0x12, 0x03, 0x1a, 0x0f, 0x10,
    0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x03, 0x12, 0x03, 0x1b, 0x02, 0x11, 0x0a, 0x0c, 0x0a,
    0x05, 0x05, 0x00, 0x02, 0x03, 0x01, 0x12, 0x03, 0x1b, 0x02, 0x09, 0x0a, 0x0c, 0x0a, 0x05, 0x05,
    0x00, 0x02, 0x03, 0x02, 0x12, 0x03, 0x1b, 0x0f, 0x10, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02,
    0x04, 0x12, 0x03, 0x1c, 0x02, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x04, 0x01, 0x12,
    0x03, 0x1c, 0x02, 0x0b, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x04, 0x02, 0x12, 0x03, 0x1c,
    0x0f, 0x10, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x01, 0x12, 0x04, 0x1f, 0x00, 0x21, 0x01, 0x0a, 0x0a,
    0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x1f, 0x08, 0x0d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01,
    0x02, 0x00, 0x12, 0x03, 0x20, 0x02, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x04,
    0x12, 0x03, 0x20, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x05, 0x12, 0x03,
    0x20, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x20, 0x12,
    0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x03, 0x12, 0x03, 0x20, 0x1a, 0x1b, 0x0a,
    0x0a, 0x0a, 0x02, 0x04, 0x02, 0x12, 0x04, 0x23, 0x00, 0x25, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04,
    0x02, 0x01, 0x12, 0x03, 0x23, 0x08, 0x0f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x00, 0x12,
    0x03, 0x24, 0x02, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x04, 0x12, 0x03, 0x24,
    0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x05, 0x12, 0x03, 0x24, 0x0b, 0x11,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x01, 0x12, 0x03, 0x24, 0x12, 0x17, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x03, 0x12, 0x03, 0x24, 0x1a, 0x1b, 0x0a, 0x0a, 0x0a, 0x02,
    0x04, 0x03, 0x12, 0x04, 0x27, 0x00, 0x2a, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x03, 0x01, 0x12,
    0x03, 0x27, 0x08, 0x10, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x00, 0x12, 0x03, 0x28, 0x02,
    0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x04, 0x12, 0x03, 0x28, 0x02, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x05, 0x12, 0x03, 0x28, 0x0b, 0x11, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x03, 0x02, 0x00, 0x01, 0x12, 0x03, 0x28, 0x12, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x03, 0x02, 0x00, 0x03, 0x12, 0x03, 0x28, 0x1d, 0x1e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02,
    0x01, 0x12, 0x03, 0x29, 0x02, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x04, 0x12,
    0x03, 0x29, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x05, 0x12, 0x03, 0x29,
    0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x01, 0x12, 0x03, 0x29, 0x12, 0x17,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x03, 0x12, 0x03, 0x29, 0x1d, 0x1e, 0x0a, 0x0a,
    0x0a, 0x02, 0x04, 0x04, 0x12, 0x04, 0x2c, 0x00, 0x30, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x04,
    0x01, 0x12, 0x03, 0x2c, 0x08, 0x0f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x00, 0x12, 0x03,
    0x2d, 0x02, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x04, 0x12, 0x03, 0x2d, 0x02,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x05, 0x12, 0x03, 0x2d, 0x0b, 0x11, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x01, 0x12, 0x03, 0x2d, 0x14, 0x20, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x04, 0x02, 0x00, 0x03, 0x12, 0x03, 0x2d, 0x23, 0x24, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x04, 0x02, 0x01, 0x12, 0x03, 0x2e, 0x02, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01,
    0x04, 0x12, 0x03, 0x2e, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x05, 0x12,
    0x03, 0x2e, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x01, 0x12, 0x03, 0x2e,
    0x14, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x03, 0x12, 0x03, 0x2e, 0x23, 0x24,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x02, 0x12, 0x03, 0x2f, 0x02, 0x25, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x04, 0x02, 0x02, 0x04, 0x12, 0x03, 0x2f, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x04, 0x02, 0x02, 0x06, 0x12, 0x03, 0x2f, 0x0b, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02,
    0x02, 0x01, 0x12, 0x03, 0x2f, 0x14, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x02, 0x03,
    0x12, 0x03, 0x2f, 0x23, 0x24, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x05, 0x12, 0x04, 0x32, 0x00, 0x34,
    0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x05, 0x01, 0x12, 0x03, 0x32, 0x08, 0x0f, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x05, 0x02, 0x00, 0x12, 0x03, 0x33, 0x02, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05,
    0x02, 0x00, 0x04, 0x12, 0x03, 0x33, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00,
    0x05, 0x12, 0x03, 0x33, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x01, 0x12,
    0x03, 0x33, 0x12, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x03, 0x12, 0x03, 0x33,
    0x1a, 0x1b, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x06, 0x12, 0x04, 0x36, 0x00, 0x3a, 0x01, 0x0a, 0x0a,
    0x0a, 0x03, 0x04, 0x06, 0x01, 0x12, 0x03, 0x36, 0x08, 0x11, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06,
    0x02, 0x00, 0x12, 0x03, 0x37, 0x02, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x04,
    0x12, 0x03, 0x37, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x05, 0x12, 0x03,
    0x37, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x01, 0x12, 0x03, 0x37, 0x12,
    0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x03, 0x12, 0x03, 0x37, 0x21, 0x22, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x01, 0x12, 0x03, 0x38, 0x02, 0x23, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x06, 0x02, 0x01, 0x04, 0x12, 0x03, 0x38, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06,
    0x02, 0x01, 0x05, 0x12, 0x03, 0x38, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01,
    0x01, 0x12, 0x03, 0x38, 0x12, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x03, 0x12,
    0x03, 0x38, 0x21, 0x22, 0x0a, 0x53, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x02, 0x12, 0x03, 0x39, 0x02,
    0x23, 0x22, 0x46, 0x20, 0x4f, 0x72, 0x64, 0x65, 0x72, 0x65, 0x64, 0x20, 0x69, 0x6e, 0x20, 0x69,
    0x6e, 0x63, 0x72, 0x65, 0x61, 0x73, 0x69, 0x6e, 0x67, 0x20, 0x6f, 0x72, 0x64, 0x65, 0x72, 0x20,
    0x6f, 0x66, 0x20, 0x75, 0x70, 0x70, 0x65, 0x72, 0x5f, 0x62, 0x6f, 0x75, 0x6e, 0x64, 0x2c, 0x20,
    0x2b, 0x49, 0x6e, 0x66, 0x20, 0x62, 0x75, 0x63, 0x6b, 0x65, 0x74, 0x20, 0x69, 0x73, 0x20, 0x6f,
    0x70, 0x74, 0x69, 0x6f, 0x6e, 0x61, 0x6c, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02,
    0x02, 0x04, 0x12, 0x03, 0x39, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x02, 0x06,
    0x12, 0x03, 0x39, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x02, 0x01, 0x12, 0x03,
    0x39, 0x12, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x02, 0x03, 0x12, 0x03, 0x39, 0x21,
    0x22, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x07, 0x12, 0x04, 0x3c, 0x00, 0x3f, 0x01, 0x0a, 0x0a, 0x0a,
    0x03, 0x04, 0x07, 0x01, 0x12, 0x03, 0x3c, 0x08, 0x0e, 0x0a, 0x2e, 0x0a, 0x04, 0x04, 0x07, 0x02,
    0x00, 0x12, 0x03, 0x3d, 0x02, 0x27, 0x22, 0x21, 0x20, 0x43, 0x75, 0x6d, 0x75, 0x6c, 0x61, 0x74,
    0x69, 0x76, 0x65, 0x20, 0x69, 0x6e, 0x20, 0x69, 0x6e, 0x63, 0x72, 0x65, 0x61, 0x73, 0x69, 0x6e,
    0x67, 0x20, 0x6f, 0x72, 0x64, 0x65, 0x72, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02,
    0x00, 0x04, 0x12, 0x03, 0x3d, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x05,
    0x12, 0x03, 0x3d, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x01, 0x12, 0x03,
    0x3d, 0x12, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x03, 0x12, 0x03, 0x3d, 0x25,
    0x26, 0x0a, 0x19, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x01, 0x12, 0x03, 0x3e, 0x02, 0x22, 0x22, 0x0c,
    0x20, 0x49, 0x6e, 0x63, 0x6c, 0x75, 0x73, 0x69, 0x76, 0x65, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x07, 0x02, 0x01, 0x04, 0x12, 0x03, 0x3e, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07,
    0x02, 0x01, 0x05, 0x12, 0x03, 0x3e, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x01,
    0x01, 0x12, 0x03, 0x3e, 0x12, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x01, 0x03, 0x12,
    0x03, 0x3e, 0x20, 0x21, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x08, 0x12, 0x04, 0x41, 0x00, 0x49, 0x01,
    0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x08, 0x01, 0x12, 0x03, 0x41, 0x08, 0x0e, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x08, 0x02, 0x00, 0x12, 0x03, 0x42, 0x02, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02,
    0x00, 0x04, 0x12, 0x03, 0x42, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x06,
    0x12, 0x03, 0x42, 0x0b, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x01, 0x12, 0x03,
    0x42, 0x15, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x03, 0x12, 0x03, 0x42, 0x24,
    0x25, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x08, 0x02, 0x01, 0x12, 0x03, 0x43, 0x02, 0x26, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x08, 0x02, 0x01, 0x04, 0x12, 0x03, 0x43, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x08, 0x02, 0x01, 0x06, 0x12, 0x03, 0x43, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08,
    0x02, 0x01, 0x01, 0x12, 0x03, 0x43, 0x15, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x01,
    0x03, 0x12, 0x03, 0x43, 0x24, 0x25, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x08, 0x02, 0x02, 0x12, 0x03,
    0x44, 0x02, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x02, 0x04, 0x12, 0x03, 0x44, 0x02,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x02, 0x06, 0x12, 0x03, 0x44, 0x0b, 0x12, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x02, 0x01, 0x12, 0x03, 0x44, 0x15, 0x1c, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x08, 0x02, 0x02, 0x03, 0x12, 0x03, 0x44, 0x24, 0x25, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x08, 0x02, 0x03, 0x12, 0x03, 0x45, 0x02, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x03,
    0x04, 0x12, 0x03, 0x45, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x03, 0x06, 0x12,
    0x03, 0x45, 0x0b, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x03, 0x01, 0x12, 0x03, 0x45,
    0x15, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x03, 0x03, 0x12, 0x03, 0x45, 0x24, 0x25,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x08, 0x02, 0x04, 0x12, 0x03, 0x46, 0x02, 0x26, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x08, 0x02, 0x04, 0x04, 0x12, 0x03, 0x46, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x08, 0x02, 0x04, 0x06, 0x12, 0x03, 0x46, 0x0b, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02,
    0x04, 0x01, 0x12, 0x03, 0x46, 0x15, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x04, 0x03,
    0x12, 0x03, 0x46, 0x24, 0x25, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x08, 0x02, 0x05, 0x12, 0x03, 0x47,
    0x02, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x05, 0x04, 0x12, 0x03, 0x47, 0x02, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x05, 0x06, 0x12, 0x03, 0x47, 0x0b, 0x14, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x08, 0x02, 0x05, 0x01, 0x12, 0x03, 0x47, 0x15, 0x1e, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x08, 0x02, 0x05, 0x03, 0x12, 0x03, 0x47, 0x24, 0x25, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x08,
    0x02, 0x06, 0x12, 0x03, 0x48, 0x02, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x06, 0x04,
    0x12, 0x03, 0x48, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x06, 0x05, 0x12, 0x03,
    0x48, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x06, 0x01, 0x12, 0x03, 0x48, 0x15,
    0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x06, 0x03, 0x12, 0x03, 0x48, 0x24, 0x25, 0x0a,
    0x0a, 0x0a, 0x02, 0x04, 0x09, 0x12, 0x04, 0x4b, 0x00, 0x50, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04,
    0x09, 0x01, 0x12, 0x03, 0x4b, 0x08, 0x14, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x00, 0x12,
    0x03, 0x4c, 0x02, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x04, 0x12, 0x03, 0x4c,
    0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x05, 0x12, 0x03, 0x4c, 0x0b, 0x11,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x01, 0x12, 0x03, 0x4c, 0x16, 0x1a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x03, 0x12, 0x03, 0x4c, 0x1f, 0x20, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x09, 0x02, 0x01, 0x12, 0x03, 0x4d, 0x02, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02,
    0x01, 0x04, 0x12, 0x03, 0x4d, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x01, 0x05,
    0x12, 0x03, 0x4d, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x01, 0x01, 0x12, 0x03,
    0x4d, 0x16, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x01, 0x03, 0x12, 0x03, 0x4d, 0x1f,
    0x20, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x02, 0x12, 0x03, 0x4e, 0x02, 0x21, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x09, 0x02, 0x02, 0x04, 0x12, 0x03, 0x4e, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x09, 0x02, 0x02, 0x06, 0x12, 0x03, 0x4e, 0x0b, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09,
    0x02, 0x02, 0x01, 0x12, 0x03, 0x4e, 0x16, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x02,
    0x03, 0x12, 0x03, 0x4e, 0x1f, 0x20, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x03, 0x12, 0x03,
    0x4f, 0x02, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x03, 0x04, 0x12, 0x03, 0x4f, 0x02,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x03, 0x06, 0x12, 0x03, 0x4f, 0x0b, 0x11, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x03, 0x01, 0x12, 0x03, 0x4f, 0x16, 0x1c, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x09, 0x02, 0x03, 0x03, 0x12, 0x03, 0x4f, 0x1f, 0x20,
];

static mut file_descriptor_proto_lazy: ::protobuf::lazy::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::lazy::Lazy {
    lock: ::protobuf::lazy::ONCE_INIT,
    ptr: 0 as *const ::protobuf::descriptor::FileDescriptorProto,
};

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    unsafe {
        file_descriptor_proto_lazy.get(|| {
            parse_descriptor_proto()
        })
    }
}
