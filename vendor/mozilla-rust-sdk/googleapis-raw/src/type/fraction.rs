// This file is generated by rust-protobuf 2.24.1. Do not edit
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![cfg_attr(rustfmt, rustfmt::skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_imports)]
#![allow(unused_results)]
//! Generated file from `google/type/fraction.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_24_1;

#[derive(PartialEq,Clone,Default)]
pub struct Fraction {
    // message fields
    pub numerator: i64,
    pub denominator: i64,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a Fraction {
    fn default() -> &'a Fraction {
        <Fraction as ::protobuf::Message>::default_instance()
    }
}

impl Fraction {
    pub fn new() -> Fraction {
        ::std::default::Default::default()
    }

    // int64 numerator = 1;


    pub fn get_numerator(&self) -> i64 {
        self.numerator
    }
    pub fn clear_numerator(&mut self) {
        self.numerator = 0;
    }

    // Param is passed by value, moved
    pub fn set_numerator(&mut self, v: i64) {
        self.numerator = v;
    }

    // int64 denominator = 2;


    pub fn get_denominator(&self) -> i64 {
        self.denominator
    }
    pub fn clear_denominator(&mut self) {
        self.denominator = 0;
    }

    // Param is passed by value, moved
    pub fn set_denominator(&mut self, v: i64) {
        self.denominator = v;
    }
}

impl ::protobuf::Message for Fraction {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.numerator = tmp;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.denominator = tmp;
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
        if self.numerator != 0 {
            my_size += ::protobuf::rt::value_size(1, self.numerator, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.denominator != 0 {
            my_size += ::protobuf::rt::value_size(2, self.denominator, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if self.numerator != 0 {
            os.write_int64(1, self.numerator)?;
        }
        if self.denominator != 0 {
            os.write_int64(2, self.denominator)?;
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

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> Fraction {
        Fraction::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                "numerator",
                |m: &Fraction| { &m.numerator },
                |m: &mut Fraction| { &mut m.numerator },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                "denominator",
                |m: &Fraction| { &m.denominator },
                |m: &mut Fraction| { &mut m.denominator },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<Fraction>(
                "Fraction",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static Fraction {
        static instance: ::protobuf::rt::LazyV2<Fraction> = ::protobuf::rt::LazyV2::INIT;
        instance.get(Fraction::new)
    }
}

impl ::protobuf::Clear for Fraction {
    fn clear(&mut self) {
        self.numerator = 0;
        self.denominator = 0;
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Fraction {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Fraction {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1agoogle/type/fraction.proto\x12\x0bgoogle.type\"J\n\x08Fraction\x12\
    \x1c\n\tnumerator\x18\x01\x20\x01(\x03R\tnumerator\x12\x20\n\x0bdenomina\
    tor\x18\x02\x20\x01(\x03R\x0bdenominatorBf\n\x0fcom.google.typeB\rFracti\
    onProtoP\x01Z<google.golang.org/genproto/googleapis/type/fraction;fracti\
    on\xa2\x02\x03GTPJ\xbe\x08\n\x06\x12\x04\x0f\0!\x01\n\xbe\x04\n\x01\x0c\
    \x12\x03\x0f\0\x122\xb3\x04\x20Copyright\x202019\x20Google\x20LLC.\n\n\
    \x20Licensed\x20under\x20the\x20Apache\x20License,\x20Version\x202.0\x20\
    (the\x20\"License\");\n\x20you\x20may\x20not\x20use\x20this\x20file\x20e\
    xcept\x20in\x20compliance\x20with\x20the\x20License.\n\x20You\x20may\x20\
    obtain\x20a\x20copy\x20of\x20the\x20License\x20at\n\n\x20\x20\x20\x20\
    \x20http://www.apache.org/licenses/LICENSE-2.0\n\n\x20Unless\x20required\
    \x20by\x20applicable\x20law\x20or\x20agreed\x20to\x20in\x20writing,\x20s\
    oftware\n\x20distributed\x20under\x20the\x20License\x20is\x20distributed\
    \x20on\x20an\x20\"AS\x20IS\"\x20BASIS,\n\x20WITHOUT\x20WARRANTIES\x20OR\
    \x20CONDITIONS\x20OF\x20ANY\x20KIND,\x20either\x20express\x20or\x20impli\
    ed.\n\x20See\x20the\x20License\x20for\x20the\x20specific\x20language\x20\
    governing\x20permissions\x20and\n\x20limitations\x20under\x20the\x20Lice\
    nse.\n\n\n\x08\n\x01\x02\x12\x03\x11\0\x14\n\x08\n\x01\x08\x12\x03\x13\0\
    S\n\t\n\x02\x08\x0b\x12\x03\x13\0S\n\x08\n\x01\x08\x12\x03\x14\0\"\n\t\n\
    \x02\x08\n\x12\x03\x14\0\"\n\x08\n\x01\x08\x12\x03\x15\0.\n\t\n\x02\x08\
    \x08\x12\x03\x15\0.\n\x08\n\x01\x08\x12\x03\x16\0(\n\t\n\x02\x08\x01\x12\
    \x03\x16\0(\n\x08\n\x01\x08\x12\x03\x17\0!\n\t\n\x02\x08$\x12\x03\x17\0!\
    \nU\n\x02\x04\0\x12\x04\x1a\0!\x01\x1aI\x20Represents\x20a\x20fraction\
    \x20in\x20terms\x20of\x20a\x20numerator\x20divided\x20by\x20a\x20denomin\
    ator.\n\n\n\n\x03\x04\0\x01\x12\x03\x1a\x08\x10\nL\n\x04\x04\0\x02\0\x12\
    \x03\x1c\x02\x16\x1a?\x20The\x20portion\x20of\x20the\x20denominator\x20i\
    n\x20the\x20faction,\x20e.g.\x202\x20in\x202/3.\n\n\r\n\x05\x04\0\x02\0\
    \x04\x12\x04\x1c\x02\x1a\x12\n\x0c\n\x05\x04\0\x02\0\x05\x12\x03\x1c\x02\
    \x07\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03\x1c\x08\x11\n\x0c\n\x05\x04\0\
    \x02\0\x03\x12\x03\x1c\x14\x15\n]\n\x04\x04\0\x02\x01\x12\x03\x20\x02\
    \x18\x1aP\x20The\x20value\x20by\x20which\x20the\x20numerator\x20is\x20di\
    vided,\x20e.g.\x203\x20in\x202/3.\x20Must\x20be\n\x20positive.\n\n\r\n\
    \x05\x04\0\x02\x01\x04\x12\x04\x20\x02\x1c\x16\n\x0c\n\x05\x04\0\x02\x01\
    \x05\x12\x03\x20\x02\x07\n\x0c\n\x05\x04\0\x02\x01\x01\x12\x03\x20\x08\
    \x13\n\x0c\n\x05\x04\0\x02\x01\x03\x12\x03\x20\x16\x17b\x06proto3\
";

static file_descriptor_proto_lazy: ::protobuf::rt::LazyV2<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::LazyV2::INIT;

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    file_descriptor_proto_lazy.get(|| {
        parse_descriptor_proto()
    })
}
