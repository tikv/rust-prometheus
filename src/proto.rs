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

#[derive(Clone,Default)]
pub struct LabelPair {
    // message fields
    name: ::protobuf::SingularField<::std::string::String>,
    value: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
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
            instance.get(|| {
                LabelPair {
                    name: ::protobuf::SingularField::none(),
                    value: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
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
}

impl ::protobuf::Message for LabelPair {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.name));
                },
                2 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.value));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.name {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        for value in &self.value {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.name.as_ref() {
            try!(os.write_string(1, &v));
        };
        if let Some(v) = self.value.as_ref() {
            try!(os.write_string(2, &v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<LabelPair>()
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
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "name",
                    LabelPair::has_name,
                    LabelPair::get_name,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "value",
                    LabelPair::has_value,
                    LabelPair::get_value,
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

impl ::std::cmp::PartialEq for LabelPair {
    fn eq(&self, other: &LabelPair) -> bool {
        self.name == other.name &&
        self.value == other.value &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for LabelPair {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct Gauge {
    // message fields
    value: ::std::option::Option<f64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
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
            instance.get(|| {
                Gauge {
                    value: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
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
}

impl ::protobuf::Message for Gauge {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.value = ::std::option::Option::Some(tmp);
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.value.is_some() {
            my_size += 9;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.value {
            try!(os.write_double(1, v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<Gauge>()
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
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "value",
                    Gauge::has_value,
                    Gauge::get_value,
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

impl ::std::cmp::PartialEq for Gauge {
    fn eq(&self, other: &Gauge) -> bool {
        self.value == other.value &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Gauge {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct Counter {
    // message fields
    value: ::std::option::Option<f64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
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
            instance.get(|| {
                Counter {
                    value: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
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
}

impl ::protobuf::Message for Counter {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.value = ::std::option::Option::Some(tmp);
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.value.is_some() {
            my_size += 9;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.value {
            try!(os.write_double(1, v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<Counter>()
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
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "value",
                    Counter::has_value,
                    Counter::get_value,
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

impl ::std::cmp::PartialEq for Counter {
    fn eq(&self, other: &Counter) -> bool {
        self.value == other.value &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Counter {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct Quantile {
    // message fields
    quantile: ::std::option::Option<f64>,
    value: ::std::option::Option<f64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
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
            instance.get(|| {
                Quantile {
                    quantile: ::std::option::Option::None,
                    value: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
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
}

impl ::protobuf::Message for Quantile {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.quantile = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.value = ::std::option::Option::Some(tmp);
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.quantile.is_some() {
            my_size += 9;
        };
        if self.value.is_some() {
            my_size += 9;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.quantile {
            try!(os.write_double(1, v));
        };
        if let Some(v) = self.value {
            try!(os.write_double(2, v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<Quantile>()
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
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "quantile",
                    Quantile::has_quantile,
                    Quantile::get_quantile,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "value",
                    Quantile::has_value,
                    Quantile::get_value,
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

impl ::std::cmp::PartialEq for Quantile {
    fn eq(&self, other: &Quantile) -> bool {
        self.quantile == other.quantile &&
        self.value == other.value &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Quantile {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct Summary {
    // message fields
    sample_count: ::std::option::Option<u64>,
    sample_sum: ::std::option::Option<f64>,
    quantile: ::protobuf::RepeatedField<Quantile>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
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
            instance.get(|| {
                Summary {
                    sample_count: ::std::option::Option::None,
                    sample_sum: ::std::option::Option::None,
                    quantile: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
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
}

impl ::protobuf::Message for Summary {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint64());
                    self.sample_count = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.sample_sum = ::std::option::Option::Some(tmp);
                },
                3 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.quantile));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.sample_count {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        if self.sample_sum.is_some() {
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
            try!(os.write_uint64(1, v));
        };
        if let Some(v) = self.sample_sum {
            try!(os.write_double(2, v));
        };
        for v in &self.quantile {
            try!(os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<Summary>()
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
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "sample_count",
                    Summary::has_sample_count,
                    Summary::get_sample_count,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "sample_sum",
                    Summary::has_sample_sum,
                    Summary::get_sample_sum,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "quantile",
                    Summary::get_quantile,
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

impl ::std::cmp::PartialEq for Summary {
    fn eq(&self, other: &Summary) -> bool {
        self.sample_count == other.sample_count &&
        self.sample_sum == other.sample_sum &&
        self.quantile == other.quantile &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Summary {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct Untyped {
    // message fields
    value: ::std::option::Option<f64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
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
            instance.get(|| {
                Untyped {
                    value: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
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
}

impl ::protobuf::Message for Untyped {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.value = ::std::option::Option::Some(tmp);
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.value.is_some() {
            my_size += 9;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.value {
            try!(os.write_double(1, v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<Untyped>()
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
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "value",
                    Untyped::has_value,
                    Untyped::get_value,
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

impl ::std::cmp::PartialEq for Untyped {
    fn eq(&self, other: &Untyped) -> bool {
        self.value == other.value &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Untyped {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct Histogram {
    // message fields
    sample_count: ::std::option::Option<u64>,
    sample_sum: ::std::option::Option<f64>,
    bucket: ::protobuf::RepeatedField<Bucket>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
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
            instance.get(|| {
                Histogram {
                    sample_count: ::std::option::Option::None,
                    sample_sum: ::std::option::Option::None,
                    bucket: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
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
}

impl ::protobuf::Message for Histogram {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint64());
                    self.sample_count = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.sample_sum = ::std::option::Option::Some(tmp);
                },
                3 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.bucket));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.sample_count {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        if self.sample_sum.is_some() {
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
            try!(os.write_uint64(1, v));
        };
        if let Some(v) = self.sample_sum {
            try!(os.write_double(2, v));
        };
        for v in &self.bucket {
            try!(os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<Histogram>()
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
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "sample_count",
                    Histogram::has_sample_count,
                    Histogram::get_sample_count,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "sample_sum",
                    Histogram::has_sample_sum,
                    Histogram::get_sample_sum,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "bucket",
                    Histogram::get_bucket,
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

impl ::std::cmp::PartialEq for Histogram {
    fn eq(&self, other: &Histogram) -> bool {
        self.sample_count == other.sample_count &&
        self.sample_sum == other.sample_sum &&
        self.bucket == other.bucket &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Histogram {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct Bucket {
    // message fields
    cumulative_count: ::std::option::Option<u64>,
    upper_bound: ::std::option::Option<f64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
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
            instance.get(|| {
                Bucket {
                    cumulative_count: ::std::option::Option::None,
                    upper_bound: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
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
}

impl ::protobuf::Message for Bucket {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint64());
                    self.cumulative_count = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.upper_bound = ::std::option::Option::Some(tmp);
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.cumulative_count {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        if self.upper_bound.is_some() {
            my_size += 9;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.cumulative_count {
            try!(os.write_uint64(1, v));
        };
        if let Some(v) = self.upper_bound {
            try!(os.write_double(2, v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<Bucket>()
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
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "cumulative_count",
                    Bucket::has_cumulative_count,
                    Bucket::get_cumulative_count,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "upper_bound",
                    Bucket::has_upper_bound,
                    Bucket::get_upper_bound,
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

impl ::std::cmp::PartialEq for Bucket {
    fn eq(&self, other: &Bucket) -> bool {
        self.cumulative_count == other.cumulative_count &&
        self.upper_bound == other.upper_bound &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Bucket {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
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
    cached_size: ::std::cell::Cell<u32>,
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
            instance.get(|| {
                Metric {
                    label: ::protobuf::RepeatedField::new(),
                    gauge: ::protobuf::SingularPtrField::none(),
                    counter: ::protobuf::SingularPtrField::none(),
                    summary: ::protobuf::SingularPtrField::none(),
                    untyped: ::protobuf::SingularPtrField::none(),
                    histogram: ::protobuf::SingularPtrField::none(),
                    timestamp_ms: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
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
}

impl ::protobuf::Message for Metric {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.label));
                },
                2 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.gauge));
                },
                3 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.counter));
                },
                4 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.summary));
                },
                5 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.untyped));
                },
                7 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.histogram));
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int64());
                    self.timestamp_ms = ::std::option::Option::Some(tmp);
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
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
        for value in &self.gauge {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.counter {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.summary {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.untyped {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.histogram {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.timestamp_ms {
            my_size += ::protobuf::rt::value_size(6, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.label {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.gauge.as_ref() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.counter.as_ref() {
            try!(os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.summary.as_ref() {
            try!(os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.untyped.as_ref() {
            try!(os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.histogram.as_ref() {
            try!(os.write_tag(7, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.timestamp_ms {
            try!(os.write_int64(6, v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<Metric>()
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
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "label",
                    Metric::get_label,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "gauge",
                    Metric::has_gauge,
                    Metric::get_gauge,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "counter",
                    Metric::has_counter,
                    Metric::get_counter,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "summary",
                    Metric::has_summary,
                    Metric::get_summary,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "untyped",
                    Metric::has_untyped,
                    Metric::get_untyped,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "histogram",
                    Metric::has_histogram,
                    Metric::get_histogram,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor(
                    "timestamp_ms",
                    Metric::has_timestamp_ms,
                    Metric::get_timestamp_ms,
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

impl ::std::cmp::PartialEq for Metric {
    fn eq(&self, other: &Metric) -> bool {
        self.label == other.label &&
        self.gauge == other.gauge &&
        self.counter == other.counter &&
        self.summary == other.summary &&
        self.untyped == other.untyped &&
        self.histogram == other.histogram &&
        self.timestamp_ms == other.timestamp_ms &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Metric {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct MetricFamily {
    // message fields
    name: ::protobuf::SingularField<::std::string::String>,
    help: ::protobuf::SingularField<::std::string::String>,
    field_type: ::std::option::Option<MetricType>,
    metric: ::protobuf::RepeatedField<Metric>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
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
            instance.get(|| {
                MetricFamily {
                    name: ::protobuf::SingularField::none(),
                    help: ::protobuf::SingularField::none(),
                    field_type: ::std::option::Option::None,
                    metric: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
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
}

impl ::protobuf::Message for MetricFamily {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.name));
                },
                2 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.help));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_enum());
                    self.field_type = ::std::option::Option::Some(tmp);
                },
                4 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.metric));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.name {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        for value in &self.help {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        for value in &self.field_type {
            my_size += ::protobuf::rt::enum_size(3, *value);
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
            try!(os.write_string(1, &v));
        };
        if let Some(v) = self.help.as_ref() {
            try!(os.write_string(2, &v));
        };
        if let Some(v) = self.field_type {
            try!(os.write_enum(3, v.value()));
        };
        for v in &self.metric {
            try!(os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<MetricFamily>()
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
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "name",
                    MetricFamily::has_name,
                    MetricFamily::get_name,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "help",
                    MetricFamily::has_help,
                    MetricFamily::get_help,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "type",
                    MetricFamily::has_field_type,
                    MetricFamily::get_field_type,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "metric",
                    MetricFamily::get_metric,
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

impl ::std::cmp::PartialEq for MetricFamily {
    fn eq(&self, other: &MetricFamily) -> bool {
        self.name == other.name &&
        self.help == other.help &&
        self.field_type == other.field_type &&
        self.metric == other.metric &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for MetricFamily {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
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

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x0d, 0x6d, 0x65, 0x74, 0x72, 0x69, 0x63, 0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12,
    0x14, 0x69, 0x6f, 0x2e, 0x70, 0x72, 0x6f, 0x6d, 0x65, 0x74, 0x68, 0x65, 0x75, 0x73, 0x2e, 0x63,
    0x6c, 0x69, 0x65, 0x6e, 0x74, 0x22, 0x28, 0x0a, 0x09, 0x4c, 0x61, 0x62, 0x65, 0x6c, 0x50, 0x61,
    0x69, 0x72, 0x12, 0x0c, 0x0a, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09,
    0x12, 0x0d, 0x0a, 0x05, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x22,
    0x16, 0x0a, 0x05, 0x47, 0x61, 0x75, 0x67, 0x65, 0x12, 0x0d, 0x0a, 0x05, 0x76, 0x61, 0x6c, 0x75,
    0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x01, 0x22, 0x18, 0x0a, 0x07, 0x43, 0x6f, 0x75, 0x6e, 0x74,
    0x65, 0x72, 0x12, 0x0d, 0x0a, 0x05, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28,
    0x01, 0x22, 0x2b, 0x0a, 0x08, 0x51, 0x75, 0x61, 0x6e, 0x74, 0x69, 0x6c, 0x65, 0x12, 0x10, 0x0a,
    0x08, 0x71, 0x75, 0x61, 0x6e, 0x74, 0x69, 0x6c, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x01, 0x12,
    0x0d, 0x0a, 0x05, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x01, 0x22, 0x65,
    0x0a, 0x07, 0x53, 0x75, 0x6d, 0x6d, 0x61, 0x72, 0x79, 0x12, 0x14, 0x0a, 0x0c, 0x73, 0x61, 0x6d,
    0x70, 0x6c, 0x65, 0x5f, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x18, 0x01, 0x20, 0x01, 0x28, 0x04, 0x12,
    0x12, 0x0a, 0x0a, 0x73, 0x61, 0x6d, 0x70, 0x6c, 0x65, 0x5f, 0x73, 0x75, 0x6d, 0x18, 0x02, 0x20,
    0x01, 0x28, 0x01, 0x12, 0x30, 0x0a, 0x08, 0x71, 0x75, 0x61, 0x6e, 0x74, 0x69, 0x6c, 0x65, 0x18,
    0x03, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x1e, 0x2e, 0x69, 0x6f, 0x2e, 0x70, 0x72, 0x6f, 0x6d, 0x65,
    0x74, 0x68, 0x65, 0x75, 0x73, 0x2e, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x2e, 0x51, 0x75, 0x61,
    0x6e, 0x74, 0x69, 0x6c, 0x65, 0x22, 0x18, 0x0a, 0x07, 0x55, 0x6e, 0x74, 0x79, 0x70, 0x65, 0x64,
    0x12, 0x0d, 0x0a, 0x05, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x01, 0x22,
    0x63, 0x0a, 0x09, 0x48, 0x69, 0x73, 0x74, 0x6f, 0x67, 0x72, 0x61, 0x6d, 0x12, 0x14, 0x0a, 0x0c,
    0x73, 0x61, 0x6d, 0x70, 0x6c, 0x65, 0x5f, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x18, 0x01, 0x20, 0x01,
    0x28, 0x04, 0x12, 0x12, 0x0a, 0x0a, 0x73, 0x61, 0x6d, 0x70, 0x6c, 0x65, 0x5f, 0x73, 0x75, 0x6d,
    0x18, 0x02, 0x20, 0x01, 0x28, 0x01, 0x12, 0x2c, 0x0a, 0x06, 0x62, 0x75, 0x63, 0x6b, 0x65, 0x74,
    0x18, 0x03, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x1c, 0x2e, 0x69, 0x6f, 0x2e, 0x70, 0x72, 0x6f, 0x6d,
    0x65, 0x74, 0x68, 0x65, 0x75, 0x73, 0x2e, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x2e, 0x42, 0x75,
    0x63, 0x6b, 0x65, 0x74, 0x22, 0x37, 0x0a, 0x06, 0x42, 0x75, 0x63, 0x6b, 0x65, 0x74, 0x12, 0x18,
    0x0a, 0x10, 0x63, 0x75, 0x6d, 0x75, 0x6c, 0x61, 0x74, 0x69, 0x76, 0x65, 0x5f, 0x63, 0x6f, 0x75,
    0x6e, 0x74, 0x18, 0x01, 0x20, 0x01, 0x28, 0x04, 0x12, 0x13, 0x0a, 0x0b, 0x75, 0x70, 0x70, 0x65,
    0x72, 0x5f, 0x62, 0x6f, 0x75, 0x6e, 0x64, 0x18, 0x02, 0x20, 0x01, 0x28, 0x01, 0x22, 0xbe, 0x02,
    0x0a, 0x06, 0x4d, 0x65, 0x74, 0x72, 0x69, 0x63, 0x12, 0x2e, 0x0a, 0x05, 0x6c, 0x61, 0x62, 0x65,
    0x6c, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x1f, 0x2e, 0x69, 0x6f, 0x2e, 0x70, 0x72, 0x6f,
    0x6d, 0x65, 0x74, 0x68, 0x65, 0x75, 0x73, 0x2e, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x2e, 0x4c,
    0x61, 0x62, 0x65, 0x6c, 0x50, 0x61, 0x69, 0x72, 0x12, 0x2a, 0x0a, 0x05, 0x67, 0x61, 0x75, 0x67,
    0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1b, 0x2e, 0x69, 0x6f, 0x2e, 0x70, 0x72, 0x6f,
    0x6d, 0x65, 0x74, 0x68, 0x65, 0x75, 0x73, 0x2e, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x2e, 0x47,
    0x61, 0x75, 0x67, 0x65, 0x12, 0x2e, 0x0a, 0x07, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x65, 0x72, 0x18,
    0x03, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1d, 0x2e, 0x69, 0x6f, 0x2e, 0x70, 0x72, 0x6f, 0x6d, 0x65,
    0x74, 0x68, 0x65, 0x75, 0x73, 0x2e, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x2e, 0x43, 0x6f, 0x75,
    0x6e, 0x74, 0x65, 0x72, 0x12, 0x2e, 0x0a, 0x07, 0x73, 0x75, 0x6d, 0x6d, 0x61, 0x72, 0x79, 0x18,
    0x04, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1d, 0x2e, 0x69, 0x6f, 0x2e, 0x70, 0x72, 0x6f, 0x6d, 0x65,
    0x74, 0x68, 0x65, 0x75, 0x73, 0x2e, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x2e, 0x53, 0x75, 0x6d,
    0x6d, 0x61, 0x72, 0x79, 0x12, 0x2e, 0x0a, 0x07, 0x75, 0x6e, 0x74, 0x79, 0x70, 0x65, 0x64, 0x18,
    0x05, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1d, 0x2e, 0x69, 0x6f, 0x2e, 0x70, 0x72, 0x6f, 0x6d, 0x65,
    0x74, 0x68, 0x65, 0x75, 0x73, 0x2e, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x2e, 0x55, 0x6e, 0x74,
    0x79, 0x70, 0x65, 0x64, 0x12, 0x32, 0x0a, 0x09, 0x68, 0x69, 0x73, 0x74, 0x6f, 0x67, 0x72, 0x61,
    0x6d, 0x18, 0x07, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1f, 0x2e, 0x69, 0x6f, 0x2e, 0x70, 0x72, 0x6f,
    0x6d, 0x65, 0x74, 0x68, 0x65, 0x75, 0x73, 0x2e, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x2e, 0x48,
    0x69, 0x73, 0x74, 0x6f, 0x67, 0x72, 0x61, 0x6d, 0x12, 0x14, 0x0a, 0x0c, 0x74, 0x69, 0x6d, 0x65,
    0x73, 0x74, 0x61, 0x6d, 0x70, 0x5f, 0x6d, 0x73, 0x18, 0x06, 0x20, 0x01, 0x28, 0x03, 0x22, 0x88,
    0x01, 0x0a, 0x0c, 0x4d, 0x65, 0x74, 0x72, 0x69, 0x63, 0x46, 0x61, 0x6d, 0x69, 0x6c, 0x79, 0x12,
    0x0c, 0x0a, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x12, 0x0c, 0x0a,
    0x04, 0x68, 0x65, 0x6c, 0x70, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x12, 0x2e, 0x0a, 0x04, 0x74,
    0x79, 0x70, 0x65, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x20, 0x2e, 0x69, 0x6f, 0x2e, 0x70,
    0x72, 0x6f, 0x6d, 0x65, 0x74, 0x68, 0x65, 0x75, 0x73, 0x2e, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74,
    0x2e, 0x4d, 0x65, 0x74, 0x72, 0x69, 0x63, 0x54, 0x79, 0x70, 0x65, 0x12, 0x2c, 0x0a, 0x06, 0x6d,
    0x65, 0x74, 0x72, 0x69, 0x63, 0x18, 0x04, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x1c, 0x2e, 0x69, 0x6f,
    0x2e, 0x70, 0x72, 0x6f, 0x6d, 0x65, 0x74, 0x68, 0x65, 0x75, 0x73, 0x2e, 0x63, 0x6c, 0x69, 0x65,
    0x6e, 0x74, 0x2e, 0x4d, 0x65, 0x74, 0x72, 0x69, 0x63, 0x2a, 0x4d, 0x0a, 0x0a, 0x4d, 0x65, 0x74,
    0x72, 0x69, 0x63, 0x54, 0x79, 0x70, 0x65, 0x12, 0x0b, 0x0a, 0x07, 0x43, 0x4f, 0x55, 0x4e, 0x54,
    0x45, 0x52, 0x10, 0x00, 0x12, 0x09, 0x0a, 0x05, 0x47, 0x41, 0x55, 0x47, 0x45, 0x10, 0x01, 0x12,
    0x0b, 0x0a, 0x07, 0x53, 0x55, 0x4d, 0x4d, 0x41, 0x52, 0x59, 0x10, 0x02, 0x12, 0x0b, 0x0a, 0x07,
    0x55, 0x4e, 0x54, 0x59, 0x50, 0x45, 0x44, 0x10, 0x03, 0x12, 0x0d, 0x0a, 0x09, 0x48, 0x49, 0x53,
    0x54, 0x4f, 0x47, 0x52, 0x41, 0x4d, 0x10, 0x04, 0x42, 0x16, 0x0a, 0x14, 0x69, 0x6f, 0x2e, 0x70,
    0x72, 0x6f, 0x6d, 0x65, 0x74, 0x68, 0x65, 0x75, 0x73, 0x2e, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74,
    0x4a, 0xb4, 0x13, 0x0a, 0x06, 0x12, 0x04, 0x0d, 0x00, 0x50, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x02,
    0x12, 0x03, 0x0f, 0x08, 0x1c, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x10, 0x00, 0x2d, 0x0a,
    0x0b, 0x0a, 0x04, 0x08, 0xe7, 0x07, 0x00, 0x12, 0x03, 0x10, 0x00, 0x2d, 0x0a, 0x0c, 0x0a, 0x05,
    0x08, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x03, 0x10, 0x07, 0x13, 0x0a, 0x0d, 0x0a, 0x06, 0x08, 0xe7,
    0x07, 0x00, 0x02, 0x00, 0x12, 0x03, 0x10, 0x07, 0x13, 0x0a, 0x0e, 0x0a, 0x07, 0x08, 0xe7, 0x07,
    0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x10, 0x07, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07,
    0x00, 0x07, 0x12, 0x03, 0x10, 0x16, 0x2c, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x12,
    0x00, 0x15, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x12, 0x08, 0x11, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x13, 0x02, 0x1c, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x00, 0x04, 0x12, 0x03, 0x13, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x00, 0x05, 0x12, 0x03, 0x13, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00,
    0x01, 0x12, 0x03, 0x13, 0x12, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12,
    0x03, 0x13, 0x1a, 0x1b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x14, 0x02,
    0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x04, 0x12, 0x03, 0x14, 0x02, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x05, 0x12, 0x03, 0x14, 0x0b, 0x11, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x14, 0x12, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x14, 0x1a, 0x1b, 0x0a, 0x0a, 0x0a, 0x02, 0x05, 0x00, 0x12,
    0x04, 0x17, 0x00, 0x1d, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x05, 0x00, 0x01, 0x12, 0x03, 0x17, 0x05,
    0x0f, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x00, 0x12, 0x03, 0x18, 0x02, 0x11, 0x0a, 0x0c,
    0x0a, 0x05, 0x05, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x18, 0x02, 0x09, 0x0a, 0x0c, 0x0a, 0x05,
    0x05, 0x00, 0x02, 0x00, 0x02, 0x12, 0x03, 0x18, 0x0f, 0x10, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00,
    0x02, 0x01, 0x12, 0x03, 0x19, 0x02, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x01, 0x01,
    0x12, 0x03, 0x19, 0x02, 0x07, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x01, 0x02, 0x12, 0x03,
    0x19, 0x0f, 0x10, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x02, 0x12, 0x03, 0x1a, 0x02, 0x11,
    0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x1a, 0x02, 0x09, 0x0a, 0x0c,
    0x0a, 0x05, 0x05, 0x00, 0x02, 0x02, 0x02, 0x12, 0x03, 0x1a, 0x0f, 0x10, 0x0a, 0x0b, 0x0a, 0x04,
    0x05, 0x00, 0x02, 0x03, 0x12, 0x03, 0x1b, 0x02, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02,
    0x03, 0x01, 0x12, 0x03, 0x1b, 0x02, 0x09, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x03, 0x02,
    0x12, 0x03, 0x1b, 0x0f, 0x10, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x04, 0x12, 0x03, 0x1c,
    0x02, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x04, 0x01, 0x12, 0x03, 0x1c, 0x02, 0x0b,
    0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x04, 0x02, 0x12, 0x03, 0x1c, 0x0f, 0x10, 0x0a, 0x0a,
    0x0a, 0x02, 0x04, 0x01, 0x12, 0x04, 0x1f, 0x00, 0x21, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01,
    0x01, 0x12, 0x03, 0x1f, 0x08, 0x0d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x00, 0x12, 0x03,
    0x20, 0x02, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x04, 0x12, 0x03, 0x20, 0x02,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x05, 0x12, 0x03, 0x20, 0x0b, 0x11, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x20, 0x12, 0x17, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x00, 0x03, 0x12, 0x03, 0x20, 0x1a, 0x1b, 0x0a, 0x0a, 0x0a, 0x02, 0x04,
    0x02, 0x12, 0x04, 0x23, 0x00, 0x25, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x02, 0x01, 0x12, 0x03,
    0x23, 0x08, 0x0f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x00, 0x12, 0x03, 0x24, 0x02, 0x1c,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x04, 0x12, 0x03, 0x24, 0x02, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x05, 0x12, 0x03, 0x24, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x02, 0x02, 0x00, 0x01, 0x12, 0x03, 0x24, 0x12, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02,
    0x02, 0x00, 0x03, 0x12, 0x03, 0x24, 0x1a, 0x1b, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x03, 0x12, 0x04,
    0x27, 0x00, 0x2a, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x03, 0x01, 0x12, 0x03, 0x27, 0x08, 0x10,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x00, 0x12, 0x03, 0x28, 0x02, 0x1f, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x03, 0x02, 0x00, 0x04, 0x12, 0x03, 0x28, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x03, 0x02, 0x00, 0x05, 0x12, 0x03, 0x28, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02,
    0x00, 0x01, 0x12, 0x03, 0x28, 0x12, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x03,
    0x12, 0x03, 0x28, 0x1d, 0x1e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x01, 0x12, 0x03, 0x29,
    0x02, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x04, 0x12, 0x03, 0x29, 0x02, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x05, 0x12, 0x03, 0x29, 0x0b, 0x11, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x01, 0x12, 0x03, 0x29, 0x12, 0x17, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x03, 0x02, 0x01, 0x03, 0x12, 0x03, 0x29, 0x1d, 0x1e, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x04,
    0x12, 0x04, 0x2c, 0x00, 0x30, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x04, 0x01, 0x12, 0x03, 0x2c,
    0x08, 0x0f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x00, 0x12, 0x03, 0x2d, 0x02, 0x25, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x04, 0x12, 0x03, 0x2d, 0x02, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x04, 0x02, 0x00, 0x05, 0x12, 0x03, 0x2d, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x04, 0x02, 0x00, 0x01, 0x12, 0x03, 0x2d, 0x14, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02,
    0x00, 0x03, 0x12, 0x03, 0x2d, 0x23, 0x24, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x01, 0x12,
    0x03, 0x2e, 0x02, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x04, 0x12, 0x03, 0x2e,
    0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x05, 0x12, 0x03, 0x2e, 0x0b, 0x11,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x01, 0x12, 0x03, 0x2e, 0x14, 0x1e, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x03, 0x12, 0x03, 0x2e, 0x23, 0x24, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x04, 0x02, 0x02, 0x12, 0x03, 0x2f, 0x02, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02,
    0x02, 0x04, 0x12, 0x03, 0x2f, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x02, 0x06,
    0x12, 0x03, 0x2f, 0x0b, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x02, 0x01, 0x12, 0x03,
    0x2f, 0x14, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x02, 0x03, 0x12, 0x03, 0x2f, 0x23,
    0x24, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x05, 0x12, 0x04, 0x32, 0x00, 0x34, 0x01, 0x0a, 0x0a, 0x0a,
    0x03, 0x04, 0x05, 0x01, 0x12, 0x03, 0x32, 0x08, 0x0f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02,
    0x00, 0x12, 0x03, 0x33, 0x02, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x04, 0x12,
    0x03, 0x33, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x05, 0x12, 0x03, 0x33,
    0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x01, 0x12, 0x03, 0x33, 0x12, 0x17,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x03, 0x12, 0x03, 0x33, 0x1a, 0x1b, 0x0a, 0x0a,
    0x0a, 0x02, 0x04, 0x06, 0x12, 0x04, 0x36, 0x00, 0x3a, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x06,
    0x01, 0x12, 0x03, 0x36, 0x08, 0x11, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x00, 0x12, 0x03,
    0x37, 0x02, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x04, 0x12, 0x03, 0x37, 0x02,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x05, 0x12, 0x03, 0x37, 0x0b, 0x11, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x01, 0x12, 0x03, 0x37, 0x12, 0x1e, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x06, 0x02, 0x00, 0x03, 0x12, 0x03, 0x37, 0x21, 0x22, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x06, 0x02, 0x01, 0x12, 0x03, 0x38, 0x02, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01,
    0x04, 0x12, 0x03, 0x38, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x05, 0x12,
    0x03, 0x38, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x01, 0x12, 0x03, 0x38,
    0x12, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x03, 0x12, 0x03, 0x38, 0x21, 0x22,
    0x0a, 0x53, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x02, 0x12, 0x03, 0x39, 0x02, 0x23, 0x22, 0x46, 0x20,
    0x4f, 0x72, 0x64, 0x65, 0x72, 0x65, 0x64, 0x20, 0x69, 0x6e, 0x20, 0x69, 0x6e, 0x63, 0x72, 0x65,
    0x61, 0x73, 0x69, 0x6e, 0x67, 0x20, 0x6f, 0x72, 0x64, 0x65, 0x72, 0x20, 0x6f, 0x66, 0x20, 0x75,
    0x70, 0x70, 0x65, 0x72, 0x5f, 0x62, 0x6f, 0x75, 0x6e, 0x64, 0x2c, 0x20, 0x2b, 0x49, 0x6e, 0x66,
    0x20, 0x62, 0x75, 0x63, 0x6b, 0x65, 0x74, 0x20, 0x69, 0x73, 0x20, 0x6f, 0x70, 0x74, 0x69, 0x6f,
    0x6e, 0x61, 0x6c, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x02, 0x04, 0x12, 0x03,
    0x39, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x02, 0x06, 0x12, 0x03, 0x39, 0x0b,
    0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x02, 0x01, 0x12, 0x03, 0x39, 0x12, 0x18, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x02, 0x03, 0x12, 0x03, 0x39, 0x21, 0x22, 0x0a, 0x0a, 0x0a,
    0x02, 0x04, 0x07, 0x12, 0x04, 0x3c, 0x00, 0x3f, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x07, 0x01,
    0x12, 0x03, 0x3c, 0x08, 0x0e, 0x0a, 0x2e, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x00, 0x12, 0x03, 0x3d,
    0x02, 0x27, 0x22, 0x21, 0x20, 0x43, 0x75, 0x6d, 0x75, 0x6c, 0x61, 0x74, 0x69, 0x76, 0x65, 0x20,
    0x69, 0x6e, 0x20, 0x69, 0x6e, 0x63, 0x72, 0x65, 0x61, 0x73, 0x69, 0x6e, 0x67, 0x20, 0x6f, 0x72,
    0x64, 0x65, 0x72, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x04, 0x12, 0x03,
    0x3d, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x05, 0x12, 0x03, 0x3d, 0x0b,
    0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x01, 0x12, 0x03, 0x3d, 0x12, 0x22, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x03, 0x12, 0x03, 0x3d, 0x25, 0x26, 0x0a, 0x19, 0x0a,
    0x04, 0x04, 0x07, 0x02, 0x01, 0x12, 0x03, 0x3e, 0x02, 0x22, 0x22, 0x0c, 0x20, 0x49, 0x6e, 0x63,
    0x6c, 0x75, 0x73, 0x69, 0x76, 0x65, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x01,
    0x04, 0x12, 0x03, 0x3e, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x01, 0x05, 0x12,
    0x03, 0x3e, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x01, 0x01, 0x12, 0x03, 0x3e,
    0x12, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x01, 0x03, 0x12, 0x03, 0x3e, 0x20, 0x21,
    0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x08, 0x12, 0x04, 0x41, 0x00, 0x49, 0x01, 0x0a, 0x0a, 0x0a, 0x03,
    0x04, 0x08, 0x01, 0x12, 0x03, 0x41, 0x08, 0x0e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x08, 0x02, 0x00,
    0x12, 0x03, 0x42, 0x02, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x04, 0x12, 0x03,
    0x42, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x06, 0x12, 0x03, 0x42, 0x0b,
    0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x01, 0x12, 0x03, 0x42, 0x15, 0x1a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x03, 0x12, 0x03, 0x42, 0x24, 0x25, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x08, 0x02, 0x01, 0x12, 0x03, 0x43, 0x02, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08,
    0x02, 0x01, 0x04, 0x12, 0x03, 0x43, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x01,
    0x06, 0x12, 0x03, 0x43, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x01, 0x01, 0x12,
    0x03, 0x43, 0x15, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x01, 0x03, 0x12, 0x03, 0x43,
    0x24, 0x25, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x08, 0x02, 0x02, 0x12, 0x03, 0x44, 0x02, 0x26, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x02, 0x04, 0x12, 0x03, 0x44, 0x02, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x08, 0x02, 0x02, 0x06, 0x12, 0x03, 0x44, 0x0b, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x08, 0x02, 0x02, 0x01, 0x12, 0x03, 0x44, 0x15, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02,
    0x02, 0x03, 0x12, 0x03, 0x44, 0x24, 0x25, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x08, 0x02, 0x03, 0x12,
    0x03, 0x45, 0x02, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x03, 0x04, 0x12, 0x03, 0x45,
    0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x03, 0x06, 0x12, 0x03, 0x45, 0x0b, 0x12,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x03, 0x01, 0x12, 0x03, 0x45, 0x15, 0x1c, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x08, 0x02, 0x03, 0x03, 0x12, 0x03, 0x45, 0x24, 0x25, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x08, 0x02, 0x04, 0x12, 0x03, 0x46, 0x02, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02,
    0x04, 0x04, 0x12, 0x03, 0x46, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x04, 0x06,
    0x12, 0x03, 0x46, 0x0b, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x04, 0x01, 0x12, 0x03,
    0x46, 0x15, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x04, 0x03, 0x12, 0x03, 0x46, 0x24,
    0x25, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x08, 0x02, 0x05, 0x12, 0x03, 0x47, 0x02, 0x26, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x08, 0x02, 0x05, 0x04, 0x12, 0x03, 0x47, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x08, 0x02, 0x05, 0x06, 0x12, 0x03, 0x47, 0x0b, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08,
    0x02, 0x05, 0x01, 0x12, 0x03, 0x47, 0x15, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x05,
    0x03, 0x12, 0x03, 0x47, 0x24, 0x25, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x08, 0x02, 0x06, 0x12, 0x03,
    0x48, 0x02, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x06, 0x04, 0x12, 0x03, 0x48, 0x02,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x06, 0x05, 0x12, 0x03, 0x48, 0x0b, 0x10, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x06, 0x01, 0x12, 0x03, 0x48, 0x15, 0x21, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x08, 0x02, 0x06, 0x03, 0x12, 0x03, 0x48, 0x24, 0x25, 0x0a, 0x0a, 0x0a, 0x02, 0x04,
    0x09, 0x12, 0x04, 0x4b, 0x00, 0x50, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x09, 0x01, 0x12, 0x03,
    0x4b, 0x08, 0x14, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x00, 0x12, 0x03, 0x4c, 0x02, 0x21,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x04, 0x12, 0x03, 0x4c, 0x02, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x05, 0x12, 0x03, 0x4c, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x09, 0x02, 0x00, 0x01, 0x12, 0x03, 0x4c, 0x16, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09,
    0x02, 0x00, 0x03, 0x12, 0x03, 0x4c, 0x1f, 0x20, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x01,
    0x12, 0x03, 0x4d, 0x02, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x01, 0x04, 0x12, 0x03,
    0x4d, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x01, 0x05, 0x12, 0x03, 0x4d, 0x0b,
    0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x01, 0x01, 0x12, 0x03, 0x4d, 0x16, 0x1a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x01, 0x03, 0x12, 0x03, 0x4d, 0x1f, 0x20, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x09, 0x02, 0x02, 0x12, 0x03, 0x4e, 0x02, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09,
    0x02, 0x02, 0x04, 0x12, 0x03, 0x4e, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x02,
    0x06, 0x12, 0x03, 0x4e, 0x0b, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x02, 0x01, 0x12,
    0x03, 0x4e, 0x16, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x02, 0x03, 0x12, 0x03, 0x4e,
    0x1f, 0x20, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x03, 0x12, 0x03, 0x4f, 0x02, 0x21, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x03, 0x04, 0x12, 0x03, 0x4f, 0x02, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x09, 0x02, 0x03, 0x06, 0x12, 0x03, 0x4f, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x09, 0x02, 0x03, 0x01, 0x12, 0x03, 0x4f, 0x16, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02,
    0x03, 0x03, 0x12, 0x03, 0x4f, 0x1f, 0x20,
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
