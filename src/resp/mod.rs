/*
Redis Serialization Protocol (RESP) 协议规范

RESP数据类型：

1. Simple String (简单字符串)
   - 格式: "+<string>\r\n"
   - 示例: "+OK\r\n"
   - 说明: 不能包含\r或\n字符，用于传输简单状态信息

2. Simple Error (简单错误)
   - 格式: "-<error message>\r\n"
   - 示例: "-Error message\r\n"
   - 说明: 与Simple String类似，但表示错误状态

3. Integer (整数)
   - 格式: ":[<+|->]<value>\r\n"
   - 示例: ":1000\r\n"
   - 说明: 64位有符号整数

4. Bulk String (批量字符串)
   - 格式: "$<length>\r\n<string>\r\n"
   - 示例: "$6\r\nfoobar\r\n"
   - 说明: 可以包含任何二进制数据，包括\r\n字符
   - Null值: "$-1\r\n"

5. Array (数组)
   - 格式: "*<number-of-elements>\r\n<element-1>...<element-n>"
   - 示例: "*2\r\n$3\r\nfoo\r\n$3\r\nbar\r\n"
   - 说明: 包含多个RESP类型的数组
   - Null数组: "*-1\r\n"

6. Null (空值)
   - 格式: "$-1\r\n"
   - 说明: 表示不存在的值

RESP3新增数据类型：

7. Boolean (布尔值)
   - 格式: "#<t|f>\r\n"
   - 示例: "#t\r\n" (true), "#f\r\n" (false)

8. Double (双精度浮点数)
   - 格式: ",<floating-point-number>\r\n"
   - 示例: ",1.23\r\n"

9. Big Number (大整数)
   - 格式: "(<big number>\r\n"
   - 示例: "(3492890328409238509324850943850943825024385\r\n"

10. Bulk Error (批量错误)
    - 格式: "!<length>\r\n<error>\r\n"
    - 示例: "!21\r\nSYNTAX invalid syntax\r\n"

11. Verbatim String (逐字字符串)
    - 格式: "=<length>\r\n<encoding>:<string>\r\n"
    - 示例: "=15\r\ntxt:Some string\r\n"

12. Map (映射)
    - 格式: "%<number-of-key-value-pairs>\r\n<key-1><value-1>...<key-n><value-n>"
    - 示例: "%2\r\n+first\r\n:1\r\n+second\r\n:2\r\n"

13. Set (集合)
    - 格式: "~<number-of-elements>\r\n<element-1>...<element-n>"
    - 示例: "~3\r\n+orange\r\n+apple\r\n+one\r\n"

14. Push (推送)
    - 格式: "><number-of-elements>\r\n<element-1>...<element-n>"
    - 说明: 用于服务器向客户端推送数据
*/
mod decode;
mod encode;

use std::{
    collections::BTreeMap,
    ops::{Deref, DerefMut},
};

use bytes::BytesMut;
use enum_dispatch::enum_dispatch;
use thiserror::Error;

#[enum_dispatch]
pub trait RespEncode {
    fn encode(self) -> Vec<u8>;
}

pub trait RespDecode: Sized {
    const PREFIX: &'static str;
    fn decode(buf: &mut BytesMut) -> Result<Self, RespError>;
    fn expect_length(buf: &[u8]) -> Result<usize, RespError>;
}

#[derive(Error, Debug, PartialEq, Eq)]
pub enum RespError {
    #[error("Invalid frame: {0}")]
    InvalidFrame(String),
    #[error("Invalid frame type: {0}")]
    InvalidFrameType(String),
    #[error("Invalid frame length： {0}")]
    InvalidFrameLength(isize),
    #[error("Frame is not complete")]
    NotComplete,

    #[error("Parse error: {0}")]
    ParseIntError(#[from] std::num::ParseIntError),
    #[error("Utf8 error: {0}")]
    Utf8Error(#[from] std::str::Utf8Error),
    #[error("Parse float error: {0}")]
    ParseFloatError(#[from] std::num::ParseFloatError),
}

#[enum_dispatch(RespEncode)]
#[derive(Debug, PartialEq, Clone)]
pub enum RespFrame {
    SimpleString(SimpleString),
    Error(SimpleError),
    Integer(i64),
    BulkString(BulkString),
    NullBulkString(RespNullBulkString),
    Array(RespArray),
    NullArray(RespNullArray),
    Null(RespNull),

    Boolean(bool),
    Double(f64),
    Map(RespMap),
    Set(RespSet),
}

#[derive(Debug, PartialEq, Clone)]
pub struct SimpleString(pub(crate) String);

#[derive(Debug, PartialEq, Clone)]
pub struct SimpleError(pub(crate) String);

#[derive(Debug, PartialEq, Clone)]
pub struct RespNull;

#[derive(Debug, PartialEq, Clone)]
pub struct RespNullArray;

#[derive(Debug, PartialEq, Clone)]
pub struct RespNullBulkString;

#[derive(Debug, PartialEq, Clone)]
pub struct BulkString(pub(crate) Vec<u8>);

#[derive(Debug, PartialEq, Clone)]
pub struct RespArray(pub(crate) Vec<RespFrame>);

#[derive(Debug, PartialEq, Clone)]
pub struct RespMap(pub(crate) BTreeMap<String, RespFrame>);

#[derive(Debug, PartialEq, Clone)]
pub struct RespSet(pub(crate) Vec<RespFrame>);

impl Deref for SimpleString {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Deref for SimpleError {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Deref for BulkString {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Deref for RespArray {
    type Target = Vec<RespFrame>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Deref for RespMap {
    type Target = BTreeMap<String, RespFrame>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Deref for RespSet {
    type Target = Vec<RespFrame>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for RespMap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl SimpleString {
    pub fn new(s: impl Into<String>) -> Self {
        Self(s.into())
    }
}

impl SimpleError {
    pub fn new(s: impl Into<String>) -> Self {
        SimpleError(s.into())
    }
}

impl BulkString {
    pub fn new(s: impl Into<Vec<u8>>) -> Self {
        BulkString(s.into())
    }
}

impl RespArray {
    pub fn new(s: impl Into<Vec<RespFrame>>) -> Self {
        RespArray(s.into())
    }
}

impl RespMap {
    pub fn new() -> Self {
        RespMap(BTreeMap::new())
    }
}

impl Default for RespMap {
    fn default() -> Self {
        RespMap::new()
    }
}

impl RespSet {
    pub fn new(s: impl Into<Vec<RespFrame>>) -> Self {
        RespSet(s.into())
    }
}

impl From<&str> for SimpleString {
    fn from(s: &str) -> Self {
        SimpleString::new(s)
    }
}

impl From<&str> for RespFrame {
    fn from(s: &str) -> Self {
        RespFrame::SimpleString(SimpleString::from(s))
    }
}

impl From<&str> for SimpleError {
    fn from(s: &str) -> Self {
        SimpleError::new(s)
    }
}

impl From<&str> for BulkString {
    fn from(s: &str) -> Self {
        BulkString::new(s)
    }
}

impl From<&[u8]> for BulkString {
    fn from(s: &[u8]) -> Self {
        BulkString::new(s)
    }
}

impl From<&[u8]> for RespFrame {
    fn from(s: &[u8]) -> Self {
        RespFrame::BulkString(BulkString::from(s))
    }
}

impl<const N: usize> From<&[u8; N]> for BulkString {
    fn from(s: &[u8; N]) -> Self {
        BulkString::new(s)
    }
}

impl<const N: usize> From<&[u8; N]> for RespFrame {
    fn from(s: &[u8; N]) -> Self {
        Into::<BulkString>::into(s).into()
    }
}

impl AsRef<[u8]> for BulkString {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}
