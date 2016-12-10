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
pub struct DiffEntry {
    // message oneof groups
    kind: ::std::option::Option<DiffEntry_oneof_kind>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DiffEntry {}

#[derive(Clone,PartialEq)]
pub enum DiffEntry_oneof_kind {
    put(DiffEntry_Put),
    delete(DiffEntry_Delete),
    patch(DiffEntry_Patch),
    header(DiffEntry_Header),
}

impl DiffEntry {
    pub fn new() -> DiffEntry {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DiffEntry {
        static mut instance: ::protobuf::lazy::Lazy<DiffEntry> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DiffEntry,
        };
        unsafe {
            instance.get(|| {
                DiffEntry {
                    kind: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .spotify.sparkey.DiffEntry.Put put = 1;

    pub fn clear_put(&mut self) {
        self.kind = ::std::option::Option::None;
    }

    pub fn has_put(&self) -> bool {
        match self.kind {
            ::std::option::Option::Some(DiffEntry_oneof_kind::put(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_put(&mut self, v: DiffEntry_Put) {
        self.kind = ::std::option::Option::Some(DiffEntry_oneof_kind::put(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_put(&mut self) -> &mut DiffEntry_Put {
        if let ::std::option::Option::Some(DiffEntry_oneof_kind::put(_)) = self.kind {
        } else {
            self.kind = ::std::option::Option::Some(DiffEntry_oneof_kind::put(DiffEntry_Put::new()));
        }
        match self.kind {
            ::std::option::Option::Some(DiffEntry_oneof_kind::put(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_put(&mut self) -> DiffEntry_Put {
        if self.has_put() {
            match self.kind.take() {
                ::std::option::Option::Some(DiffEntry_oneof_kind::put(v)) => v,
                _ => panic!(),
            }
        } else {
            DiffEntry_Put::new()
        }
    }

    pub fn get_put(&self) -> &DiffEntry_Put {
        match self.kind {
            ::std::option::Option::Some(DiffEntry_oneof_kind::put(ref v)) => v,
            _ => DiffEntry_Put::default_instance(),
        }
    }

    // optional .spotify.sparkey.DiffEntry.Delete delete = 2;

    pub fn clear_delete(&mut self) {
        self.kind = ::std::option::Option::None;
    }

    pub fn has_delete(&self) -> bool {
        match self.kind {
            ::std::option::Option::Some(DiffEntry_oneof_kind::delete(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_delete(&mut self, v: DiffEntry_Delete) {
        self.kind = ::std::option::Option::Some(DiffEntry_oneof_kind::delete(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_delete(&mut self) -> &mut DiffEntry_Delete {
        if let ::std::option::Option::Some(DiffEntry_oneof_kind::delete(_)) = self.kind {
        } else {
            self.kind = ::std::option::Option::Some(DiffEntry_oneof_kind::delete(DiffEntry_Delete::new()));
        }
        match self.kind {
            ::std::option::Option::Some(DiffEntry_oneof_kind::delete(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_delete(&mut self) -> DiffEntry_Delete {
        if self.has_delete() {
            match self.kind.take() {
                ::std::option::Option::Some(DiffEntry_oneof_kind::delete(v)) => v,
                _ => panic!(),
            }
        } else {
            DiffEntry_Delete::new()
        }
    }

    pub fn get_delete(&self) -> &DiffEntry_Delete {
        match self.kind {
            ::std::option::Option::Some(DiffEntry_oneof_kind::delete(ref v)) => v,
            _ => DiffEntry_Delete::default_instance(),
        }
    }

    // optional .spotify.sparkey.DiffEntry.Patch patch = 3;

    pub fn clear_patch(&mut self) {
        self.kind = ::std::option::Option::None;
    }

    pub fn has_patch(&self) -> bool {
        match self.kind {
            ::std::option::Option::Some(DiffEntry_oneof_kind::patch(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_patch(&mut self, v: DiffEntry_Patch) {
        self.kind = ::std::option::Option::Some(DiffEntry_oneof_kind::patch(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_patch(&mut self) -> &mut DiffEntry_Patch {
        if let ::std::option::Option::Some(DiffEntry_oneof_kind::patch(_)) = self.kind {
        } else {
            self.kind = ::std::option::Option::Some(DiffEntry_oneof_kind::patch(DiffEntry_Patch::new()));
        }
        match self.kind {
            ::std::option::Option::Some(DiffEntry_oneof_kind::patch(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_patch(&mut self) -> DiffEntry_Patch {
        if self.has_patch() {
            match self.kind.take() {
                ::std::option::Option::Some(DiffEntry_oneof_kind::patch(v)) => v,
                _ => panic!(),
            }
        } else {
            DiffEntry_Patch::new()
        }
    }

    pub fn get_patch(&self) -> &DiffEntry_Patch {
        match self.kind {
            ::std::option::Option::Some(DiffEntry_oneof_kind::patch(ref v)) => v,
            _ => DiffEntry_Patch::default_instance(),
        }
    }

    // optional .spotify.sparkey.DiffEntry.Header header = 4;

    pub fn clear_header(&mut self) {
        self.kind = ::std::option::Option::None;
    }

    pub fn has_header(&self) -> bool {
        match self.kind {
            ::std::option::Option::Some(DiffEntry_oneof_kind::header(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_header(&mut self, v: DiffEntry_Header) {
        self.kind = ::std::option::Option::Some(DiffEntry_oneof_kind::header(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_header(&mut self) -> &mut DiffEntry_Header {
        if let ::std::option::Option::Some(DiffEntry_oneof_kind::header(_)) = self.kind {
        } else {
            self.kind = ::std::option::Option::Some(DiffEntry_oneof_kind::header(DiffEntry_Header::new()));
        }
        match self.kind {
            ::std::option::Option::Some(DiffEntry_oneof_kind::header(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_header(&mut self) -> DiffEntry_Header {
        if self.has_header() {
            match self.kind.take() {
                ::std::option::Option::Some(DiffEntry_oneof_kind::header(v)) => v,
                _ => panic!(),
            }
        } else {
            DiffEntry_Header::new()
        }
    }

    pub fn get_header(&self) -> &DiffEntry_Header {
        match self.kind {
            ::std::option::Option::Some(DiffEntry_oneof_kind::header(ref v)) => v,
            _ => DiffEntry_Header::default_instance(),
        }
    }
}

impl ::protobuf::Message for DiffEntry {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.kind = ::std::option::Option::Some(DiffEntry_oneof_kind::put(try!(is.read_message())));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.kind = ::std::option::Option::Some(DiffEntry_oneof_kind::delete(try!(is.read_message())));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.kind = ::std::option::Option::Some(DiffEntry_oneof_kind::patch(try!(is.read_message())));
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.kind = ::std::option::Option::Some(DiffEntry_oneof_kind::header(try!(is.read_message())));
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
        if let ::std::option::Option::Some(ref v) = self.kind {
            match v {
                &DiffEntry_oneof_kind::put(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &DiffEntry_oneof_kind::delete(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &DiffEntry_oneof_kind::patch(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &DiffEntry_oneof_kind::header(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
            };
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let ::std::option::Option::Some(ref v) = self.kind {
            match v {
                &DiffEntry_oneof_kind::put(ref v) => {
                    try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
                &DiffEntry_oneof_kind::delete(ref v) => {
                    try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
                &DiffEntry_oneof_kind::patch(ref v) => {
                    try!(os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
                &DiffEntry_oneof_kind::header(ref v) => {
                    try!(os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
            };
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
        ::std::any::TypeId::of::<DiffEntry>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for DiffEntry {
    fn new() -> DiffEntry {
        DiffEntry::new()
    }

    fn descriptor_static(_: ::std::option::Option<DiffEntry>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "put",
                    DiffEntry::has_put,
                    DiffEntry::get_put,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "delete",
                    DiffEntry::has_delete,
                    DiffEntry::get_delete,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "patch",
                    DiffEntry::has_patch,
                    DiffEntry::get_patch,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "header",
                    DiffEntry::has_header,
                    DiffEntry::get_header,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DiffEntry>(
                    "DiffEntry",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DiffEntry {
    fn clear(&mut self) {
        self.clear_put();
        self.clear_delete();
        self.clear_patch();
        self.clear_header();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for DiffEntry {
    fn eq(&self, other: &DiffEntry) -> bool {
        self.kind == other.kind &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for DiffEntry {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct DiffEntry_Header {
    // message fields
    num_entries: ::std::option::Option<u64>,
    num_puts: ::std::option::Option<u64>,
    num_deletes: ::std::option::Option<u64>,
    num_patches: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DiffEntry_Header {}

impl DiffEntry_Header {
    pub fn new() -> DiffEntry_Header {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DiffEntry_Header {
        static mut instance: ::protobuf::lazy::Lazy<DiffEntry_Header> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DiffEntry_Header,
        };
        unsafe {
            instance.get(|| {
                DiffEntry_Header {
                    num_entries: ::std::option::Option::None,
                    num_puts: ::std::option::Option::None,
                    num_deletes: ::std::option::Option::None,
                    num_patches: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional fixed64 num_entries = 1;

    pub fn clear_num_entries(&mut self) {
        self.num_entries = ::std::option::Option::None;
    }

    pub fn has_num_entries(&self) -> bool {
        self.num_entries.is_some()
    }

    // Param is passed by value, moved
    pub fn set_num_entries(&mut self, v: u64) {
        self.num_entries = ::std::option::Option::Some(v);
    }

    pub fn get_num_entries(&self) -> u64 {
        self.num_entries.unwrap_or(0)
    }

    // optional fixed64 num_puts = 2;

    pub fn clear_num_puts(&mut self) {
        self.num_puts = ::std::option::Option::None;
    }

    pub fn has_num_puts(&self) -> bool {
        self.num_puts.is_some()
    }

    // Param is passed by value, moved
    pub fn set_num_puts(&mut self, v: u64) {
        self.num_puts = ::std::option::Option::Some(v);
    }

    pub fn get_num_puts(&self) -> u64 {
        self.num_puts.unwrap_or(0)
    }

    // optional fixed64 num_deletes = 3;

    pub fn clear_num_deletes(&mut self) {
        self.num_deletes = ::std::option::Option::None;
    }

    pub fn has_num_deletes(&self) -> bool {
        self.num_deletes.is_some()
    }

    // Param is passed by value, moved
    pub fn set_num_deletes(&mut self, v: u64) {
        self.num_deletes = ::std::option::Option::Some(v);
    }

    pub fn get_num_deletes(&self) -> u64 {
        self.num_deletes.unwrap_or(0)
    }

    // optional fixed64 num_patches = 4;

    pub fn clear_num_patches(&mut self) {
        self.num_patches = ::std::option::Option::None;
    }

    pub fn has_num_patches(&self) -> bool {
        self.num_patches.is_some()
    }

    // Param is passed by value, moved
    pub fn set_num_patches(&mut self, v: u64) {
        self.num_patches = ::std::option::Option::Some(v);
    }

    pub fn get_num_patches(&self) -> u64 {
        self.num_patches.unwrap_or(0)
    }
}

impl ::protobuf::Message for DiffEntry_Header {
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
                    let tmp = try!(is.read_fixed64());
                    self.num_entries = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_fixed64());
                    self.num_puts = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_fixed64());
                    self.num_deletes = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_fixed64());
                    self.num_patches = ::std::option::Option::Some(tmp);
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
        if self.num_entries.is_some() {
            my_size += 9;
        };
        if self.num_puts.is_some() {
            my_size += 9;
        };
        if self.num_deletes.is_some() {
            my_size += 9;
        };
        if self.num_patches.is_some() {
            my_size += 9;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.num_entries {
            try!(os.write_fixed64(1, v));
        };
        if let Some(v) = self.num_puts {
            try!(os.write_fixed64(2, v));
        };
        if let Some(v) = self.num_deletes {
            try!(os.write_fixed64(3, v));
        };
        if let Some(v) = self.num_patches {
            try!(os.write_fixed64(4, v));
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
        ::std::any::TypeId::of::<DiffEntry_Header>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for DiffEntry_Header {
    fn new() -> DiffEntry_Header {
        DiffEntry_Header::new()
    }

    fn descriptor_static(_: ::std::option::Option<DiffEntry_Header>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "num_entries",
                    DiffEntry_Header::has_num_entries,
                    DiffEntry_Header::get_num_entries,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "num_puts",
                    DiffEntry_Header::has_num_puts,
                    DiffEntry_Header::get_num_puts,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "num_deletes",
                    DiffEntry_Header::has_num_deletes,
                    DiffEntry_Header::get_num_deletes,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "num_patches",
                    DiffEntry_Header::has_num_patches,
                    DiffEntry_Header::get_num_patches,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DiffEntry_Header>(
                    "DiffEntry_Header",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DiffEntry_Header {
    fn clear(&mut self) {
        self.clear_num_entries();
        self.clear_num_puts();
        self.clear_num_deletes();
        self.clear_num_patches();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for DiffEntry_Header {
    fn eq(&self, other: &DiffEntry_Header) -> bool {
        self.num_entries == other.num_entries &&
        self.num_puts == other.num_puts &&
        self.num_deletes == other.num_deletes &&
        self.num_patches == other.num_patches &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for DiffEntry_Header {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct DiffEntry_Put {
    // message fields
    key: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    value: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DiffEntry_Put {}

impl DiffEntry_Put {
    pub fn new() -> DiffEntry_Put {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DiffEntry_Put {
        static mut instance: ::protobuf::lazy::Lazy<DiffEntry_Put> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DiffEntry_Put,
        };
        unsafe {
            instance.get(|| {
                DiffEntry_Put {
                    key: ::protobuf::SingularField::none(),
                    value: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional bytes key = 1;

    pub fn clear_key(&mut self) {
        self.key.clear();
    }

    pub fn has_key(&self) -> bool {
        self.key.is_some()
    }

    // Param is passed by value, moved
    pub fn set_key(&mut self, v: ::std::vec::Vec<u8>) {
        self.key = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_key(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.key.is_none() {
            self.key.set_default();
        };
        self.key.as_mut().unwrap()
    }

    // Take field
    pub fn take_key(&mut self) -> ::std::vec::Vec<u8> {
        self.key.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_key(&self) -> &[u8] {
        match self.key.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    // optional bytes value = 2;

    pub fn clear_value(&mut self) {
        self.value.clear();
    }

    pub fn has_value(&self) -> bool {
        self.value.is_some()
    }

    // Param is passed by value, moved
    pub fn set_value(&mut self, v: ::std::vec::Vec<u8>) {
        self.value = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_value(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.value.is_none() {
            self.value.set_default();
        };
        self.value.as_mut().unwrap()
    }

    // Take field
    pub fn take_value(&mut self) -> ::std::vec::Vec<u8> {
        self.value.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_value(&self) -> &[u8] {
        match self.value.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }
}

impl ::protobuf::Message for DiffEntry_Put {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.key));
                },
                2 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.value));
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
        for value in &self.key {
            my_size += ::protobuf::rt::bytes_size(1, &value);
        };
        for value in &self.value {
            my_size += ::protobuf::rt::bytes_size(2, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.key.as_ref() {
            try!(os.write_bytes(1, &v));
        };
        if let Some(v) = self.value.as_ref() {
            try!(os.write_bytes(2, &v));
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
        ::std::any::TypeId::of::<DiffEntry_Put>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for DiffEntry_Put {
    fn new() -> DiffEntry_Put {
        DiffEntry_Put::new()
    }

    fn descriptor_static(_: ::std::option::Option<DiffEntry_Put>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "key",
                    DiffEntry_Put::has_key,
                    DiffEntry_Put::get_key,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "value",
                    DiffEntry_Put::has_value,
                    DiffEntry_Put::get_value,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DiffEntry_Put>(
                    "DiffEntry_Put",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DiffEntry_Put {
    fn clear(&mut self) {
        self.clear_key();
        self.clear_value();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for DiffEntry_Put {
    fn eq(&self, other: &DiffEntry_Put) -> bool {
        self.key == other.key &&
        self.value == other.value &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for DiffEntry_Put {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct DiffEntry_Delete {
    // message fields
    key: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DiffEntry_Delete {}

impl DiffEntry_Delete {
    pub fn new() -> DiffEntry_Delete {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DiffEntry_Delete {
        static mut instance: ::protobuf::lazy::Lazy<DiffEntry_Delete> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DiffEntry_Delete,
        };
        unsafe {
            instance.get(|| {
                DiffEntry_Delete {
                    key: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional bytes key = 1;

    pub fn clear_key(&mut self) {
        self.key.clear();
    }

    pub fn has_key(&self) -> bool {
        self.key.is_some()
    }

    // Param is passed by value, moved
    pub fn set_key(&mut self, v: ::std::vec::Vec<u8>) {
        self.key = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_key(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.key.is_none() {
            self.key.set_default();
        };
        self.key.as_mut().unwrap()
    }

    // Take field
    pub fn take_key(&mut self) -> ::std::vec::Vec<u8> {
        self.key.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_key(&self) -> &[u8] {
        match self.key.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }
}

impl ::protobuf::Message for DiffEntry_Delete {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.key));
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
        for value in &self.key {
            my_size += ::protobuf::rt::bytes_size(1, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.key.as_ref() {
            try!(os.write_bytes(1, &v));
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
        ::std::any::TypeId::of::<DiffEntry_Delete>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for DiffEntry_Delete {
    fn new() -> DiffEntry_Delete {
        DiffEntry_Delete::new()
    }

    fn descriptor_static(_: ::std::option::Option<DiffEntry_Delete>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "key",
                    DiffEntry_Delete::has_key,
                    DiffEntry_Delete::get_key,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DiffEntry_Delete>(
                    "DiffEntry_Delete",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DiffEntry_Delete {
    fn clear(&mut self) {
        self.clear_key();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for DiffEntry_Delete {
    fn eq(&self, other: &DiffEntry_Delete) -> bool {
        self.key == other.key &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for DiffEntry_Delete {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct DiffEntry_Patch {
    // message fields
    key: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    delta: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DiffEntry_Patch {}

impl DiffEntry_Patch {
    pub fn new() -> DiffEntry_Patch {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DiffEntry_Patch {
        static mut instance: ::protobuf::lazy::Lazy<DiffEntry_Patch> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DiffEntry_Patch,
        };
        unsafe {
            instance.get(|| {
                DiffEntry_Patch {
                    key: ::protobuf::SingularField::none(),
                    delta: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional bytes key = 1;

    pub fn clear_key(&mut self) {
        self.key.clear();
    }

    pub fn has_key(&self) -> bool {
        self.key.is_some()
    }

    // Param is passed by value, moved
    pub fn set_key(&mut self, v: ::std::vec::Vec<u8>) {
        self.key = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_key(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.key.is_none() {
            self.key.set_default();
        };
        self.key.as_mut().unwrap()
    }

    // Take field
    pub fn take_key(&mut self) -> ::std::vec::Vec<u8> {
        self.key.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_key(&self) -> &[u8] {
        match self.key.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    // optional bytes delta = 2;

    pub fn clear_delta(&mut self) {
        self.delta.clear();
    }

    pub fn has_delta(&self) -> bool {
        self.delta.is_some()
    }

    // Param is passed by value, moved
    pub fn set_delta(&mut self, v: ::std::vec::Vec<u8>) {
        self.delta = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_delta(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.delta.is_none() {
            self.delta.set_default();
        };
        self.delta.as_mut().unwrap()
    }

    // Take field
    pub fn take_delta(&mut self) -> ::std::vec::Vec<u8> {
        self.delta.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_delta(&self) -> &[u8] {
        match self.delta.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }
}

impl ::protobuf::Message for DiffEntry_Patch {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.key));
                },
                2 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.delta));
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
        for value in &self.key {
            my_size += ::protobuf::rt::bytes_size(1, &value);
        };
        for value in &self.delta {
            my_size += ::protobuf::rt::bytes_size(2, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.key.as_ref() {
            try!(os.write_bytes(1, &v));
        };
        if let Some(v) = self.delta.as_ref() {
            try!(os.write_bytes(2, &v));
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
        ::std::any::TypeId::of::<DiffEntry_Patch>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for DiffEntry_Patch {
    fn new() -> DiffEntry_Patch {
        DiffEntry_Patch::new()
    }

    fn descriptor_static(_: ::std::option::Option<DiffEntry_Patch>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "key",
                    DiffEntry_Patch::has_key,
                    DiffEntry_Patch::get_key,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "delta",
                    DiffEntry_Patch::has_delta,
                    DiffEntry_Patch::get_delta,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DiffEntry_Patch>(
                    "DiffEntry_Patch",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DiffEntry_Patch {
    fn clear(&mut self) {
        self.clear_key();
        self.clear_delta();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for DiffEntry_Patch {
    fn eq(&self, other: &DiffEntry_Patch) -> bool {
        self.key == other.key &&
        self.delta == other.delta &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for DiffEntry_Patch {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x0e, 0x73, 0x72, 0x63, 0x2f, 0x64, 0x69, 0x66, 0x66, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f,
    0x12, 0x0f, 0x73, 0x70, 0x6f, 0x74, 0x69, 0x66, 0x79, 0x2e, 0x73, 0x70, 0x61, 0x72, 0x6b, 0x65,
    0x79, 0x22, 0x80, 0x04, 0x0a, 0x09, 0x44, 0x69, 0x66, 0x66, 0x45, 0x6e, 0x74, 0x72, 0x79, 0x12,
    0x32, 0x0a, 0x03, 0x70, 0x75, 0x74, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1e, 0x2e, 0x73,
    0x70, 0x6f, 0x74, 0x69, 0x66, 0x79, 0x2e, 0x73, 0x70, 0x61, 0x72, 0x6b, 0x65, 0x79, 0x2e, 0x44,
    0x69, 0x66, 0x66, 0x45, 0x6e, 0x74, 0x72, 0x79, 0x2e, 0x50, 0x75, 0x74, 0x48, 0x00, 0x52, 0x03,
    0x70, 0x75, 0x74, 0x12, 0x3b, 0x0a, 0x06, 0x64, 0x65, 0x6c, 0x65, 0x74, 0x65, 0x18, 0x02, 0x20,
    0x01, 0x28, 0x0b, 0x32, 0x21, 0x2e, 0x73, 0x70, 0x6f, 0x74, 0x69, 0x66, 0x79, 0x2e, 0x73, 0x70,
    0x61, 0x72, 0x6b, 0x65, 0x79, 0x2e, 0x44, 0x69, 0x66, 0x66, 0x45, 0x6e, 0x74, 0x72, 0x79, 0x2e,
    0x44, 0x65, 0x6c, 0x65, 0x74, 0x65, 0x48, 0x00, 0x52, 0x06, 0x64, 0x65, 0x6c, 0x65, 0x74, 0x65,
    0x12, 0x38, 0x0a, 0x05, 0x70, 0x61, 0x74, 0x63, 0x68, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0b, 0x32,
    0x20, 0x2e, 0x73, 0x70, 0x6f, 0x74, 0x69, 0x66, 0x79, 0x2e, 0x73, 0x70, 0x61, 0x72, 0x6b, 0x65,
    0x79, 0x2e, 0x44, 0x69, 0x66, 0x66, 0x45, 0x6e, 0x74, 0x72, 0x79, 0x2e, 0x50, 0x61, 0x74, 0x63,
    0x68, 0x48, 0x00, 0x52, 0x05, 0x70, 0x61, 0x74, 0x63, 0x68, 0x12, 0x3b, 0x0a, 0x06, 0x68, 0x65,
    0x61, 0x64, 0x65, 0x72, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x21, 0x2e, 0x73, 0x70, 0x6f,
    0x74, 0x69, 0x66, 0x79, 0x2e, 0x73, 0x70, 0x61, 0x72, 0x6b, 0x65, 0x79, 0x2e, 0x44, 0x69, 0x66,
    0x66, 0x45, 0x6e, 0x74, 0x72, 0x79, 0x2e, 0x48, 0x65, 0x61, 0x64, 0x65, 0x72, 0x48, 0x00, 0x52,
    0x06, 0x68, 0x65, 0x61, 0x64, 0x65, 0x72, 0x1a, 0x86, 0x01, 0x0a, 0x06, 0x48, 0x65, 0x61, 0x64,
    0x65, 0x72, 0x12, 0x1f, 0x0a, 0x0b, 0x6e, 0x75, 0x6d, 0x5f, 0x65, 0x6e, 0x74, 0x72, 0x69, 0x65,
    0x73, 0x18, 0x01, 0x20, 0x01, 0x28, 0x06, 0x52, 0x0a, 0x6e, 0x75, 0x6d, 0x45, 0x6e, 0x74, 0x72,
    0x69, 0x65, 0x73, 0x12, 0x19, 0x0a, 0x08, 0x6e, 0x75, 0x6d, 0x5f, 0x70, 0x75, 0x74, 0x73, 0x18,
    0x02, 0x20, 0x01, 0x28, 0x06, 0x52, 0x07, 0x6e, 0x75, 0x6d, 0x50, 0x75, 0x74, 0x73, 0x12, 0x1f,
    0x0a, 0x0b, 0x6e, 0x75, 0x6d, 0x5f, 0x64, 0x65, 0x6c, 0x65, 0x74, 0x65, 0x73, 0x18, 0x03, 0x20,
    0x01, 0x28, 0x06, 0x52, 0x0a, 0x6e, 0x75, 0x6d, 0x44, 0x65, 0x6c, 0x65, 0x74, 0x65, 0x73, 0x12,
    0x1f, 0x0a, 0x0b, 0x6e, 0x75, 0x6d, 0x5f, 0x70, 0x61, 0x74, 0x63, 0x68, 0x65, 0x73, 0x18, 0x04,
    0x20, 0x01, 0x28, 0x06, 0x52, 0x0a, 0x6e, 0x75, 0x6d, 0x50, 0x61, 0x74, 0x63, 0x68, 0x65, 0x73,
    0x1a, 0x2d, 0x0a, 0x03, 0x50, 0x75, 0x74, 0x12, 0x10, 0x0a, 0x03, 0x6b, 0x65, 0x79, 0x18, 0x01,
    0x20, 0x01, 0x28, 0x0c, 0x52, 0x03, 0x6b, 0x65, 0x79, 0x12, 0x14, 0x0a, 0x05, 0x76, 0x61, 0x6c,
    0x75, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x05, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x1a,
    0x1a, 0x0a, 0x06, 0x44, 0x65, 0x6c, 0x65, 0x74, 0x65, 0x12, 0x10, 0x0a, 0x03, 0x6b, 0x65, 0x79,
    0x18, 0x01, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x03, 0x6b, 0x65, 0x79, 0x1a, 0x2f, 0x0a, 0x05, 0x50,
    0x61, 0x74, 0x63, 0x68, 0x12, 0x10, 0x0a, 0x03, 0x6b, 0x65, 0x79, 0x18, 0x01, 0x20, 0x01, 0x28,
    0x0c, 0x52, 0x03, 0x6b, 0x65, 0x79, 0x12, 0x14, 0x0a, 0x05, 0x64, 0x65, 0x6c, 0x74, 0x61, 0x18,
    0x02, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x05, 0x64, 0x65, 0x6c, 0x74, 0x61, 0x42, 0x06, 0x0a, 0x04,
    0x6b, 0x69, 0x6e, 0x64, 0x4a, 0xa6, 0x09, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x21, 0x01, 0x0a,
    0x08, 0x0a, 0x01, 0x0c, 0x12, 0x03, 0x00, 0x00, 0x12, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03,
    0x02, 0x08, 0x17, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x04, 0x00, 0x21, 0x01, 0x0a,
    0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x04, 0x08, 0x11, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x00, 0x08, 0x00, 0x12, 0x04, 0x05, 0x04, 0x0a, 0x05, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x08,
    0x00, 0x01, 0x12, 0x03, 0x05, 0x0a, 0x0e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12,
    0x03, 0x06, 0x08, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x06, 0x12, 0x03, 0x06,
    0x08, 0x0b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x06, 0x0c, 0x0f,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x06, 0x12, 0x13, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x07, 0x08, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x01, 0x06, 0x12, 0x03, 0x07, 0x08, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x01, 0x01, 0x12, 0x03, 0x07, 0x0f, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03,
    0x12, 0x03, 0x07, 0x18, 0x19, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x08,
    0x08, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x06, 0x12, 0x03, 0x08, 0x08, 0x0d,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x08, 0x0e, 0x13, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x08, 0x16, 0x17, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x00, 0x02, 0x03, 0x12, 0x03, 0x09, 0x08, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x03, 0x06, 0x12, 0x03, 0x09, 0x08, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x01,
    0x12, 0x03, 0x09, 0x0f, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x03, 0x12, 0x03,
    0x09, 0x18, 0x19, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x00, 0x03, 0x00, 0x12, 0x04, 0x0c, 0x04, 0x12,
    0x05, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x03, 0x00, 0x01, 0x12, 0x03, 0x0c, 0x0c, 0x12, 0x0a,
    0x47, 0x0a, 0x06, 0x04, 0x00, 0x03, 0x00, 0x02, 0x00, 0x12, 0x03, 0x0e, 0x08, 0x20, 0x1a, 0x38,
    0x20, 0x4e, 0x4f, 0x54, 0x45, 0x3a, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x6d, 0x65, 0x73, 0x73,
    0x61, 0x67, 0x65, 0x20, 0x6d, 0x75, 0x73, 0x74, 0x20, 0x6f, 0x6e, 0x6c, 0x79, 0x20, 0x63, 0x6f,
    0x6e, 0x74, 0x61, 0x69, 0x6e, 0x20, 0x66, 0x69, 0x78, 0x65, 0x64, 0x2d, 0x73, 0x69, 0x7a, 0x65,
    0x20, 0x74, 0x79, 0x70, 0x65, 0x73, 0x21, 0x0a, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x00,
    0x02, 0x00, 0x04, 0x12, 0x04, 0x0e, 0x08, 0x0c, 0x14, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03,
    0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x0e, 0x08, 0x0f, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03,
    0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0e, 0x10, 0x1b, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03,
    0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x0e, 0x1e, 0x1f, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x00, 0x03,
    0x00, 0x02, 0x01, 0x12, 0x03, 0x0f, 0x08, 0x1d, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x00,
    0x02, 0x01, 0x04, 0x12, 0x04, 0x0f, 0x08, 0x0e, 0x20, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03,
    0x00, 0x02, 0x01, 0x05, 0x12, 0x03, 0x0f, 0x08, 0x0f, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03,
    0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x0f, 0x10, 0x18, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03,
    0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x0f, 0x1b, 0x1c, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x00, 0x03,
    0x00, 0x02, 0x02, 0x12, 0x03, 0x10, 0x08, 0x20, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x00,
    0x02, 0x02, 0x04, 0x12, 0x04, 0x10, 0x08, 0x0f, 0x1d, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03,
    0x00, 0x02, 0x02, 0x05, 0x12, 0x03, 0x10, 0x08, 0x0f, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03,
    0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x10, 0x10, 0x1b, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03,
    0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x10, 0x1e, 0x1f, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x00, 0x03,
    0x00, 0x02, 0x03, 0x12, 0x03, 0x11, 0x08, 0x20, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x00,
    0x02, 0x03, 0x04, 0x12, 0x04, 0x11, 0x08, 0x10, 0x20, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03,
    0x00, 0x02, 0x03, 0x05, 0x12, 0x03, 0x11, 0x08, 0x0f, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03,
    0x00, 0x02, 0x03, 0x01, 0x12, 0x03, 0x11, 0x10, 0x1b, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03,
    0x00, 0x02, 0x03, 0x03, 0x12, 0x03, 0x11, 0x1e, 0x1f, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x00, 0x03,
    0x01, 0x12, 0x04, 0x14, 0x04, 0x17, 0x05, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x03, 0x01, 0x01,
    0x12, 0x03, 0x14, 0x0c, 0x0f, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x00, 0x03, 0x01, 0x02, 0x00, 0x12,
    0x03, 0x15, 0x08, 0x16, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x01, 0x02, 0x00, 0x04, 0x12,
    0x04, 0x15, 0x08, 0x14, 0x11, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x01, 0x02, 0x00, 0x05,
    0x12, 0x03, 0x15, 0x08, 0x0d, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x01, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x15, 0x0e, 0x11, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x01, 0x02, 0x00, 0x03,
    0x12, 0x03, 0x15, 0x14, 0x15, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x00, 0x03, 0x01, 0x02, 0x01, 0x12,
    0x03, 0x16, 0x08, 0x18, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x01, 0x02, 0x01, 0x04, 0x12,
    0x04, 0x16, 0x08, 0x15, 0x16, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x01, 0x02, 0x01, 0x05,
    0x12, 0x03, 0x16, 0x08, 0x0d, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x01, 0x02, 0x01, 0x01,
    0x12, 0x03, 0x16, 0x0e, 0x13, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x01, 0x02, 0x01, 0x03,
    0x12, 0x03, 0x16, 0x16, 0x17, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x00, 0x03, 0x02, 0x12, 0x04, 0x19,
    0x04, 0x1b, 0x05, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x03, 0x02, 0x01, 0x12, 0x03, 0x19, 0x0c,
    0x12, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x00, 0x03, 0x02, 0x02, 0x00, 0x12, 0x03, 0x1a, 0x08, 0x16,
    0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x02, 0x02, 0x00, 0x04, 0x12, 0x04, 0x1a, 0x08, 0x19,
    0x14, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x02, 0x02, 0x00, 0x05, 0x12, 0x03, 0x1a, 0x08,
    0x0d, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x02, 0x02, 0x00, 0x01, 0x12, 0x03, 0x1a, 0x0e,
    0x11, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x02, 0x02, 0x00, 0x03, 0x12, 0x03, 0x1a, 0x14,
    0x15, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x00, 0x03, 0x03, 0x12, 0x04, 0x1d, 0x04, 0x20, 0x05, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x03, 0x03, 0x01, 0x12, 0x03, 0x1d, 0x0c, 0x11, 0x0a, 0x0d, 0x0a,
    0x06, 0x04, 0x00, 0x03, 0x03, 0x02, 0x00, 0x12, 0x03, 0x1e, 0x08, 0x16, 0x0a, 0x0f, 0x0a, 0x07,
    0x04, 0x00, 0x03, 0x03, 0x02, 0x00, 0x04, 0x12, 0x04, 0x1e, 0x08, 0x1d, 0x13, 0x0a, 0x0e, 0x0a,
    0x07, 0x04, 0x00, 0x03, 0x03, 0x02, 0x00, 0x05, 0x12, 0x03, 0x1e, 0x08, 0x0d, 0x0a, 0x0e, 0x0a,
    0x07, 0x04, 0x00, 0x03, 0x03, 0x02, 0x00, 0x01, 0x12, 0x03, 0x1e, 0x0e, 0x11, 0x0a, 0x0e, 0x0a,
    0x07, 0x04, 0x00, 0x03, 0x03, 0x02, 0x00, 0x03, 0x12, 0x03, 0x1e, 0x14, 0x15, 0x0a, 0x0d, 0x0a,
    0x06, 0x04, 0x00, 0x03, 0x03, 0x02, 0x01, 0x12, 0x03, 0x1f, 0x08, 0x18, 0x0a, 0x0f, 0x0a, 0x07,
    0x04, 0x00, 0x03, 0x03, 0x02, 0x01, 0x04, 0x12, 0x04, 0x1f, 0x08, 0x1e, 0x16, 0x0a, 0x0e, 0x0a,
    0x07, 0x04, 0x00, 0x03, 0x03, 0x02, 0x01, 0x05, 0x12, 0x03, 0x1f, 0x08, 0x0d, 0x0a, 0x0e, 0x0a,
    0x07, 0x04, 0x00, 0x03, 0x03, 0x02, 0x01, 0x01, 0x12, 0x03, 0x1f, 0x0e, 0x13, 0x0a, 0x0e, 0x0a,
    0x07, 0x04, 0x00, 0x03, 0x03, 0x02, 0x01, 0x03, 0x12, 0x03, 0x1f, 0x16, 0x17, 0x62, 0x06, 0x70,
    0x72, 0x6f, 0x74, 0x6f, 0x33,
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
