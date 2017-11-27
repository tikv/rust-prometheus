// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
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
        }
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
        }
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
        if let Some(ref v) = self.name.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        if let Some(ref v) = self.value.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.name.as_ref() {
            os.write_string(1, &v)?;
        }
        if let Some(ref v) = self.value.as_ref() {
            os.write_string(2, &v)?;
        }
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
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
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
                    }
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
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.value {
            os.write_double(1, v)?;
        }
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
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
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
                    }
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
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.value {
            os.write_double(1, v)?;
        }
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
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
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
                    }
                    let tmp = is.read_double()?;
                    self.quantile = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
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
        }
        if let Some(v) = self.value {
            my_size += 9;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.quantile {
            os.write_double(1, v)?;
        }
        if let Some(v) = self.value {
            os.write_double(2, v)?;
        }
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
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
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
        for v in &self.quantile {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.sample_count = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
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
        }
        if let Some(v) = self.sample_sum {
            my_size += 9;
        }
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
        }
        if let Some(v) = self.sample_sum {
            os.write_double(2, v)?;
        }
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
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
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
                    }
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
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.value {
            os.write_double(1, v)?;
        }
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
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
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
        for v in &self.bucket {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.sample_count = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
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
        }
        if let Some(v) = self.sample_sum {
            my_size += 9;
        }
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
        }
        if let Some(v) = self.sample_sum {
            os.write_double(2, v)?;
        }
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
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
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
                    }
                    let tmp = is.read_uint64()?;
                    self.cumulative_count = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
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
        }
        if let Some(v) = self.upper_bound {
            my_size += 9;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.cumulative_count {
            os.write_uint64(1, v)?;
        }
        if let Some(v) = self.upper_bound {
            os.write_double(2, v)?;
        }
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
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
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
        }
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
        }
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
        }
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
        }
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
        }
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
        for v in &self.label {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.gauge {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.counter {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.summary {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.untyped {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.histogram {
            if !v.is_initialized() {
                return false;
            }
        };
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
                    }
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
        if let Some(ref v) = self.gauge.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.counter.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.summary.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.untyped.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.histogram.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(v) = self.timestamp_ms {
            my_size += ::protobuf::rt::value_size(6, v, ::protobuf::wire_format::WireTypeVarint);
        }
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
        if let Some(ref v) = self.gauge.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.counter.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.summary.as_ref() {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.untyped.as_ref() {
            os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.histogram.as_ref() {
            os.write_tag(7, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(v) = self.timestamp_ms {
            os.write_int64(6, v)?;
        }
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
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
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
        }
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
        }
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
        for v in &self.metric {
            if !v.is_initialized() {
                return false;
            }
        };
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
                    }
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
        if let Some(ref v) = self.name.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        if let Some(ref v) = self.help.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        if let Some(v) = self.field_type {
            my_size += ::protobuf::rt::enum_size(3, v);
        }
        for value in &self.metric {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.name.as_ref() {
            os.write_string(1, &v)?;
        }
        if let Some(ref v) = self.help.as_ref() {
            os.write_string(2, &v)?;
        }
        if let Some(v) = self.field_type {
            os.write_enum(3, v.value())?;
        }
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
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
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
        static values: &[MetricType] = &[
            MetricType::COUNTER,
            MetricType::GAUGE,
            MetricType::SUMMARY,
            MetricType::UNTYPED,
            MetricType::HISTOGRAM,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<MetricType>) -> &'static ::protobuf::reflect::EnumDescriptor {
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

static file_descriptor_proto_data: &[u8] = b"\
    \n\x13proto/metrics.proto\x12\x14io.prometheus.client\"5\n\tLabelPair\
    \x12\x12\n\x04name\x18\x01\x20\x01(\tR\x04name\x12\x14\n\x05value\x18\
    \x02\x20\x01(\tR\x05value\"\x1d\n\x05Gauge\x12\x14\n\x05value\x18\x01\
    \x20\x01(\x01R\x05value\"\x1f\n\x07Counter\x12\x14\n\x05value\x18\x01\
    \x20\x01(\x01R\x05value\"<\n\x08Quantile\x12\x1a\n\x08quantile\x18\x01\
    \x20\x01(\x01R\x08quantile\x12\x14\n\x05value\x18\x02\x20\x01(\x01R\x05v\
    alue\"\x87\x01\n\x07Summary\x12!\n\x0csample_count\x18\x01\x20\x01(\x04R\
    \x0bsampleCount\x12\x1d\n\nsample_sum\x18\x02\x20\x01(\x01R\tsampleSum\
    \x12:\n\x08quantile\x18\x03\x20\x03(\x0b2\x1e.io.prometheus.client.Quant\
    ileR\x08quantile\"\x1f\n\x07Untyped\x12\x14\n\x05value\x18\x01\x20\x01(\
    \x01R\x05value\"\x83\x01\n\tHistogram\x12!\n\x0csample_count\x18\x01\x20\
    \x01(\x04R\x0bsampleCount\x12\x1d\n\nsample_sum\x18\x02\x20\x01(\x01R\ts\
    ampleSum\x124\n\x06bucket\x18\x03\x20\x03(\x0b2\x1c.io.prometheus.client\
    .BucketR\x06bucket\"T\n\x06Bucket\x12)\n\x10cumulative_count\x18\x01\x20\
    \x01(\x04R\x0fcumulativeCount\x12\x1f\n\x0bupper_bound\x18\x02\x20\x01(\
    \x01R\nupperBound\"\xff\x02\n\x06Metric\x125\n\x05label\x18\x01\x20\x03(\
    \x0b2\x1f.io.prometheus.client.LabelPairR\x05label\x121\n\x05gauge\x18\
    \x02\x20\x01(\x0b2\x1b.io.prometheus.client.GaugeR\x05gauge\x127\n\x07co\
    unter\x18\x03\x20\x01(\x0b2\x1d.io.prometheus.client.CounterR\x07counter\
    \x127\n\x07summary\x18\x04\x20\x01(\x0b2\x1d.io.prometheus.client.Summar\
    yR\x07summary\x127\n\x07untyped\x18\x05\x20\x01(\x0b2\x1d.io.prometheus.\
    client.UntypedR\x07untyped\x12=\n\thistogram\x18\x07\x20\x01(\x0b2\x1f.i\
    o.prometheus.client.HistogramR\thistogram\x12!\n\x0ctimestamp_ms\x18\x06\
    \x20\x01(\x03R\x0btimestampMs\"\xa2\x01\n\x0cMetricFamily\x12\x12\n\x04n\
    ame\x18\x01\x20\x01(\tR\x04name\x12\x12\n\x04help\x18\x02\x20\x01(\tR\
    \x04help\x124\n\x04type\x18\x03\x20\x01(\x0e2\x20.io.prometheus.client.M\
    etricTypeR\x04type\x124\n\x06metric\x18\x04\x20\x03(\x0b2\x1c.io.prometh\
    eus.client.MetricR\x06metric*M\n\nMetricType\x12\x0b\n\x07COUNTER\x10\0\
    \x12\t\n\x05GAUGE\x10\x01\x12\x0b\n\x07SUMMARY\x10\x02\x12\x0b\n\x07UNTY\
    PED\x10\x03\x12\r\n\tHISTOGRAM\x10\x04B\x16\n\x14io.prometheus.clientJ\
    \xf3\x17\n\x06\x12\x04\r\0P\x01\n\xbc\x04\n\x01\x0c\x12\x03\r\0\x122\xb1\
    \x04\x20Copyright\x202013\x20Prometheus\x20Team\n\x20Licensed\x20under\
    \x20the\x20Apache\x20License,\x20Version\x202.0\x20(the\x20\"License\");\
    \n\x20you\x20may\x20not\x20use\x20this\x20file\x20except\x20in\x20compli\
    ance\x20with\x20the\x20License.\n\x20You\x20may\x20obtain\x20a\x20copy\
    \x20of\x20the\x20License\x20at\n\n\x20http://www.apache.org/licenses/LIC\
    ENSE-2.0\n\n\x20Unless\x20required\x20by\x20applicable\x20law\x20or\x20a\
    greed\x20to\x20in\x20writing,\x20software\n\x20distributed\x20under\x20t\
    he\x20License\x20is\x20distributed\x20on\x20an\x20\"AS\x20IS\"\x20BASIS,\
    \n\x20WITHOUT\x20WARRANTIES\x20OR\x20CONDITIONS\x20OF\x20ANY\x20KIND,\
    \x20either\x20express\x20or\x20implied.\n\x20See\x20the\x20License\x20fo\
    r\x20the\x20specific\x20language\x20governing\x20permissions\x20and\n\
    \x20limitations\x20under\x20the\x20License.\n\n\x08\n\x01\x02\x12\x03\
    \x0f\x08\x1c\n\x08\n\x01\x08\x12\x03\x10\0-\n\x0b\n\x04\x08\xe7\x07\0\
    \x12\x03\x10\0-\n\x0c\n\x05\x08\xe7\x07\0\x02\x12\x03\x10\x07\x13\n\r\n\
    \x06\x08\xe7\x07\0\x02\0\x12\x03\x10\x07\x13\n\x0e\n\x07\x08\xe7\x07\0\
    \x02\0\x01\x12\x03\x10\x07\x13\n\x0c\n\x05\x08\xe7\x07\0\x07\x12\x03\x10\
    \x16,\n\n\n\x02\x04\0\x12\x04\x12\0\x15\x01\n\n\n\x03\x04\0\x01\x12\x03\
    \x12\x08\x11\n\x0b\n\x04\x04\0\x02\0\x12\x03\x13\x02\x1c\n\x0c\n\x05\x04\
    \0\x02\0\x04\x12\x03\x13\x02\n\n\x0c\n\x05\x04\0\x02\0\x05\x12\x03\x13\
    \x0b\x11\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03\x13\x12\x16\n\x0c\n\x05\x04\
    \0\x02\0\x03\x12\x03\x13\x1a\x1b\n\x0b\n\x04\x04\0\x02\x01\x12\x03\x14\
    \x02\x1c\n\x0c\n\x05\x04\0\x02\x01\x04\x12\x03\x14\x02\n\n\x0c\n\x05\x04\
    \0\x02\x01\x05\x12\x03\x14\x0b\x11\n\x0c\n\x05\x04\0\x02\x01\x01\x12\x03\
    \x14\x12\x17\n\x0c\n\x05\x04\0\x02\x01\x03\x12\x03\x14\x1a\x1b\n\n\n\x02\
    \x05\0\x12\x04\x17\0\x1d\x01\n\n\n\x03\x05\0\x01\x12\x03\x17\x05\x0f\n\
    \x0b\n\x04\x05\0\x02\0\x12\x03\x18\x02\x11\n\x0c\n\x05\x05\0\x02\0\x01\
    \x12\x03\x18\x02\t\n\x0c\n\x05\x05\0\x02\0\x02\x12\x03\x18\x0f\x10\n\x0b\
    \n\x04\x05\0\x02\x01\x12\x03\x19\x02\x11\n\x0c\n\x05\x05\0\x02\x01\x01\
    \x12\x03\x19\x02\x07\n\x0c\n\x05\x05\0\x02\x01\x02\x12\x03\x19\x0f\x10\n\
    \x0b\n\x04\x05\0\x02\x02\x12\x03\x1a\x02\x11\n\x0c\n\x05\x05\0\x02\x02\
    \x01\x12\x03\x1a\x02\t\n\x0c\n\x05\x05\0\x02\x02\x02\x12\x03\x1a\x0f\x10\
    \n\x0b\n\x04\x05\0\x02\x03\x12\x03\x1b\x02\x11\n\x0c\n\x05\x05\0\x02\x03\
    \x01\x12\x03\x1b\x02\t\n\x0c\n\x05\x05\0\x02\x03\x02\x12\x03\x1b\x0f\x10\
    \n\x0b\n\x04\x05\0\x02\x04\x12\x03\x1c\x02\x11\n\x0c\n\x05\x05\0\x02\x04\
    \x01\x12\x03\x1c\x02\x0b\n\x0c\n\x05\x05\0\x02\x04\x02\x12\x03\x1c\x0f\
    \x10\n\n\n\x02\x04\x01\x12\x04\x1f\0!\x01\n\n\n\x03\x04\x01\x01\x12\x03\
    \x1f\x08\r\n\x0b\n\x04\x04\x01\x02\0\x12\x03\x20\x02\x1c\n\x0c\n\x05\x04\
    \x01\x02\0\x04\x12\x03\x20\x02\n\n\x0c\n\x05\x04\x01\x02\0\x05\x12\x03\
    \x20\x0b\x11\n\x0c\n\x05\x04\x01\x02\0\x01\x12\x03\x20\x12\x17\n\x0c\n\
    \x05\x04\x01\x02\0\x03\x12\x03\x20\x1a\x1b\n\n\n\x02\x04\x02\x12\x04#\0%\
    \x01\n\n\n\x03\x04\x02\x01\x12\x03#\x08\x0f\n\x0b\n\x04\x04\x02\x02\0\
    \x12\x03$\x02\x1c\n\x0c\n\x05\x04\x02\x02\0\x04\x12\x03$\x02\n\n\x0c\n\
    \x05\x04\x02\x02\0\x05\x12\x03$\x0b\x11\n\x0c\n\x05\x04\x02\x02\0\x01\
    \x12\x03$\x12\x17\n\x0c\n\x05\x04\x02\x02\0\x03\x12\x03$\x1a\x1b\n\n\n\
    \x02\x04\x03\x12\x04'\0*\x01\n\n\n\x03\x04\x03\x01\x12\x03'\x08\x10\n\
    \x0b\n\x04\x04\x03\x02\0\x12\x03(\x02\x1f\n\x0c\n\x05\x04\x03\x02\0\x04\
    \x12\x03(\x02\n\n\x0c\n\x05\x04\x03\x02\0\x05\x12\x03(\x0b\x11\n\x0c\n\
    \x05\x04\x03\x02\0\x01\x12\x03(\x12\x1a\n\x0c\n\x05\x04\x03\x02\0\x03\
    \x12\x03(\x1d\x1e\n\x0b\n\x04\x04\x03\x02\x01\x12\x03)\x02\x1f\n\x0c\n\
    \x05\x04\x03\x02\x01\x04\x12\x03)\x02\n\n\x0c\n\x05\x04\x03\x02\x01\x05\
    \x12\x03)\x0b\x11\n\x0c\n\x05\x04\x03\x02\x01\x01\x12\x03)\x12\x17\n\x0c\
    \n\x05\x04\x03\x02\x01\x03\x12\x03)\x1d\x1e\n\n\n\x02\x04\x04\x12\x04,\0\
    0\x01\n\n\n\x03\x04\x04\x01\x12\x03,\x08\x0f\n\x0b\n\x04\x04\x04\x02\0\
    \x12\x03-\x02%\n\x0c\n\x05\x04\x04\x02\0\x04\x12\x03-\x02\n\n\x0c\n\x05\
    \x04\x04\x02\0\x05\x12\x03-\x0b\x11\n\x0c\n\x05\x04\x04\x02\0\x01\x12\
    \x03-\x14\x20\n\x0c\n\x05\x04\x04\x02\0\x03\x12\x03-#$\n\x0b\n\x04\x04\
    \x04\x02\x01\x12\x03.\x02%\n\x0c\n\x05\x04\x04\x02\x01\x04\x12\x03.\x02\
    \n\n\x0c\n\x05\x04\x04\x02\x01\x05\x12\x03.\x0b\x11\n\x0c\n\x05\x04\x04\
    \x02\x01\x01\x12\x03.\x14\x1e\n\x0c\n\x05\x04\x04\x02\x01\x03\x12\x03.#$\
    \n\x0b\n\x04\x04\x04\x02\x02\x12\x03/\x02%\n\x0c\n\x05\x04\x04\x02\x02\
    \x04\x12\x03/\x02\n\n\x0c\n\x05\x04\x04\x02\x02\x06\x12\x03/\x0b\x13\n\
    \x0c\n\x05\x04\x04\x02\x02\x01\x12\x03/\x14\x1c\n\x0c\n\x05\x04\x04\x02\
    \x02\x03\x12\x03/#$\n\n\n\x02\x04\x05\x12\x042\04\x01\n\n\n\x03\x04\x05\
    \x01\x12\x032\x08\x0f\n\x0b\n\x04\x04\x05\x02\0\x12\x033\x02\x1c\n\x0c\n\
    \x05\x04\x05\x02\0\x04\x12\x033\x02\n\n\x0c\n\x05\x04\x05\x02\0\x05\x12\
    \x033\x0b\x11\n\x0c\n\x05\x04\x05\x02\0\x01\x12\x033\x12\x17\n\x0c\n\x05\
    \x04\x05\x02\0\x03\x12\x033\x1a\x1b\n\n\n\x02\x04\x06\x12\x046\0:\x01\n\
    \n\n\x03\x04\x06\x01\x12\x036\x08\x11\n\x0b\n\x04\x04\x06\x02\0\x12\x037\
    \x02#\n\x0c\n\x05\x04\x06\x02\0\x04\x12\x037\x02\n\n\x0c\n\x05\x04\x06\
    \x02\0\x05\x12\x037\x0b\x11\n\x0c\n\x05\x04\x06\x02\0\x01\x12\x037\x12\
    \x1e\n\x0c\n\x05\x04\x06\x02\0\x03\x12\x037!\"\n\x0b\n\x04\x04\x06\x02\
    \x01\x12\x038\x02#\n\x0c\n\x05\x04\x06\x02\x01\x04\x12\x038\x02\n\n\x0c\
    \n\x05\x04\x06\x02\x01\x05\x12\x038\x0b\x11\n\x0c\n\x05\x04\x06\x02\x01\
    \x01\x12\x038\x12\x1c\n\x0c\n\x05\x04\x06\x02\x01\x03\x12\x038!\"\nS\n\
    \x04\x04\x06\x02\x02\x12\x039\x02#\"F\x20Ordered\x20in\x20increasing\x20\
    order\x20of\x20upper_bound,\x20+Inf\x20bucket\x20is\x20optional.\n\n\x0c\
    \n\x05\x04\x06\x02\x02\x04\x12\x039\x02\n\n\x0c\n\x05\x04\x06\x02\x02\
    \x06\x12\x039\x0b\x11\n\x0c\n\x05\x04\x06\x02\x02\x01\x12\x039\x12\x18\n\
    \x0c\n\x05\x04\x06\x02\x02\x03\x12\x039!\"\n\n\n\x02\x04\x07\x12\x04<\0?\
    \x01\n\n\n\x03\x04\x07\x01\x12\x03<\x08\x0e\n.\n\x04\x04\x07\x02\0\x12\
    \x03=\x02'\"!\x20Cumulative\x20in\x20increasing\x20order.\n\n\x0c\n\x05\
    \x04\x07\x02\0\x04\x12\x03=\x02\n\n\x0c\n\x05\x04\x07\x02\0\x05\x12\x03=\
    \x0b\x11\n\x0c\n\x05\x04\x07\x02\0\x01\x12\x03=\x12\"\n\x0c\n\x05\x04\
    \x07\x02\0\x03\x12\x03=%&\n\x19\n\x04\x04\x07\x02\x01\x12\x03>\x02\"\"\
    \x0c\x20Inclusive.\n\n\x0c\n\x05\x04\x07\x02\x01\x04\x12\x03>\x02\n\n\
    \x0c\n\x05\x04\x07\x02\x01\x05\x12\x03>\x0b\x11\n\x0c\n\x05\x04\x07\x02\
    \x01\x01\x12\x03>\x12\x1d\n\x0c\n\x05\x04\x07\x02\x01\x03\x12\x03>\x20!\
    \n\n\n\x02\x04\x08\x12\x04A\0I\x01\n\n\n\x03\x04\x08\x01\x12\x03A\x08\
    \x0e\n\x0b\n\x04\x04\x08\x02\0\x12\x03B\x02&\n\x0c\n\x05\x04\x08\x02\0\
    \x04\x12\x03B\x02\n\n\x0c\n\x05\x04\x08\x02\0\x06\x12\x03B\x0b\x14\n\x0c\
    \n\x05\x04\x08\x02\0\x01\x12\x03B\x15\x1a\n\x0c\n\x05\x04\x08\x02\0\x03\
    \x12\x03B$%\n\x0b\n\x04\x04\x08\x02\x01\x12\x03C\x02&\n\x0c\n\x05\x04\
    \x08\x02\x01\x04\x12\x03C\x02\n\n\x0c\n\x05\x04\x08\x02\x01\x06\x12\x03C\
    \x0b\x10\n\x0c\n\x05\x04\x08\x02\x01\x01\x12\x03C\x15\x1a\n\x0c\n\x05\
    \x04\x08\x02\x01\x03\x12\x03C$%\n\x0b\n\x04\x04\x08\x02\x02\x12\x03D\x02\
    &\n\x0c\n\x05\x04\x08\x02\x02\x04\x12\x03D\x02\n\n\x0c\n\x05\x04\x08\x02\
    \x02\x06\x12\x03D\x0b\x12\n\x0c\n\x05\x04\x08\x02\x02\x01\x12\x03D\x15\
    \x1c\n\x0c\n\x05\x04\x08\x02\x02\x03\x12\x03D$%\n\x0b\n\x04\x04\x08\x02\
    \x03\x12\x03E\x02&\n\x0c\n\x05\x04\x08\x02\x03\x04\x12\x03E\x02\n\n\x0c\
    \n\x05\x04\x08\x02\x03\x06\x12\x03E\x0b\x12\n\x0c\n\x05\x04\x08\x02\x03\
    \x01\x12\x03E\x15\x1c\n\x0c\n\x05\x04\x08\x02\x03\x03\x12\x03E$%\n\x0b\n\
    \x04\x04\x08\x02\x04\x12\x03F\x02&\n\x0c\n\x05\x04\x08\x02\x04\x04\x12\
    \x03F\x02\n\n\x0c\n\x05\x04\x08\x02\x04\x06\x12\x03F\x0b\x12\n\x0c\n\x05\
    \x04\x08\x02\x04\x01\x12\x03F\x15\x1c\n\x0c\n\x05\x04\x08\x02\x04\x03\
    \x12\x03F$%\n\x0b\n\x04\x04\x08\x02\x05\x12\x03G\x02&\n\x0c\n\x05\x04\
    \x08\x02\x05\x04\x12\x03G\x02\n\n\x0c\n\x05\x04\x08\x02\x05\x06\x12\x03G\
    \x0b\x14\n\x0c\n\x05\x04\x08\x02\x05\x01\x12\x03G\x15\x1e\n\x0c\n\x05\
    \x04\x08\x02\x05\x03\x12\x03G$%\n\x0b\n\x04\x04\x08\x02\x06\x12\x03H\x02\
    &\n\x0c\n\x05\x04\x08\x02\x06\x04\x12\x03H\x02\n\n\x0c\n\x05\x04\x08\x02\
    \x06\x05\x12\x03H\x0b\x10\n\x0c\n\x05\x04\x08\x02\x06\x01\x12\x03H\x15!\
    \n\x0c\n\x05\x04\x08\x02\x06\x03\x12\x03H$%\n\n\n\x02\x04\t\x12\x04K\0P\
    \x01\n\n\n\x03\x04\t\x01\x12\x03K\x08\x14\n\x0b\n\x04\x04\t\x02\0\x12\
    \x03L\x02!\n\x0c\n\x05\x04\t\x02\0\x04\x12\x03L\x02\n\n\x0c\n\x05\x04\t\
    \x02\0\x05\x12\x03L\x0b\x11\n\x0c\n\x05\x04\t\x02\0\x01\x12\x03L\x16\x1a\
    \n\x0c\n\x05\x04\t\x02\0\x03\x12\x03L\x1f\x20\n\x0b\n\x04\x04\t\x02\x01\
    \x12\x03M\x02!\n\x0c\n\x05\x04\t\x02\x01\x04\x12\x03M\x02\n\n\x0c\n\x05\
    \x04\t\x02\x01\x05\x12\x03M\x0b\x11\n\x0c\n\x05\x04\t\x02\x01\x01\x12\
    \x03M\x16\x1a\n\x0c\n\x05\x04\t\x02\x01\x03\x12\x03M\x1f\x20\n\x0b\n\x04\
    \x04\t\x02\x02\x12\x03N\x02!\n\x0c\n\x05\x04\t\x02\x02\x04\x12\x03N\x02\
    \n\n\x0c\n\x05\x04\t\x02\x02\x06\x12\x03N\x0b\x15\n\x0c\n\x05\x04\t\x02\
    \x02\x01\x12\x03N\x16\x1a\n\x0c\n\x05\x04\t\x02\x02\x03\x12\x03N\x1f\x20\
    \n\x0b\n\x04\x04\t\x02\x03\x12\x03O\x02!\n\x0c\n\x05\x04\t\x02\x03\x04\
    \x12\x03O\x02\n\n\x0c\n\x05\x04\t\x02\x03\x06\x12\x03O\x0b\x11\n\x0c\n\
    \x05\x04\t\x02\x03\x01\x12\x03O\x16\x1c\n\x0c\n\x05\x04\t\x02\x03\x03\
    \x12\x03O\x1f\x20\
";

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
