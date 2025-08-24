use crate::{
    cmd::{extract_args, validate_command, CommandError, Get, Set},
    RespArray, RespFrame,
};

impl TryFrom<RespArray> for Get {
    type Error = CommandError;

    fn try_from(value: RespArray) -> Result<Self, Self::Error> {
        validate_command(&value, &["get"], 1)?;

        let mut args = extract_args(value, 1)?.into_iter();
        match args.next() {
            Some(RespFrame::BulkString(key)) => Ok(Get {
                key: String::from_utf8(key.0)?,
            }),
            _ => Err(CommandError::InvalidArgument("Invalid key".to_string())),
        }
    }
}

impl TryFrom<RespArray> for Set {
    type Error = CommandError;
    fn try_from(value: RespArray) -> Result<Self, Self::Error> {
        validate_command(&value, &["set"], 2)?;

        let mut args = extract_args(value, 1)?.into_iter();
        match (args.next(), args.next()) {
            (Some(RespFrame::BulkString(key)), Some(value)) => Ok(Set {
                key: String::from_utf8(key.0)?,
                value,
            }),
            _ => Err(CommandError::InvalidArgument(
                "Invalid key or value".to_string(),
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use bytes::BytesMut;

    use crate::{cmd::Get, BulkString, RespDecode};

    use super::*;

    #[test]
    fn test_get() -> Result<()> {
        let mut buf = BytesMut::new();
        buf.extend_from_slice(b"*2\r\n$3\r\nget\r\n$3\r\nkey\r\n");
        let cmd = RespArray::decode(&mut buf)?;
        let get = Get::try_from(cmd)?;
        assert_eq!(get.key, "key");
        Ok(())
    }

    #[test]
    fn test_set() -> Result<()> {
        let mut buf = BytesMut::new();
        buf.extend_from_slice(b"*3\r\n$3\r\nset\r\n$3\r\nkey\r\n$5\r\nvalue\r\n");
        let cmd = RespArray::decode(&mut buf)?;
        let set = Set::try_from(cmd)?;
        assert_eq!(set.key, "key");
        assert_eq!(set.value, RespFrame::BulkString(BulkString::from("value")));
        Ok(())
    }
}
