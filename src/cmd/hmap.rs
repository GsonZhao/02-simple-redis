use crate::{
    cmd::{extract_args, validate_command, CommandError, HGet, HGetAll, HSet},
    RespArray, RespFrame,
};

impl TryFrom<RespArray> for HGet {
    type Error = CommandError;

    fn try_from(value: RespArray) -> Result<Self, Self::Error> {
        validate_command(&value, &["hget"], 2)?;

        let mut args = extract_args(value, 1)?.into_iter();
        match (args.next(), args.next()) {
            (Some(RespFrame::BulkString(key)), Some(RespFrame::BulkString(field))) => Ok(HGet {
                key: String::from_utf8(key.0)?,
                field: String::from_utf8(field.0)?,
            }),
            _ => Err(CommandError::InvalidArgument(
                "Invalid key or field".to_string(),
            )),
        }
    }
}

impl TryFrom<RespArray> for HSet {
    type Error = CommandError;

    fn try_from(value: RespArray) -> Result<Self, Self::Error> {
        validate_command(&value, &["hset"], 3)?;

        let mut args = extract_args(value, 1)?.into_iter();
        match (args.next(), args.next(), args.next()) {
            (Some(RespFrame::BulkString(key)), Some(RespFrame::BulkString(field)), Some(value)) => {
                Ok(HSet {
                    key: String::from_utf8(key.0)?,
                    field: String::from_utf8(field.0)?,
                    value,
                })
            }
            _ => Err(CommandError::InvalidArgument(
                "Invalid key or field".to_string(),
            )),
        }
    }
}

impl TryFrom<RespArray> for HGetAll {
    type Error = CommandError;

    fn try_from(value: RespArray) -> Result<Self, Self::Error> {
        validate_command(&value, &["hgetall"], 1)?;

        let mut args = extract_args(value, 1)?.into_iter();
        match args.next() {
            Some(RespFrame::BulkString(key)) => Ok(HGetAll {
                key: String::from_utf8(key.0)?,
            }),
            _ => Err(CommandError::InvalidArgument("Invalid key".to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use bytes::BytesMut;

    use crate::{cmd::HGet, BulkString, RespDecode};

    use super::*;

    #[test]
    fn test_hget() -> Result<()> {
        let mut buf = BytesMut::new();
        buf.extend_from_slice(b"*3\r\n$4\r\nhget\r\n$3\r\nkey\r\n$5\r\nfield\r\n");
        let cmd = RespArray::decode(&mut buf)?;
        let hget = HGet::try_from(cmd)?;
        assert_eq!(hget.key, "key");
        assert_eq!(hget.field, "field");
        Ok(())
    }

    #[test]
    fn test_hset() -> Result<()> {
        let mut buf = BytesMut::new();
        buf.extend_from_slice(b"*4\r\n$4\r\nhset\r\n$3\r\nkey\r\n$5\r\nfield\r\n$5\r\nvalue\r\n");
        let cmd = RespArray::decode(&mut buf)?;
        let hset = HSet::try_from(cmd)?;
        assert_eq!(hset.key, "key");
        assert_eq!(hset.field, "field");
        assert_eq!(hset.value, RespFrame::BulkString(BulkString::from("value")));
        Ok(())
    }

    #[test]
    fn test_hgetall() -> Result<()> {
        let mut buf = BytesMut::new();
        buf.extend_from_slice(b"*2\r\n$7\r\nhgetall\r\n$3\r\nkey\r\n");
        let cmd = RespArray::decode(&mut buf)?;
        let hgetall = HGetAll::try_from(cmd)?;
        assert_eq!(hgetall.key, "key");
        Ok(())
    }
}
