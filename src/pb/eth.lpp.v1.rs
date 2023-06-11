// @generated
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Transfers {
    #[prost(message, repeated, tag="1")]
    pub transfers: ::prost::alloc::vec::Vec<Transfer>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Transfer {
    #[prost(bytes="vec", tag="1")]
    pub from: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="2")]
    pub to: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint64, tag="3")]
    pub token_id: u64,
    #[prost(bytes="vec", tag="4")]
    pub trx_hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint64, tag="5")]
    pub ordinal: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Collects {
    #[prost(message, repeated, tag="1")]
    pub collects: ::prost::alloc::vec::Vec<Collect>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Collect {
    #[prost(uint64, tag="1")]
    pub profile_id: u64,
    #[prost(uint64, tag="2")]
    pub pub_id: u64,
    #[prost(bytes="vec", tag="3")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Follows {
    #[prost(message, repeated, tag="1")]
    pub follows: ::prost::alloc::vec::Vec<Follow>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Follow {
    #[prost(message, repeated, tag="1")]
    pub profile_ids: ::prost::alloc::vec::Vec<ProfileId>,
    #[prost(message, repeated, tag="2")]
    pub datas: ::prost::alloc::vec::Vec<Data>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProfileId {
    #[prost(uint64, tag="1")]
    pub profile_id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Data {
    #[prost(bytes="vec", tag="1")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
/// Encoded file descriptor set for the `eth.lpp.v1` package
pub const FILE_DESCRIPTOR_SET: &[u8] = &[
    0x0a, 0xb7, 0x0d, 0x0a, 0x09, 0x6c, 0x70, 0x70, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x0a,
    0x65, 0x74, 0x68, 0x2e, 0x6c, 0x70, 0x70, 0x2e, 0x76, 0x31, 0x22, 0x3f, 0x0a, 0x09, 0x54, 0x72,
    0x61, 0x6e, 0x73, 0x66, 0x65, 0x72, 0x73, 0x12, 0x32, 0x0a, 0x09, 0x74, 0x72, 0x61, 0x6e, 0x73,
    0x66, 0x65, 0x72, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x14, 0x2e, 0x65, 0x74, 0x68,
    0x2e, 0x6c, 0x70, 0x70, 0x2e, 0x76, 0x31, 0x2e, 0x54, 0x72, 0x61, 0x6e, 0x73, 0x66, 0x65, 0x72,
    0x52, 0x09, 0x74, 0x72, 0x61, 0x6e, 0x73, 0x66, 0x65, 0x72, 0x73, 0x22, 0x7e, 0x0a, 0x08, 0x54,
    0x72, 0x61, 0x6e, 0x73, 0x66, 0x65, 0x72, 0x12, 0x12, 0x0a, 0x04, 0x66, 0x72, 0x6f, 0x6d, 0x18,
    0x01, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x04, 0x66, 0x72, 0x6f, 0x6d, 0x12, 0x0e, 0x0a, 0x02, 0x74,
    0x6f, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x02, 0x74, 0x6f, 0x12, 0x19, 0x0a, 0x08, 0x74,
    0x6f, 0x6b, 0x65, 0x6e, 0x5f, 0x69, 0x64, 0x18, 0x03, 0x20, 0x01, 0x28, 0x04, 0x52, 0x07, 0x74,
    0x6f, 0x6b, 0x65, 0x6e, 0x49, 0x64, 0x12, 0x19, 0x0a, 0x08, 0x74, 0x72, 0x78, 0x5f, 0x68, 0x61,
    0x73, 0x68, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x07, 0x74, 0x72, 0x78, 0x48, 0x61, 0x73,
    0x68, 0x12, 0x18, 0x0a, 0x07, 0x6f, 0x72, 0x64, 0x69, 0x6e, 0x61, 0x6c, 0x18, 0x05, 0x20, 0x01,
    0x28, 0x04, 0x52, 0x07, 0x6f, 0x72, 0x64, 0x69, 0x6e, 0x61, 0x6c, 0x22, 0x3b, 0x0a, 0x08, 0x43,
    0x6f, 0x6c, 0x6c, 0x65, 0x63, 0x74, 0x73, 0x12, 0x2f, 0x0a, 0x08, 0x63, 0x6f, 0x6c, 0x6c, 0x65,
    0x63, 0x74, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x13, 0x2e, 0x65, 0x74, 0x68, 0x2e,
    0x6c, 0x70, 0x70, 0x2e, 0x76, 0x31, 0x2e, 0x43, 0x6f, 0x6c, 0x6c, 0x65, 0x63, 0x74, 0x52, 0x08,
    0x63, 0x6f, 0x6c, 0x6c, 0x65, 0x63, 0x74, 0x73, 0x22, 0x51, 0x0a, 0x07, 0x43, 0x6f, 0x6c, 0x6c,
    0x65, 0x63, 0x74, 0x12, 0x1c, 0x0a, 0x09, 0x70, 0x72, 0x6f, 0x66, 0x69, 0x6c, 0x65, 0x49, 0x64,
    0x18, 0x01, 0x20, 0x01, 0x28, 0x04, 0x52, 0x09, 0x70, 0x72, 0x6f, 0x66, 0x69, 0x6c, 0x65, 0x49,
    0x64, 0x12, 0x14, 0x0a, 0x05, 0x70, 0x75, 0x62, 0x49, 0x64, 0x18, 0x02, 0x20, 0x01, 0x28, 0x04,
    0x52, 0x05, 0x70, 0x75, 0x62, 0x49, 0x64, 0x12, 0x12, 0x0a, 0x04, 0x64, 0x61, 0x74, 0x61, 0x18,
    0x03, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x04, 0x64, 0x61, 0x74, 0x61, 0x22, 0x37, 0x0a, 0x07, 0x46,
    0x6f, 0x6c, 0x6c, 0x6f, 0x77, 0x73, 0x12, 0x2c, 0x0a, 0x07, 0x66, 0x6f, 0x6c, 0x6c, 0x6f, 0x77,
    0x73, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x12, 0x2e, 0x65, 0x74, 0x68, 0x2e, 0x6c, 0x70,
    0x70, 0x2e, 0x76, 0x31, 0x2e, 0x46, 0x6f, 0x6c, 0x6c, 0x6f, 0x77, 0x52, 0x07, 0x66, 0x6f, 0x6c,
    0x6c, 0x6f, 0x77, 0x73, 0x22, 0x67, 0x0a, 0x06, 0x46, 0x6f, 0x6c, 0x6c, 0x6f, 0x77, 0x12, 0x35,
    0x0a, 0x0a, 0x70, 0x72, 0x6f, 0x66, 0x69, 0x6c, 0x65, 0x49, 0x64, 0x73, 0x18, 0x01, 0x20, 0x03,
    0x28, 0x0b, 0x32, 0x15, 0x2e, 0x65, 0x74, 0x68, 0x2e, 0x6c, 0x70, 0x70, 0x2e, 0x76, 0x31, 0x2e,
    0x50, 0x72, 0x6f, 0x66, 0x69, 0x6c, 0x65, 0x49, 0x64, 0x52, 0x0a, 0x70, 0x72, 0x6f, 0x66, 0x69,
    0x6c, 0x65, 0x49, 0x64, 0x73, 0x12, 0x26, 0x0a, 0x05, 0x64, 0x61, 0x74, 0x61, 0x73, 0x18, 0x02,
    0x20, 0x03, 0x28, 0x0b, 0x32, 0x10, 0x2e, 0x65, 0x74, 0x68, 0x2e, 0x6c, 0x70, 0x70, 0x2e, 0x76,
    0x31, 0x2e, 0x44, 0x61, 0x74, 0x61, 0x52, 0x05, 0x64, 0x61, 0x74, 0x61, 0x73, 0x22, 0x29, 0x0a,
    0x09, 0x50, 0x72, 0x6f, 0x66, 0x69, 0x6c, 0x65, 0x49, 0x64, 0x12, 0x1c, 0x0a, 0x09, 0x70, 0x72,
    0x6f, 0x66, 0x69, 0x6c, 0x65, 0x49, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x04, 0x52, 0x09, 0x70,
    0x72, 0x6f, 0x66, 0x69, 0x6c, 0x65, 0x49, 0x64, 0x22, 0x1a, 0x0a, 0x04, 0x44, 0x61, 0x74, 0x61,
    0x12, 0x12, 0x0a, 0x04, 0x64, 0x61, 0x74, 0x61, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x04,
    0x64, 0x61, 0x74, 0x61, 0x4a, 0xdb, 0x08, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x29, 0x01, 0x0a,
    0x08, 0x0a, 0x01, 0x0c, 0x12, 0x03, 0x00, 0x00, 0x12, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03,
    0x02, 0x00, 0x13, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x04, 0x00, 0x06, 0x01, 0x0a,
    0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x04, 0x08, 0x11, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x00, 0x02, 0x00, 0x12, 0x03, 0x05, 0x02, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00,
    0x04, 0x12, 0x03, 0x05, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x06, 0x12,
    0x03, 0x05, 0x0b, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x05,
    0x14, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x05, 0x20, 0x21,
    0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x01, 0x12, 0x04, 0x08, 0x00, 0x0e, 0x01, 0x0a, 0x0a, 0x0a, 0x03,
    0x04, 0x01, 0x01, 0x12, 0x03, 0x08, 0x08, 0x10, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x00,
    0x12, 0x03, 0x09, 0x02, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x05, 0x12, 0x03,
    0x09, 0x02, 0x07, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x09, 0x08,
    0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x03, 0x12, 0x03, 0x09, 0x0f, 0x10, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x01, 0x12, 0x03, 0x0a, 0x02, 0x0f, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x01, 0x05, 0x12, 0x03, 0x0a, 0x02, 0x07, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x01, 0x01, 0x12, 0x03, 0x0a, 0x08, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01,
    0x03, 0x12, 0x03, 0x0a, 0x0d, 0x0e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x02, 0x12, 0x03,
    0x0b, 0x02, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x05, 0x12, 0x03, 0x0b, 0x02,
    0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x01, 0x12, 0x03, 0x0b, 0x09, 0x11, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x03, 0x12, 0x03, 0x0b, 0x14, 0x15, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x01, 0x02, 0x03, 0x12, 0x03, 0x0c, 0x02, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x03, 0x05, 0x12, 0x03, 0x0c, 0x02, 0x07, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x03,
    0x01, 0x12, 0x03, 0x0c, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x03, 0x03, 0x12,
    0x03, 0x0c, 0x13, 0x14, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x04, 0x12, 0x03, 0x0d, 0x02,
    0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x04, 0x05, 0x12, 0x03, 0x0d, 0x02, 0x08, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x04, 0x01, 0x12, 0x03, 0x0d, 0x09, 0x10, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x04, 0x03, 0x12, 0x03, 0x0d, 0x13, 0x14, 0x0a, 0x0a, 0x0a, 0x02, 0x04,
    0x02, 0x12, 0x04, 0x10, 0x00, 0x12, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x02, 0x01, 0x12, 0x03,
    0x10, 0x08, 0x10, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x00, 0x12, 0x03, 0x11, 0x02, 0x20,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x04, 0x12, 0x03, 0x11, 0x02, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x06, 0x12, 0x03, 0x11, 0x0b, 0x12, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x02, 0x02, 0x00, 0x01, 0x12, 0x03, 0x11, 0x13, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02,
    0x02, 0x00, 0x03, 0x12, 0x03, 0x11, 0x1e, 0x1f, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x03, 0x12, 0x04,
    0x14, 0x00, 0x18, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x03, 0x01, 0x12, 0x03, 0x14, 0x08, 0x0f,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x00, 0x12, 0x03, 0x15, 0x02, 0x17, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x03, 0x02, 0x00, 0x05, 0x12, 0x03, 0x15, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x03, 0x02, 0x00, 0x01, 0x12, 0x03, 0x15, 0x09, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02,
    0x00, 0x03, 0x12, 0x03, 0x15, 0x15, 0x16, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x01, 0x12,
    0x03, 0x16, 0x02, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x05, 0x12, 0x03, 0x16,
    0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x01, 0x12, 0x03, 0x16, 0x09, 0x0e,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x03, 0x12, 0x03, 0x16, 0x11, 0x12, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x03, 0x02, 0x02, 0x12, 0x03, 0x17, 0x02, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x03, 0x02, 0x02, 0x05, 0x12, 0x03, 0x17, 0x02, 0x07, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02,
    0x02, 0x01, 0x12, 0x03, 0x17, 0x08, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x02, 0x03,
    0x12, 0x03, 0x17, 0x0f, 0x10, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x04, 0x12, 0x04, 0x1a, 0x00, 0x1c,
    0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x04, 0x01, 0x12, 0x03, 0x1a, 0x08, 0x0f, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x04, 0x02, 0x00, 0x12, 0x03, 0x1b, 0x02, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04,
    0x02, 0x00, 0x04, 0x12, 0x03, 0x1b, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00,
    0x06, 0x12, 0x03, 0x1b, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x01, 0x12,
    0x03, 0x1b, 0x12, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x03, 0x12, 0x03, 0x1b,
    0x1c, 0x1d, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x05, 0x12, 0x04, 0x1e, 0x00, 0x21, 0x01, 0x0a, 0x0a,
    0x0a, 0x03, 0x04, 0x05, 0x01, 0x12, 0x03, 0x1e, 0x08, 0x0e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05,
    0x02, 0x00, 0x12, 0x03, 0x1f, 0x02, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x04,
    0x12, 0x03, 0x1f, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x06, 0x12, 0x03,
    0x1f, 0x0b, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x01, 0x12, 0x03, 0x1f, 0x15,
    0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x03, 0x12, 0x03, 0x1f, 0x22, 0x23, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x01, 0x12, 0x03, 0x20, 0x02, 0x1a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x05, 0x02, 0x01, 0x04, 0x12, 0x03, 0x20, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05,
    0x02, 0x01, 0x06, 0x12, 0x03, 0x20, 0x0b, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01,
    0x01, 0x12, 0x03, 0x20, 0x10, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x03, 0x12,
    0x03, 0x20, 0x18, 0x19, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x06, 0x12, 0x04, 0x23, 0x00, 0x25, 0x01,
    0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x06, 0x01, 0x12, 0x03, 0x23, 0x08, 0x11, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x06, 0x02, 0x00, 0x12, 0x03, 0x24, 0x02, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02,
    0x00, 0x05, 0x12, 0x03, 0x24, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x24, 0x09, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x03, 0x12, 0x03,
    0x24, 0x15, 0x16, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x07, 0x12, 0x04, 0x27, 0x00, 0x29, 0x01, 0x0a,
    0x0a, 0x0a, 0x03, 0x04, 0x07, 0x01, 0x12, 0x03, 0x27, 0x08, 0x0c, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x07, 0x02, 0x00, 0x12, 0x03, 0x28, 0x02, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00,
    0x05, 0x12, 0x03, 0x28, 0x02, 0x07, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x01, 0x12,
    0x03, 0x28, 0x08, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x03, 0x12, 0x03, 0x28,
    0x0f, 0x10, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
];
// @@protoc_insertion_point(module)