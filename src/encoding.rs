use crate::AvmError;
use std::cmp::min;

#[derive(Debug, PartialEq)]
pub struct VarUint64 {
    pub value: u64,
    pub nbytes: usize,
}

impl TryFrom<&[u8]> for VarUint64 {
    type Error = AvmError;

    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        let mut result = 0;
        let mut consumed_bytes = 0;
        // a varint-encoded u64 is at most 10 bytes long
        for i in 0..min(data.len(), 10) {
            consumed_bytes += 1;
            // the most-significant bit (MSB) indicates if we reached
            // the end of the varint (bit 0) or not (bit 1)
            let bits = (data[i] & 0x7F) as u64;
            // in the 10th and last byte the largest possible value
            // is 0x01. any larger value means there's an overflow
            if i == 9 && bits > 0x01 {
                return Err(AvmError::InvalidVarUint64);
            }
            // varints are encoded in little-endian order
            result |= bits << (7 * i);
            // if MSB is 0 we reached the end of the varint
            if data[i] & 0x80 == 0 {
                return Ok(VarUint64 {
                    value: result,
                    nbytes: consumed_bytes,
                });
            }
        }
        Err(AvmError::InvalidVarUint64)
    }
}

#[derive(Debug, PartialEq)]
pub struct VarBytes<'a> {
    pub value: &'a [u8],
    pub nbytes: usize,
}

impl<'a> TryFrom<&'a [u8]> for VarBytes<'a> {
    type Error = AvmError;

    // a varbyte is prefixed by a varint that denotes the length
    // of the payload, followed by the payload
    fn try_from(data: &'a [u8]) -> Result<Self, Self::Error> {
        let len: VarUint64 = data.try_into()?;
        let varbytes_len = len.nbytes + (len.value as usize);
        if data.len() < varbytes_len {
            Err(AvmError::InvalidVarBytes)
        } else {
            Ok(VarBytes {
                value: &data[len.nbytes..varbytes_len],
                nbytes: varbytes_len,
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_varint_from_bytes() {
        let varint_0 = vec![0];
        assert_eq!(
            VarUint64 {
                value: 0,
                nbytes: 1
            },
            (&varint_0[..]).try_into().unwrap()
        );

        let varint_1 = vec![1];
        assert_eq!(
            VarUint64 {
                value: 1,
                nbytes: 1
            },
            (&varint_1[..]).try_into().unwrap()
        );

        let varint_128 = vec![0x80, 0x01];
        assert_eq!(
            VarUint64 {
                value: 128,
                nbytes: 2
            },
            (&varint_128[..]).try_into().unwrap()
        );

        let varint_150 = vec![0x96, 0x01];
        assert_eq!(
            VarUint64 {
                value: 150,
                nbytes: 2
            },
            (&varint_150[..]).try_into().unwrap()
        );

        let varint_156903062 = vec![0x96, 0xCD, 0xE8, 0x4A];
        assert_eq!(
            VarUint64 {
                value: 156903062,
                nbytes: 4
            },
            (&varint_156903062[..]).try_into().unwrap()
        );

        let varint_10458368080512427670 =
            vec![0x96, 0xCD, 0xE8, 0x89, 0xBC, 0xCB, 0xE5, 0x91, 0x91, 0x01];
        assert_eq!(
            VarUint64 {
                value: 10458368080512427670,
                nbytes: 10
            },
            (&varint_10458368080512427670[..]).try_into().unwrap()
        );

        let varint_max = vec![0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x01];
        assert_eq!(
            VarUint64 {
                value: u64::MAX,
                nbytes: 10
            },
            (&varint_max[..]).try_into().unwrap()
        );

        let varint_overflow = vec![0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x02];
        assert_eq!(
            AvmError::InvalidVarUint64,
            VarUint64::try_from(&varint_overflow[..]).unwrap_err()
        );
    }

    #[test]
    fn test_varbytes_from_bytes() {
        let varbytes0 = vec![0x00, 0xDE, 0xAD, 0xBE, 0xEF, 0x00, 0x00];
        assert_eq!(
            VarBytes {
                value: &varbytes0[1..1],
                nbytes: 1
            },
            (&varbytes0[..]).try_into().unwrap()
        );

        let varbytes1 = vec![0x04, 0xDE, 0xAD, 0xBE, 0xEF, 0x00, 0x00];
        assert_eq!(
            VarBytes {
                value: &varbytes1[1..5],
                nbytes: 5
            },
            (&varbytes1[..]).try_into().unwrap()
        );

        // the payload should have 4 bytes, but has actually only 2
        let varbytes_invalid_length = vec![0x04, 0xDE, 0xAD];
        assert_eq!(
            AvmError::InvalidVarBytes,
            VarBytes::try_from(&varbytes_invalid_length[..]).unwrap_err()
        );
    }
}
