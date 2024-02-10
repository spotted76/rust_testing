// Automatically generated rust module for 'msg_one.proto' file

#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
#![allow(unknown_lints)]
#![allow(clippy::all)]
#![cfg_attr(rustfmt, rustfmt_skip)]


use quick_protobuf::{MessageInfo, MessageRead, MessageWrite, BytesReader, Writer, WriterBackend, Result};
use quick_protobuf::sizeofs::*;
use super::*;

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct MessageOne {
    pub f_int32: Option<i32>,
    pub f_int64: Option<i64>,
    pub f_uint32: Option<u32>,
    pub f_uint64: Option<u64>,
}

impl<'a> MessageRead<'a> for MessageOne {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.f_int32 = Some(r.read_int32(bytes)?),
                Ok(16) => msg.f_int64 = Some(r.read_int64(bytes)?),
                Ok(24) => msg.f_uint32 = Some(r.read_uint32(bytes)?),
                Ok(32) => msg.f_uint64 = Some(r.read_uint64(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for MessageOne {
    fn get_size(&self) -> usize {
        0
        + self.f_int32.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.f_int64.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.f_uint32.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.f_uint64.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.f_int32 { w.write_with_tag(8, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.f_int64 { w.write_with_tag(16, |w| w.write_int64(*s))?; }
        if let Some(ref s) = self.f_uint32 { w.write_with_tag(24, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.f_uint64 { w.write_with_tag(32, |w| w.write_uint64(*s))?; }
        Ok(())
    }
}

