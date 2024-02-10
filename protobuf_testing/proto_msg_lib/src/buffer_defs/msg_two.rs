// Automatically generated rust module for 'msg_two.proto' file

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
pub struct SubMsg {
    pub value: Option<i32>,
}

impl<'a> MessageRead<'a> for SubMsg {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.value = Some(r.read_int32(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for SubMsg {
    fn get_size(&self) -> usize {
        0
        + self.value.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.value { w.write_with_tag(8, |w| w.write_int32(*s))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Two {
    pub two_int32: Option<i32>,
    pub two_int64: Option<i64>,
    pub two_uint32: Option<u32>,
    pub two_uint64: Option<u64>,
    pub two_sub: Option<SubMsg>,
}

impl<'a> MessageRead<'a> for Two {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.two_int32 = Some(r.read_int32(bytes)?),
                Ok(16) => msg.two_int64 = Some(r.read_int64(bytes)?),
                Ok(24) => msg.two_uint32 = Some(r.read_uint32(bytes)?),
                Ok(32) => msg.two_uint64 = Some(r.read_uint64(bytes)?),
                Ok(42) => msg.two_sub = Some(r.read_message::<SubMsg>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Two {
    fn get_size(&self) -> usize {
        0
        + self.two_int32.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.two_int64.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.two_uint32.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.two_uint64.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.two_sub.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.two_int32 { w.write_with_tag(8, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.two_int64 { w.write_with_tag(16, |w| w.write_int64(*s))?; }
        if let Some(ref s) = self.two_uint32 { w.write_with_tag(24, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.two_uint64 { w.write_with_tag(32, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.two_sub { w.write_with_tag(42, |w| w.write_message(s))?; }
        Ok(())
    }
}

