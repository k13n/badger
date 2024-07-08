use crate::{
    encoding::{VarBytes, VarUint64},
    AvmError,
};

use super::{Avm, AvmData, AvmVersion, FALSE, TRUE};

type OpcodeEvalFunc = fn(&mut Avm) -> Result<(), AvmError>;

pub struct OpSpec {
    pub opcode: u8,
    pub name: &'static str,
    pub cost: u64,
    pub eval: OpcodeEvalFunc,
    pub version: AvmVersion,
}

pub const OP_SPECS: [OpSpec; 33] = [
    OpSpec {
        opcode: 0x00,
        name: "err",
        version: AvmVersion::V1,
        cost: 1,
        eval: op_err,
    },
    OpSpec {
        opcode: 0x08,
        name: "+",
        version: AvmVersion::V1,
        cost: 1,
        eval: op_plus,
    },
    OpSpec {
        opcode: 0x09,
        name: "-",
        version: AvmVersion::V1,
        cost: 1,
        eval: op_minus,
    },
    OpSpec {
        opcode: 0x0a,
        name: "/",
        version: AvmVersion::V1,
        cost: 1,
        eval: op_div,
    },
    OpSpec {
        opcode: 0x0b,
        name: "*",
        version: AvmVersion::V1,
        cost: 1,
        eval: op_mul,
    },
    OpSpec {
        opcode: 0x0c,
        name: "<",
        version: AvmVersion::V1,
        cost: 1,
        eval: op_lt,
    },
    OpSpec {
        opcode: 0x0d,
        name: ">",
        version: AvmVersion::V1,
        cost: 1,
        eval: op_gt,
    },
    OpSpec {
        opcode: 0x0e,
        name: "<=",
        version: AvmVersion::V1,
        cost: 1,
        eval: op_leq,
    },
    OpSpec {
        opcode: 0x0f,
        name: ">=",
        version: AvmVersion::V1,
        cost: 1,
        eval: op_geq,
    },
    OpSpec {
        opcode: 0x10,
        name: "&&",
        version: AvmVersion::V1,
        cost: 1,
        eval: op_and,
    },
    OpSpec {
        opcode: 0x11,
        name: "||",
        version: AvmVersion::V1,
        cost: 1,
        eval: op_or,
    },
    OpSpec {
        opcode: 0x12,
        name: "==",
        version: AvmVersion::V1,
        cost: 1,
        eval: op_eq,
    },
    OpSpec {
        opcode: 0x13,
        name: "!=",
        version: AvmVersion::V1,
        cost: 1,
        eval: op_neq,
    },
    OpSpec {
        opcode: 0x14,
        name: "!",
        version: AvmVersion::V1,
        cost: 1,
        eval: op_neg,
    },
    OpSpec {
        opcode: 0x15,
        name: "len",
        version: AvmVersion::V1,
        cost: 1,
        eval: op_len,
    },
    OpSpec {
        opcode: 0x16,
        name: "itob",
        version: AvmVersion::V1,
        cost: 1,
        eval: op_itob,
    },
    OpSpec {
        opcode: 0x17,
        name: "btoi",
        version: AvmVersion::V1,
        cost: 1,
        eval: op_btoi,
    },
    OpSpec {
        opcode: 0x20,
        name: "intcblock",
        version: AvmVersion::V1,
        cost: 1,
        eval: op_intcblock,
    },
    OpSpec {
        opcode: 0x21,
        name: "intc",
        version: AvmVersion::V1,
        cost: 1,
        eval: op_intc,
    },
    OpSpec {
        opcode: 0x22,
        name: "intc_0",
        version: AvmVersion::V1,
        cost: 1,
        eval: op_intc_0,
    },
    OpSpec {
        opcode: 0x23,
        name: "intc_1",
        version: AvmVersion::V1,
        cost: 1,
        eval: op_intc_1,
    },
    OpSpec {
        opcode: 0x24,
        name: "intc_2",
        version: AvmVersion::V1,
        cost: 1,
        eval: op_intc_2,
    },
    OpSpec {
        opcode: 0x25,
        name: "intc_3",
        version: AvmVersion::V1,
        cost: 1,
        eval: op_intc_3,
    },
    OpSpec {
        opcode: 0x26,
        name: "bytecblock",
        version: AvmVersion::V1,
        cost: 1,
        eval: op_bytecblock,
    },
    OpSpec {
        opcode: 0x27,
        name: "bytec",
        version: AvmVersion::V1,
        cost: 1,
        eval: op_bytec,
    },
    OpSpec {
        opcode: 0x28,
        name: "bytec_0",
        version: AvmVersion::V1,
        cost: 1,
        eval: op_bytec_0,
    },
    OpSpec {
        opcode: 0x29,
        name: "bytec_1",
        version: AvmVersion::V1,
        cost: 1,
        eval: op_bytec_1,
    },
    OpSpec {
        opcode: 0x2a,
        name: "bytec_2",
        version: AvmVersion::V1,
        cost: 1,
        eval: op_bytec_2,
    },
    OpSpec {
        opcode: 0x2b,
        name: "bytec_3",
        version: AvmVersion::V1,
        cost: 1,
        eval: op_bytec_3,
    },
    OpSpec {
        opcode: 0x48,
        name: "pop",
        version: AvmVersion::V1,
        cost: 1,
        eval: op_pop,
    },
    OpSpec {
        opcode: 0x50,
        name: "concat",
        version: AvmVersion::V2,
        cost: 1,
        eval: op_concat,
    },
    OpSpec {
        opcode: 0x80,
        name: "pushbytes",
        version: AvmVersion::V3,
        cost: 1,
        eval: op_pushbytes,
    },
    OpSpec {
        opcode: 0x81,
        name: "pushint",
        version: AvmVersion::V3,
        cost: 1,
        eval: op_pushint,
    },
];

fn op_err(_avm: &mut Avm) -> Result<(), AvmError> {
    Err(AvmError::ErrOpCode)
}

fn op_plus(avm: &mut Avm) -> Result<(), AvmError> {
    let rhs = avm.pop_uint64()?;
    let lhs = avm.pop_uint64()?;
    match lhs.checked_add(rhs) {
        Some(res) => {
            avm.data_stack.push(res.into());
            Ok(())
        }
        None => Err(AvmError::IntegerOverflow),
    }
}

fn op_minus(avm: &mut Avm) -> Result<(), AvmError> {
    let rhs = avm.pop_uint64()?;
    let lhs = avm.pop_uint64()?;
    match lhs.checked_sub(rhs) {
        Some(res) => {
            avm.data_stack.push(res.into());
            Ok(())
        }
        None => Err(AvmError::IntegerUnderflow),
    }
}

fn op_div(avm: &mut Avm) -> Result<(), AvmError> {
    let rhs = avm.pop_uint64()?;
    let lhs = avm.pop_uint64()?;
    match lhs.checked_div(rhs) {
        Some(res) => {
            avm.data_stack.push(res.into());
            Ok(())
        }
        None => Err(AvmError::DivisionByZero),
    }
}

fn op_mul(avm: &mut Avm) -> Result<(), AvmError> {
    let rhs = avm.pop_uint64()?;
    let lhs = avm.pop_uint64()?;
    match lhs.checked_mul(rhs) {
        Some(res) => {
            avm.data_stack.push(res.into());
            Ok(())
        }
        None => Err(AvmError::IntegerOverflow),
    }
}

fn op_lt(avm: &mut Avm) -> Result<(), AvmError> {
    let rhs = avm.pop_uint64()?;
    let lhs = avm.pop_uint64()?;
    avm.data_stack.push((lhs < rhs).into());
    Ok(())
}

fn op_gt(avm: &mut Avm) -> Result<(), AvmError> {
    let rhs = avm.pop_uint64()?;
    let lhs = avm.pop_uint64()?;
    avm.data_stack.push((lhs > rhs).into());
    Ok(())
}

fn op_leq(avm: &mut Avm) -> Result<(), AvmError> {
    let rhs = avm.pop_uint64()?;
    let lhs = avm.pop_uint64()?;
    avm.data_stack.push((lhs <= rhs).into());
    Ok(())
}

fn op_geq(avm: &mut Avm) -> Result<(), AvmError> {
    let rhs = avm.pop_uint64()?;
    let lhs = avm.pop_uint64()?;
    avm.data_stack.push((lhs >= rhs).into());
    Ok(())
}

fn op_and(avm: &mut Avm) -> Result<(), AvmError> {
    let rhs = avm.pop_uint64()?;
    let lhs = avm.pop_uint64()?;
    avm.data_stack.push((lhs != 0 && rhs != 0).into());
    Ok(())
}

fn op_or(avm: &mut Avm) -> Result<(), AvmError> {
    let rhs = avm.pop_uint64()?;
    let lhs = avm.pop_uint64()?;
    avm.data_stack.push((lhs != 0 || rhs != 0).into());
    Ok(())
}

fn check_eq(lhs: AvmData, rhs: AvmData) -> Result<bool, AvmError> {
    match (lhs, rhs) {
        (AvmData::Uint64(lhs), AvmData::Uint64(rhs)) => Ok(lhs == rhs),
        (AvmData::Bytes(lhs), AvmData::Bytes(rhs)) => Ok(lhs == rhs),
        (AvmData::Uint64(_), AvmData::Bytes(_)) => Err(AvmError::IncompatibleTypes(
            "(uint64,bytes)",
            "(uint64,uint64) or (bytes,bytes)",
        )),
        (AvmData::Bytes(_), AvmData::Uint64(_)) => Err(AvmError::IncompatibleTypes(
            "(bytes,uint64)",
            "(uint64,uint64) or (bytes,bytes)",
        )),
    }
}

fn op_eq(avm: &mut Avm) -> Result<(), AvmError> {
    let rhs = avm.data_stack.pop().ok_or(AvmError::StackUnderflow)?;
    let lhs = avm.data_stack.pop().ok_or(AvmError::StackUnderflow)?;
    let res = if check_eq(lhs, rhs)? { TRUE } else { FALSE };
    avm.data_stack.push(res);
    Ok(())
}

fn op_neq(avm: &mut Avm) -> Result<(), AvmError> {
    let rhs = avm.data_stack.pop().ok_or(AvmError::StackUnderflow)?;
    let lhs = avm.data_stack.pop().ok_or(AvmError::StackUnderflow)?;
    let res = if check_eq(lhs, rhs)? { FALSE } else { TRUE };
    avm.data_stack.push(res);
    Ok(())
}

fn op_neg(avm: &mut Avm) -> Result<(), AvmError> {
    let value = avm.pop_uint64()?;
    avm.data_stack.push(if value == 0 { TRUE } else { FALSE });
    Ok(())
}

fn op_len(avm: &mut Avm) -> Result<(), AvmError> {
    let value = avm.pop_bytes()?;
    avm.data_stack.push((value.len() as u64).into());
    Ok(())
}

fn op_itob(avm: &mut Avm) -> Result<(), AvmError> {
    let value = avm.pop_uint64()?;
    let bytes: AvmData = value.to_be_bytes().to_vec().into();
    avm.data_stack.push(bytes);
    Ok(())
}

fn op_btoi(avm: &mut Avm) -> Result<(), AvmError> {
    let value = avm.pop_bytes()?;
    if value.len() > 8 {
        Err(AvmError::BtoiTooLong(value.len()))
    } else {
        let mut buffer = [0u8; 8];
        buffer[(8 - value.len())..].copy_from_slice(&value[..]);
        avm.data_stack.push(u64::from_be_bytes(buffer).into());
        Ok(())
    }
}

fn op_intcblock(avm: &mut Avm) -> Result<(), AvmError> {
    let nintegers = avm.read_varint()?;
    avm.intc = vec![];
    for _ in 0..nintegers.value {
        let number = avm.read_varint()?;
        avm.intc.push(number.value);
    }
    Ok(())
}

fn op_intc_n(avm: &mut Avm, idx: usize) -> Result<(), AvmError> {
    match avm.intc.get(idx) {
        Some(&val) => {
            avm.data_stack.push(AvmData::Uint64(val));
            Ok(())
        }
        None => Err(AvmError::IntcOutOfRange(idx, avm.intc.len())),
    }
}

fn op_intc(avm: &mut Avm) -> Result<(), AvmError> {
    let idx = avm.read_varint()?;
    op_intc_n(avm, idx.value as usize)
}

fn op_intc_0(avm: &mut Avm) -> Result<(), AvmError> {
    op_intc_n(avm, 0)
}

fn op_intc_1(avm: &mut Avm) -> Result<(), AvmError> {
    op_intc_n(avm, 1)
}

fn op_intc_2(avm: &mut Avm) -> Result<(), AvmError> {
    op_intc_n(avm, 2)
}

fn op_intc_3(avm: &mut Avm) -> Result<(), AvmError> {
    op_intc_n(avm, 3)
}

fn op_bytecblock(avm: &mut Avm) -> Result<(), AvmError> {
    let nbytes = avm.read_varint()?;
    avm.bytec = vec![];
    for _ in 0..nbytes.value {
        let bytes = avm.read_varbytes()?;
        avm.bytec.push(bytes.value);
    }
    Ok(())
}

fn op_bytec_n(avm: &mut Avm, idx: usize) -> Result<(), AvmError> {
    match avm.bytec.get(idx) {
        Some(&val) => {
            avm.data_stack.push(AvmData::Bytes(val.to_vec()));
            Ok(())
        }
        None => Err(AvmError::BytecOutOfRange(idx, avm.bytec.len())),
    }
}

fn op_bytec(avm: &mut Avm) -> Result<(), AvmError> {
    let idx = avm.read_varint()?;
    op_bytec_n(avm, idx.value as usize)
}

fn op_bytec_0(avm: &mut Avm) -> Result<(), AvmError> {
    op_bytec_n(avm, 0)
}

fn op_bytec_1(avm: &mut Avm) -> Result<(), AvmError> {
    op_bytec_n(avm, 1)
}

fn op_bytec_2(avm: &mut Avm) -> Result<(), AvmError> {
    op_bytec_n(avm, 2)
}

fn op_bytec_3(avm: &mut Avm) -> Result<(), AvmError> {
    op_bytec_n(avm, 3)
}

fn op_pop(avm: &mut Avm) -> Result<(), AvmError> {
    match avm.data_stack.pop() {
        Some(_) => Ok(()),
        None => Err(AvmError::EmptyStack),
    }
}

fn op_concat(avm: &mut Avm) -> Result<(), AvmError> {
    let rhs = avm.pop_bytes()?;
    let lhs = avm.pop_bytes()?;
    if lhs.len() + rhs.len() > 4096 {
        Err(AvmError::BytesTooLong)
    } else {
        let concatentation = AvmData::Bytes([lhs, rhs].concat());
        avm.data_stack.push(concatentation);
        Ok(())
    }
}

fn op_pushbytes(avm: &mut Avm) -> Result<(), AvmError> {
    let val: VarBytes = avm.read_varbytes()?;
    avm.data_stack.push(val.into());
    Ok(())
}

fn op_pushint(avm: &mut Avm) -> Result<(), AvmError> {
    let val: VarUint64 = avm.read_varint()?;
    avm.data_stack.push(val.into());
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::avm::execute_program;

    use super::*;

    #[test]
    fn test_plus() -> Result<(), AvmError> {
        // #pragma version 9
        // pushint 1
        // pushint 2
        // +
        let program = vec![0x09, 0x81, 0x01, 0x81, 0x02, 0x08];
        let mut avm = Avm::for_program(&program)?;
        let avm = execute_program(&mut avm)?;

        assert_eq!(1, avm.data_stack.len());
        assert_eq!(Some(AvmData::Uint64(3)), avm.data_stack.pop());
        Ok(())
    }

    #[test]
    fn test_plus_overflow() -> Result<(), AvmError> {
        // #pragma version 9
        // pushint 2**64 - 1
        // pushint 1
        // +
        let program = [
            vec![0x09], // #pragma version 9
            vec![
                0x81, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x01,
            ], // pushint 2**64 - 1
            vec![0x81, 0x01], // pushint 1
            vec![0x08], // +
        ]
        .concat();
        let mut avm = Avm::for_program(&program)?;

        let err = execute_program(&mut avm).unwrap_err();
        assert_eq!(AvmError::IntegerOverflow, err);

        Ok(())
    }

    #[test]
    fn test_minus() -> Result<(), AvmError> {
        // #pragma version 9
        // pushint 5
        // pushint 3
        // -
        let program = vec![0x09, 0x81, 0x05, 0x81, 0x03, 0x09];
        let mut avm = Avm::for_program(&program)?;
        let avm = execute_program(&mut avm)?;

        assert_eq!(1, avm.data_stack.len());
        assert_eq!(Some(AvmData::Uint64(2)), avm.data_stack.pop());
        Ok(())
    }

    #[test]
    fn test_minus_underflow() -> Result<(), AvmError> {
        // #pragma version 9
        // pushint 5
        // pushint 7
        // -
        let program = vec![0x09, 0x81, 0x05, 0x81, 0x07, 0x09];
        let mut avm = Avm::for_program(&program)?;

        let err = execute_program(&mut avm).unwrap_err();
        assert_eq!(AvmError::IntegerUnderflow, err);

        Ok(())
    }

    #[test]
    fn test_div() -> Result<(), AvmError> {
        // #pragma version 9
        // pushint 10
        // pushint 2
        // /
        let program = vec![0x09, 0x81, 0x0a, 0x81, 0x02, 0x0a];
        let mut avm = Avm::for_program(&program)?;
        let avm = execute_program(&mut avm)?;

        assert_eq!(1, avm.data_stack.len());
        assert_eq!(Some(AvmData::Uint64(5)), avm.data_stack.pop());
        Ok(())
    }

    #[test]
    fn test_div_by_zero() -> Result<(), AvmError> {
        // #pragma version 9
        // pushint 10
        // pushint 2
        // /
        let program = vec![0x09, 0x81, 0x0a, 0x81, 0x00, 0x0a];
        let mut avm = Avm::for_program(&program)?;

        let err = execute_program(&mut avm).unwrap_err();
        assert_eq!(AvmError::DivisionByZero, err);

        Ok(())
    }

    #[test]
    fn test_mul() -> Result<(), AvmError> {
        // #pragma version 9
        // pushint 10
        // pushint 2
        // *
        let program = vec![0x09, 0x81, 0x0a, 0x81, 0x2, 0x0b];
        let mut avm = Avm::for_program(&program)?;
        let avm = execute_program(&mut avm)?;

        assert_eq!(1, avm.data_stack.len());
        assert_eq!(Some(AvmData::Uint64(20)), avm.data_stack.pop());
        Ok(())
    }

    #[test]
    fn test_mul_overflow() -> Result<(), AvmError> {
        // #pragma version 9
        // pushint 10
        // pushint 2
        // *
        let program = [
            vec![0x09],                               // #pragma version 9
            vec![0x81, 0x80, 0x80, 0x80, 0x80, 0x10], // pushint 2**32
            vec![0x81, 0x80, 0x80, 0x80, 0x80, 0x10], // pushint 2**32
            vec![0x0b],                               // *
        ]
        .concat();
        let mut avm = Avm::for_program(&program)?;

        let err = execute_program(&mut avm).unwrap_err();
        assert_eq!(AvmError::IntegerOverflow, err);

        Ok(())
    }

    #[test]
    fn test_concat() -> Result<(), AvmError> {
        // #pragma version 9
        // pushbytes 0xdead
        // pushbytes 0xbeef
        // concat
        let program = vec![0x09, 0x80, 0x02, 0xde, 0xad, 0x80, 0x02, 0xbe, 0xef, 0x50];
        let mut avm = Avm::for_program(&program)?;
        let avm = execute_program(&mut avm)?;

        assert_eq!(1, avm.data_stack.len());
        assert_eq!(
            Some(AvmData::Bytes(vec![0xde, 0xad, 0xbe, 0xef])),
            avm.data_stack.pop()
        );
        Ok(())
    }

    #[test]
    fn test_concat_overflow() -> Result<(), AvmError> {
        // #pragma version 9
        // pushbytes (4000 bytes)
        // pushbytes (100 bytes)
        // concat
        let program = [
            vec![0x09],       // #pragma version 9
            vec![0x80],       // pushbytes
            vec![0xA0, 0x1F], // 4000 as varint
            vec![0; 4000],    // 4000 0-bytes
            vec![0x80],       // pushbytes
            vec![0x64],       // 100 as varint
            vec![0; 100],     // 4000 0-bytes
            vec![0x50],       // concat
        ]
        .concat();
        let mut avm = Avm::for_program(&program)?;

        let err = execute_program(&mut avm).unwrap_err();
        assert_eq!(AvmError::BytesTooLong, err);
        Ok(())
    }

    #[test]
    fn test_err() -> Result<(), AvmError> {
        // #pragma version 9
        // err
        let program = [0x09, 0x00];
        let mut avm = Avm::for_program(&program)?;

        let err = execute_program(&mut avm).unwrap_err();
        assert_eq!(AvmError::ErrOpCode, err);
        Ok(())
    }

    #[test]
    fn test_intcblock() -> Result<(), AvmError> {
        // #pragma version 9
        // intcblock 10 20 30 40
        // intc_1
        // intc_3
        let program = [0x09, 0x20, 0x04, 0x0a, 0x14, 0x1e, 0x28, 0x23, 0x25];
        let mut avm = Avm::for_program(&program)?;
        let avm = execute_program(&mut avm)?;

        assert_eq!(2, avm.data_stack.len());
        assert_eq!(Some(AvmData::Uint64(40)), avm.data_stack.pop());
        assert_eq!(Some(AvmData::Uint64(20)), avm.data_stack.pop());
        Ok(())
    }

    #[test]
    fn test_bytecblock() -> Result<(), AvmError> {
        // #pragma version 9
        // bytecblock 0xdead 0xbe 0xef
        // bytec_0
        // bytec_2
        let program = [
            0x09, 0x26, 0x03, 0x02, 0xde, 0xad, 0x01, 0xbe, 0x01, 0xef, 0x28, 0x2a,
        ];
        let mut avm = Avm::for_program(&program)?;
        let avm = execute_program(&mut avm)?;

        assert_eq!(2, avm.data_stack.len());
        assert_eq!(Some(AvmData::Bytes(vec![0xef])), avm.data_stack.pop());
        assert_eq!(Some(AvmData::Bytes(vec![0xde, 0xad])), avm.data_stack.pop());
        Ok(())
    }

    #[test]
    fn test_lt() -> Result<(), AvmError> {
        // #pragma version 9
        // pushint 1
        // pushint 2
        // <
        let program = [0x09, 0x81, 0x01, 0x81, 0x02, 0x0c];
        let mut avm = Avm::for_program(&program)?;
        let avm = execute_program(&mut avm)?;

        assert_eq!(1, avm.data_stack.len());
        assert_eq!(Some(TRUE), avm.data_stack.pop());

        // #pragma version 9
        // pushint 2
        // pushint 1
        // <
        let program = [0x09, 0x81, 0x02, 0x81, 0x01, 0x0c];
        let mut avm = Avm::for_program(&program)?;
        let avm = execute_program(&mut avm)?;

        assert_eq!(1, avm.data_stack.len());
        assert_eq!(Some(FALSE), avm.data_stack.pop());
        Ok(())
    }

    #[test]
    fn test_gt() -> Result<(), AvmError> {
        // #pragma version 9
        // pushint 1
        // pushint 2
        // >
        let program = [0x09, 0x81, 0x01, 0x81, 0x02, 0x0d];
        let mut avm = Avm::for_program(&program)?;
        let avm = execute_program(&mut avm)?;

        assert_eq!(1, avm.data_stack.len());
        assert_eq!(Some(FALSE), avm.data_stack.pop());

        // #pragma version 9
        // pushint 1
        // pushint 2
        // >
        let program = [0x09, 0x81, 0x02, 0x81, 0x01, 0x0d];
        let mut avm = Avm::for_program(&program)?;
        let avm = execute_program(&mut avm)?;

        assert_eq!(1, avm.data_stack.len());
        assert_eq!(Some(TRUE), avm.data_stack.pop());
        Ok(())
    }

    #[test]
    fn test_leq() -> Result<(), AvmError> {
        // #pragma version 9
        // pushint 1
        // pushint 2
        // <=
        let program = [0x09, 0x81, 0x01, 0x81, 0x02, 0x0e];
        let mut avm = Avm::for_program(&program)?;
        let avm = execute_program(&mut avm)?;

        assert_eq!(1, avm.data_stack.len());
        assert_eq!(Some(TRUE), avm.data_stack.pop());

        // #pragma version 9
        // pushint 2
        // pushint 2
        // <=
        let program = [0x09, 0x81, 0x02, 0x81, 0x02, 0x0e];
        let mut avm = Avm::for_program(&program)?;
        let avm = execute_program(&mut avm)?;

        assert_eq!(1, avm.data_stack.len());
        assert_eq!(Some(TRUE), avm.data_stack.pop());

        // #pragma version 9
        // pushint 3
        // pushint 2
        // <=
        let program = [0x09, 0x81, 0x03, 0x81, 0x02, 0x0e];
        let mut avm = Avm::for_program(&program)?;
        let avm = execute_program(&mut avm)?;

        assert_eq!(1, avm.data_stack.len());
        assert_eq!(Some(FALSE), avm.data_stack.pop());
        Ok(())
    }

    #[test]
    fn test_geq() -> Result<(), AvmError> {
        // #pragma version 9
        // pushint 1
        // pushint 2
        // >=
        let program = [0x09, 0x81, 0x01, 0x81, 0x02, 0x0f];
        let mut avm = Avm::for_program(&program)?;
        let avm = execute_program(&mut avm)?;

        assert_eq!(1, avm.data_stack.len());
        assert_eq!(Some(FALSE), avm.data_stack.pop());

        // #pragma version 9
        // pushint 2
        // pushint 2
        // >=
        let program = [0x09, 0x81, 0x02, 0x81, 0x02, 0x0f];
        let mut avm = Avm::for_program(&program)?;
        let avm = execute_program(&mut avm)?;

        assert_eq!(1, avm.data_stack.len());
        assert_eq!(Some(TRUE), avm.data_stack.pop());

        // #pragma version 9
        // pushint 3
        // pushint 2
        // >=
        let program = [0x09, 0x81, 0x03, 0x81, 0x02, 0x0f];
        let mut avm = Avm::for_program(&program)?;
        let avm = execute_program(&mut avm)?;

        assert_eq!(1, avm.data_stack.len());
        assert_eq!(Some(TRUE), avm.data_stack.pop());
        Ok(())
    }

    #[test]
    fn test_and() -> Result<(), AvmError> {
        // #pragma version 9
        // pushint 1
        // pushint 2
        // &&
        let program = [0x09, 0x81, 0x01, 0x81, 0x02, 0x10];
        let mut avm = Avm::for_program(&program)?;
        let avm = execute_program(&mut avm)?;

        assert_eq!(1, avm.data_stack.len());
        assert_eq!(Some(TRUE), avm.data_stack.pop());

        // #pragma version 9
        // pushint 0
        // pushint 2
        // &&
        let program = [0x09, 0x81, 0x00, 0x81, 0x02, 0x10];
        let mut avm = Avm::for_program(&program)?;
        let avm = execute_program(&mut avm)?;

        assert_eq!(1, avm.data_stack.len());
        assert_eq!(Some(FALSE), avm.data_stack.pop());

        Ok(())
    }

    #[test]
    fn test_or() -> Result<(), AvmError> {
        // #pragma version 9
        // pushint 0
        // pushint 2
        // ||
        let program = [0x09, 0x81, 0x00, 0x81, 0x02, 0x11];
        let mut avm = Avm::for_program(&program)?;
        let avm = execute_program(&mut avm)?;

        assert_eq!(1, avm.data_stack.len());
        assert_eq!(Some(TRUE), avm.data_stack.pop());

        // #pragma version 9
        // pushint 0
        // pushint 0
        // ||
        let program = [0x09, 0x81, 0x00, 0x81, 0x00, 0x11];
        let mut avm = Avm::for_program(&program)?;
        let avm = execute_program(&mut avm)?;

        assert_eq!(1, avm.data_stack.len());
        assert_eq!(Some(FALSE), avm.data_stack.pop());

        Ok(())
    }

    #[test]
    fn test_eq() -> Result<(), AvmError> {
        // #pragma version 9
        // pushint 0
        // pushint 0
        // ==
        let program = [0x09, 0x81, 0x00, 0x81, 0x00, 0x12];
        let mut avm = Avm::for_program(&program)?;
        let avm = execute_program(&mut avm)?;
        assert_eq!(1, avm.data_stack.len());
        assert_eq!(Some(TRUE), avm.data_stack.pop());

        // #pragma version 9
        // pushint 0
        // pushint 1
        // ==
        let program = [0x09, 0x81, 0x00, 0x81, 0x01, 0x12];
        let mut avm = Avm::for_program(&program)?;
        let avm = execute_program(&mut avm)?;
        assert_eq!(1, avm.data_stack.len());
        assert_eq!(Some(FALSE), avm.data_stack.pop());

        // #pragma version 9
        // pushbytes 0xdead
        // pushbytes 0xdead
        // ==
        let program = [0x09, 0x80, 0x02, 0xde, 0xad, 0x80, 0x02, 0xde, 0xad, 0x12];
        let mut avm = Avm::for_program(&program)?;
        let avm = execute_program(&mut avm)?;
        assert_eq!(1, avm.data_stack.len());
        assert_eq!(Some(TRUE), avm.data_stack.pop());

        // #pragma version 9
        // pushbytes 0xdead
        // pushbytes 0xbeef
        // ==
        let program = [0x09, 0x80, 0x02, 0xde, 0xad, 0x80, 0x02, 0xbe, 0xef, 0x12];
        let mut avm = Avm::for_program(&program)?;
        let avm = execute_program(&mut avm)?;
        assert_eq!(1, avm.data_stack.len());
        assert_eq!(Some(FALSE), avm.data_stack.pop());

        Ok(())
    }

    #[test]
    fn test_eq_on_incompatible_types() -> Result<(), AvmError> {
        // #pragma version 9
        // pushint 0
        // pushbytes 0xdead
        // ==
        let program = [0x09, 0x81, 0x00, 0x80, 0x02, 0xde, 0xad, 0x12];
        let mut avm = Avm::for_program(&program)?;

        let err = execute_program(&mut avm).unwrap_err();
        assert_eq!(
            AvmError::IncompatibleTypes("(uint64,bytes)", "(uint64,uint64) or (bytes,bytes)"),
            err
        );
        Ok(())
    }

    #[test]
    fn test_neq() -> Result<(), AvmError> {
        // #pragma version 9
        // pushint 0
        // pushint 0
        // !=
        let program = [0x09, 0x81, 0x00, 0x81, 0x00, 0x13];
        let mut avm = Avm::for_program(&program)?;
        let avm = execute_program(&mut avm)?;
        assert_eq!(1, avm.data_stack.len());
        assert_eq!(Some(FALSE), avm.data_stack.pop());

        // #pragma version 9
        // pushint 0
        // pushint 1
        // !=
        let program = [0x09, 0x81, 0x00, 0x81, 0x01, 0x13];
        let mut avm = Avm::for_program(&program)?;
        let avm = execute_program(&mut avm)?;
        assert_eq!(1, avm.data_stack.len());
        assert_eq!(Some(TRUE), avm.data_stack.pop());

        // #pragma version 9
        // pushbytes 0xdead
        // pushbytes 0xdead
        // !=
        let program = [0x09, 0x80, 0x02, 0xde, 0xad, 0x80, 0x02, 0xde, 0xad, 0x13];
        let mut avm = Avm::for_program(&program)?;
        let avm = execute_program(&mut avm)?;
        assert_eq!(1, avm.data_stack.len());
        assert_eq!(Some(FALSE), avm.data_stack.pop());

        // #pragma version 9
        // pushbytes 0xdead
        // pushbytes 0xbeef
        // !=
        let program = [0x09, 0x80, 0x02, 0xde, 0xad, 0x80, 0x02, 0xbe, 0xef, 0x13];
        let mut avm = Avm::for_program(&program)?;
        let avm = execute_program(&mut avm)?;
        assert_eq!(1, avm.data_stack.len());
        assert_eq!(Some(TRUE), avm.data_stack.pop());

        Ok(())
    }

    #[test]
    fn test_neq_on_incompatible_types() -> Result<(), AvmError> {
        // #pragma version 9
        // pushint 0
        // pushbytes 0xdead
        // !=
        let program = [0x09, 0x81, 0x00, 0x80, 0x02, 0xde, 0xad, 0x13];
        let mut avm = Avm::for_program(&program)?;

        let err = execute_program(&mut avm).unwrap_err();
        assert_eq!(
            AvmError::IncompatibleTypes("(uint64,bytes)", "(uint64,uint64) or (bytes,bytes)"),
            err
        );
        Ok(())
    }

    #[test]
    fn test_neg() -> Result<(), AvmError> {
        // #pragma version 9
        // pushint 0
        // !
        let program = [0x09, 0x81, 0x00, 0x14];
        let mut avm = Avm::for_program(&program)?;
        let avm = execute_program(&mut avm)?;
        assert_eq!(1, avm.data_stack.len());
        assert_eq!(Some(TRUE), avm.data_stack.pop());

        // #pragma version 9
        // pushint 1
        // !
        let program = [0x09, 0x81, 0x01, 0x14];
        let mut avm = Avm::for_program(&program)?;
        let avm = execute_program(&mut avm)?;
        assert_eq!(1, avm.data_stack.len());
        assert_eq!(Some(FALSE), avm.data_stack.pop());

        // #pragma version 9
        // pushint 2
        // !
        let program = [0x09, 0x81, 0x02, 0x14];
        let mut avm = Avm::for_program(&program)?;
        let avm = execute_program(&mut avm)?;
        assert_eq!(1, avm.data_stack.len());
        assert_eq!(Some(FALSE), avm.data_stack.pop());

        Ok(())
    }

    #[test]
    fn test_len() -> Result<(), AvmError> {
        // #pragma version 9
        // pushbytes (empty string)
        // len
        let program = [0x09, 0x80, 0x00, 0x15];
        let mut avm = Avm::for_program(&program)?;
        let avm = execute_program(&mut avm)?;
        assert_eq!(1, avm.data_stack.len());
        assert_eq!(Some(AvmData::Uint64(0)), avm.data_stack.pop());

        // #pragma version 9
        // pushbytes 0xde
        // len
        let program = [0x09, 0x80, 0x01, 0xde, 0x15];
        let mut avm = Avm::for_program(&program)?;
        let avm = execute_program(&mut avm)?;
        assert_eq!(1, avm.data_stack.len());
        assert_eq!(Some(AvmData::Uint64(1)), avm.data_stack.pop());

        // #pragma version 9
        // pushbytes 0xdead
        // len
        let program = [0x09, 0x80, 0x02, 0xde, 0xad, 0x15];
        let mut avm = Avm::for_program(&program)?;
        let avm = execute_program(&mut avm)?;
        assert_eq!(1, avm.data_stack.len());
        assert_eq!(Some(AvmData::Uint64(2)), avm.data_stack.pop());

        Ok(())
    }

    #[test]
    fn test_itob() -> Result<(), AvmError> {
        // #pragma version 9
        // pushint 0
        // itob
        let program = [0x09, 0x81, 0x00, 0x16];
        let mut avm = Avm::for_program(&program)?;
        let avm = execute_program(&mut avm)?;
        assert_eq!(1, avm.data_stack.len());
        assert_eq!(
            Some(AvmData::Bytes(vec![
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00
            ])),
            avm.data_stack.pop()
        );

        // #pragma version 9
        // pushint 45647561321464
        // itob
        let program = [0x09, 0x81, 0xF8, 0xD7, 0xB5, 0xB5, 0xC2, 0xB0, 0x0A, 0x16];
        let mut avm = Avm::for_program(&program)?;
        let avm = execute_program(&mut avm)?;
        assert_eq!(1, avm.data_stack.len());
        assert_eq!(
            Some(AvmData::Bytes(vec![
                0x00, 0x00, 0x29, 0x84, 0x26, 0xAD, 0x6B, 0xF8,
            ])),
            avm.data_stack.pop()
        );

        // #pragma version 9
        // pushint
        // itob
        let program = [
            0x09, 0x81, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x01, 0x16,
        ];
        let mut avm = Avm::for_program(&program)?;
        let avm = execute_program(&mut avm)?;
        assert_eq!(1, avm.data_stack.len());
        assert_eq!(
            Some(AvmData::Bytes(vec![
                0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF
            ])),
            avm.data_stack.pop()
        );

        Ok(())
    }

    #[test]
    fn test_btoi() -> Result<(), AvmError> {
        // #pragma version 9
        // pushbytes (empty string)
        // btoi
        let program = [0x09, 0x80, 0x00, 0x17];
        let mut avm = Avm::for_program(&program)?;
        let avm = execute_program(&mut avm)?;
        assert_eq!(1, avm.data_stack.len());
        assert_eq!(Some(AvmData::Uint64(0)), avm.data_stack.pop());

        // #pragma version 9
        // pushbytes 0xde
        // btoi
        let program = [0x09, 0x80, 0x01, 0xde, 0x17];
        let mut avm = Avm::for_program(&program)?;
        let avm = execute_program(&mut avm)?;
        assert_eq!(1, avm.data_stack.len());
        assert_eq!(Some(AvmData::Uint64(222)), avm.data_stack.pop());

        // #pragma version 9
        // pushbytes 0xdeadbeef
        // btoi
        let program = [0x09, 0x80, 0x04, 0xde, 0xad, 0xbe, 0xef, 0x17];
        let mut avm = Avm::for_program(&program)?;
        let avm = execute_program(&mut avm)?;
        assert_eq!(1, avm.data_stack.len());
        assert_eq!(Some(AvmData::Uint64(3735928559)), avm.data_stack.pop());

        // #pragma version 9
        // pushbytes 0xdeadbeefdeadbeefaa
        // btoi
        let program = [
            0x09, 0x80, 0x09, 0xde, 0xad, 0xbe, 0xef, 0xde, 0xad, 0xbe, 0xef, 0xaa, 0x17,
        ];
        let mut avm = Avm::for_program(&program)?;
        let err = execute_program(&mut avm).unwrap_err();
        assert_eq!(AvmError::BtoiTooLong(9), err);
        Ok(())
    }
}
