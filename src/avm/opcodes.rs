use crate::{
    encoding::{self, VarBytes, VarUint64},
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

pub const OP_SPECS: [OpSpec; 62] = [
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
        opcode: 0x18,
        name: "%",
        version: AvmVersion::V1,
        cost: 1,
        eval: op_mod,
    },
    OpSpec {
        opcode: 0x19,
        name: "|",
        version: AvmVersion::V1,
        cost: 1,
        eval: op_bit_or,
    },
    OpSpec {
        opcode: 0x1a,
        name: "&",
        version: AvmVersion::V1,
        cost: 1,
        eval: op_bit_and,
    },
    OpSpec {
        opcode: 0x1b,
        name: "^",
        version: AvmVersion::V1,
        cost: 1,
        eval: op_bit_xor,
    },
    OpSpec {
        opcode: 0x1c,
        name: "~",
        version: AvmVersion::V1,
        cost: 1,
        eval: op_bit_not,
    },
    OpSpec {
        opcode: 0x1d,
        name: "mulw",
        version: AvmVersion::V1,
        cost: 1,
        eval: op_mulw,
    },
    OpSpec {
        opcode: 0x1e,
        name: "addw",
        version: AvmVersion::V2,
        cost: 1,
        eval: op_addw,
    },
    OpSpec {
        opcode: 0x1f,
        name: "divmodw",
        version: AvmVersion::V4,
        cost: 20,
        eval: op_divmodw,
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
        opcode: 0x34,
        name: "load",
        version: AvmVersion::V1,
        cost: 1,
        eval: op_load,
    },
    OpSpec {
        opcode: 0x35,
        name: "store",
        version: AvmVersion::V1,
        cost: 1,
        eval: op_store,
    },
    OpSpec {
        opcode: 0x3e,
        name: "loads",
        version: AvmVersion::V5,
        cost: 1,
        eval: op_loads,
    },
    OpSpec {
        opcode: 0x3f,
        name: "stores",
        version: AvmVersion::V5,
        cost: 1,
        eval: op_stores,
    },
    OpSpec {
        opcode: 0x40,
        name: "bnz",
        version: AvmVersion::V1,
        cost: 1,
        eval: op_bnz,
    },
    OpSpec {
        opcode: 0x41,
        name: "bz",
        version: AvmVersion::V2,
        cost: 1,
        eval: op_bz,
    },
    OpSpec {
        opcode: 0x42,
        name: "b",
        version: AvmVersion::V2,
        cost: 1,
        eval: op_b,
    },
    OpSpec {
        opcode: 0x43,
        name: "return",
        version: AvmVersion::V2,
        cost: 1,
        eval: op_return,
    },
    OpSpec {
        opcode: 0x44,
        name: "assert",
        version: AvmVersion::V3,
        cost: 1,
        eval: op_assert,
    },
    OpSpec {
        opcode: 0x45,
        name: "bury",
        version: AvmVersion::V8,
        cost: 1,
        eval: op_bury,
    },
    OpSpec {
        opcode: 0x46,
        name: "popn",
        version: AvmVersion::V8,
        cost: 1,
        eval: op_popn,
    },
    OpSpec {
        opcode: 0x47,
        name: "dupn",
        version: AvmVersion::V8,
        cost: 1,
        eval: op_dupn,
    },
    OpSpec {
        opcode: 0x48,
        name: "pop",
        version: AvmVersion::V1,
        cost: 1,
        eval: op_pop,
    },
    OpSpec {
        opcode: 0x49,
        name: "dup",
        version: AvmVersion::V1,
        cost: 1,
        eval: op_dup,
    },
    OpSpec {
        opcode: 0x4a,
        name: "dup2",
        version: AvmVersion::V2,
        cost: 1,
        eval: op_dup2,
    },
    OpSpec {
        opcode: 0x4b,
        name: "dig",
        version: AvmVersion::V3,
        cost: 1,
        eval: op_dig,
    },
    OpSpec {
        opcode: 0x4c,
        name: "swap",
        version: AvmVersion::V3,
        cost: 1,
        eval: op_swap,
    },
    OpSpec {
        opcode: 0x4d,
        name: "select",
        version: AvmVersion::V3,
        cost: 1,
        eval: op_select,
    },
    OpSpec {
        opcode: 0x4e,
        name: "cover",
        version: AvmVersion::V5,
        cost: 1,
        eval: op_cover,
    },
    OpSpec {
        opcode: 0x4f,
        name: "uncover",
        version: AvmVersion::V5,
        cost: 1,
        eval: op_uncover,
    },
    OpSpec {
        opcode: 0x50,
        name: "concat",
        version: AvmVersion::V2,
        cost: 1,
        eval: op_concat,
    },
    OpSpec {
        opcode: 0x51,
        name: "substring",
        version: AvmVersion::V2,
        cost: 1,
        eval: op_substring,
    },
    OpSpec {
        opcode: 0x52,
        name: "substring3",
        version: AvmVersion::V2,
        cost: 1,
        eval: op_substring3,
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

fn op_mod(avm: &mut Avm) -> Result<(), AvmError> {
    let rhs = avm.pop_uint64()?;
    let lhs = avm.pop_uint64()?;
    match lhs.checked_rem(rhs) {
        Some(res) => {
            avm.data_stack.push(res.into());
            Ok(())
        }
        None => Err(AvmError::DivisionByZero),
    }
}

fn op_bit_or(avm: &mut Avm) -> Result<(), AvmError> {
    let rhs = avm.pop_uint64()?;
    let lhs = avm.pop_uint64()?;
    avm.data_stack.push((lhs | rhs).into());
    Ok(())
}

fn op_bit_and(avm: &mut Avm) -> Result<(), AvmError> {
    let rhs = avm.pop_uint64()?;
    let lhs = avm.pop_uint64()?;
    avm.data_stack.push((lhs & rhs).into());
    Ok(())
}

fn op_bit_xor(avm: &mut Avm) -> Result<(), AvmError> {
    let rhs = avm.pop_uint64()?;
    let lhs = avm.pop_uint64()?;
    avm.data_stack.push((lhs ^ rhs).into());
    Ok(())
}

fn op_bit_not(avm: &mut Avm) -> Result<(), AvmError> {
    let value = avm.pop_uint64()?;
    avm.data_stack.push((!value).into());
    Ok(())
}

fn op_mulw(avm: &mut Avm) -> Result<(), AvmError> {
    let b = avm.pop_uint64()?;
    let a = avm.pop_uint64()?;
    let (hi, lo) = encoding::u128_to_u64_tuple((a as u128) * (b as u128));
    avm.data_stack.push(hi.into());
    avm.data_stack.push(lo.into());
    Ok(())
}

fn op_addw(avm: &mut Avm) -> Result<(), AvmError> {
    let b = avm.pop_uint64()?;
    let a = avm.pop_uint64()?;
    let (hi, lo) = encoding::u128_to_u64_tuple((a as u128) + (b as u128));
    avm.data_stack.push(hi.into());
    avm.data_stack.push(lo.into());
    Ok(())
}

fn op_divmodw(avm: &mut Avm) -> Result<(), AvmError> {
    let d = avm.pop_uint64()?;
    let c = avm.pop_uint64()?;
    let b = avm.pop_uint64()?;
    let a = avm.pop_uint64()?;
    // compute a||b and c||d (bytewise-concatenation)
    let ab = encoding::u64_tuple_to_u128(a, b);
    let cd = encoding::u64_tuple_to_u128(c, d);
    // compute result (division, modulo)
    let wx = ab / cd;
    let yz = ab % cd;
    // split u128s into u64s
    let (w, x) = encoding::u128_to_u64_tuple(wx);
    let (y, z) = encoding::u128_to_u64_tuple(yz);
    // put results onto stack
    avm.data_stack.push(w.into());
    avm.data_stack.push(x.into());
    avm.data_stack.push(y.into());
    avm.data_stack.push(z.into());
    Ok(())
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

fn op_load(avm: &mut Avm) -> Result<(), AvmError> {
    // we do not need range checking since scratch has length 256
    // and is indexed by a single byte
    let pos = avm.read_byte()? as usize;
    avm.data_stack.push(avm.scratch[pos].clone());
    Ok(())
}

fn op_store(avm: &mut Avm) -> Result<(), AvmError> {
    // we do not need range checking since scratch has length 256
    // and is indexed by a single byte
    let pos = avm.read_byte()? as usize;
    avm.scratch[pos] = avm.pop_any()?;
    Ok(())
}

fn op_loads(avm: &mut Avm) -> Result<(), AvmError> {
    let pos = avm.pop_uint64()? as usize;
    if pos >= avm.scratch.len() {
        return Err(AvmError::ScratchAccessOutOfBounds(pos));
    }
    avm.data_stack.push(avm.scratch[pos].clone());
    Ok(())
}

fn op_stores(avm: &mut Avm) -> Result<(), AvmError> {
    let value = avm.pop_any()?;
    let pos = avm.pop_uint64()? as usize;
    if pos >= avm.scratch.len() {
        return Err(AvmError::ScratchAccessOutOfBounds(pos));
    }
    avm.scratch[pos] = value;
    Ok(())
}

fn op_bnz(avm: &mut Avm) -> Result<(), AvmError> {
    // do not move read_i16 into if branch, it moves the program counter
    let offset = avm.read_i16()?;
    if avm.pop_uint64()? != 0 {
        branch_to_offset(avm, offset)?;
    }
    Ok(())
}

fn op_bz(avm: &mut Avm) -> Result<(), AvmError> {
    // do not move read_i16 into if branch, it moves the program counter
    let offset = avm.read_i16()?;
    if avm.pop_uint64()? == 0 {
        branch_to_offset(avm, offset)?;
    }
    Ok(())
}

fn op_b(avm: &mut Avm) -> Result<(), AvmError> {
    let offset = avm.read_i16()?;
    branch_to_offset(avm, offset)
}

fn branch_to_offset(avm: &mut Avm, offset: i16) -> Result<(), AvmError> {
    let target: i64 = avm.pc as i64 + (offset as i64);
    if target < 0 || target > (avm.program.len() as i64) {
        Err(AvmError::PcOutOfBounds)
    } else {
        // TODO: we also have to check that we are jumping
        // to the beginning of an opcode
        avm.pc = target as usize;
        Ok(())
    }
}

fn op_return(avm: &mut Avm) -> Result<(), AvmError> {
    let value = avm.pop_uint64()?;
    avm.data_stack.clear();
    avm.data_stack.push(AvmData::Uint64(value));
    avm.pc = avm.program.len();
    Ok(())
}

fn op_assert(avm: &mut Avm) -> Result<(), AvmError> {
    if avm.pop_uint64()? == 0 {
        Err(AvmError::AssertionFailed(avm.pc - 1))
    } else {
        Ok(())
    }
}

fn op_bury(avm: &mut Avm) -> Result<(), AvmError> {
    let len = avm.data_stack.len();
    let offset = avm.read_byte()? as usize;
    if offset > len || offset == 0 {
        Err(AvmError::InvalidStackAccess)
    } else {
        avm.data_stack[len - offset] = avm.pop_any()?;
        Ok(())
    }
}

fn op_popn(avm: &mut Avm) -> Result<(), AvmError> {
    let n = avm.read_byte()? as usize;
    if n > avm.data_stack.len() {
        Err(AvmError::StackUnderflow)
    } else {
        for _ in 0..n {
            avm.pop_any()?;
        }
        Ok(())
    }
}

fn op_dupn(avm: &mut Avm) -> Result<(), AvmError> {
    let n = avm.read_byte()? as usize;
    avm.data_stack.reserve(n);
    let value = avm.pop_any()?;
    for _ in 0..=n {
        avm.data_stack.push(value.clone());
    }
    Ok(())
}

fn op_pop(avm: &mut Avm) -> Result<(), AvmError> {
    match avm.data_stack.pop() {
        Some(_) => Ok(()),
        None => Err(AvmError::EmptyStack),
    }
}

fn op_dup(avm: &mut Avm) -> Result<(), AvmError> {
    match avm.data_stack.last() {
        None => Err(AvmError::StackUnderflow),
        Some(v) => {
            avm.data_stack.push(v.clone());
            Ok(())
        }
    }
}

fn op_dup2(avm: &mut Avm) -> Result<(), AvmError> {
    let b = avm.pop_any()?;
    let a = avm.pop_any()?;
    avm.data_stack.push(a.clone());
    avm.data_stack.push(b.clone());
    avm.data_stack.push(a);
    avm.data_stack.push(b);
    Ok(())
}

fn op_dig(avm: &mut Avm) -> Result<(), AvmError> {
    let n = avm.read_byte()? as usize;
    if n >= avm.data_stack.len() {
        Err(AvmError::InvalidStackAccess)
    } else {
        let value = &avm.data_stack[avm.data_stack.len() - 1 - n];
        avm.data_stack.push(value.clone());
        Ok(())
    }
}

fn op_swap(avm: &mut Avm) -> Result<(), AvmError> {
    let b = avm.pop_any()?;
    let a = avm.pop_any()?;
    avm.data_stack.push(b);
    avm.data_stack.push(a);
    Ok(())
}

fn op_select(avm: &mut Avm) -> Result<(), AvmError> {
    let c = avm.pop_uint64()?;
    let b = avm.pop_any()?;
    let a = avm.pop_any()?;
    if c == 0 {
        avm.data_stack.push(a);
    } else {
        avm.data_stack.push(b);
    }
    Ok(())
}

fn op_cover(avm: &mut Avm) -> Result<(), AvmError> {
    let depth = avm.read_byte()? as usize;
    if depth < avm.data_stack.len() {
        let pos = avm.data_stack.len() - 1 - depth;
        let value = avm.pop_any()?;
        avm.data_stack.insert(pos, value);
        Ok(())
    } else {
        Err(AvmError::InvalidStackAccess)
    }
}

fn op_uncover(avm: &mut Avm) -> Result<(), AvmError> {
    let depth = avm.read_byte()? as usize;
    if depth < avm.data_stack.len() {
        let pos = avm.data_stack.len() - 1 - depth;
        let value = avm.data_stack.remove(pos);
        avm.data_stack.push(value);
        Ok(())
    } else {
        Err(AvmError::InvalidStackAccess)
    }
}

fn op_concat(avm: &mut Avm) -> Result<(), AvmError> {
    let rhs = avm.pop_bytes()?;
    let lhs = avm.pop_bytes()?;
    if lhs.len() + rhs.len() > 4096 {
        Err(AvmError::BytesTooLong)
    } else {
        avm.data_stack.push([lhs, rhs].concat().into());
        Ok(())
    }
}

fn op_substring(avm: &mut Avm) -> Result<(), AvmError> {
    let bytes = avm.pop_bytes()?;
    let start = avm.read_byte()? as usize;
    let end = avm.read_byte()? as usize;
    substring(avm, &bytes, start, end)
}

fn op_substring3(avm: &mut Avm) -> Result<(), AvmError> {
    let end = avm.pop_uint64()? as usize;
    let start = avm.pop_uint64()? as usize;
    let bytes = avm.pop_bytes()?;
    substring(avm, &bytes, start, end)
}

fn substring(avm: &mut Avm, bytes: &[u8], start: usize, end: usize) -> Result<(), AvmError> {
    if end < start || end > bytes.len() {
        Err(AvmError::InvalidSubstringAccess(start, end, bytes.len()))
    } else {
        let substring = bytes[start..end].to_vec();
        avm.data_stack.push(AvmData::Bytes(substring));
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
    use std::vec;

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
        // pushint 0
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

    #[test]
    fn test_mod() -> Result<(), AvmError> {
        // #pragma version 9
        // pushint 10
        // pushint 4
        // %
        let program = vec![0x09, 0x81, 0x0a, 0x81, 0x04, 0x18];
        let mut avm = Avm::for_program(&program)?;
        let avm = execute_program(&mut avm)?;

        assert_eq!(1, avm.data_stack.len());
        assert_eq!(Some(AvmData::Uint64(2)), avm.data_stack.pop());
        Ok(())
    }

    #[test]
    fn test_mod_by_zero() -> Result<(), AvmError> {
        // #pragma version 9
        // pushint 10
        // pushint 0
        // /
        let program = vec![0x09, 0x81, 0x0a, 0x81, 0x00, 0x18];
        let mut avm = Avm::for_program(&program)?;

        let err = execute_program(&mut avm).unwrap_err();
        assert_eq!(AvmError::DivisionByZero, err);

        Ok(())
    }

    #[test]
    fn test_bit_or() -> Result<(), AvmError> {
        // #pragma version 9
        // pushint 0b000000001000100110111101 (0xBD9302)
        // pushint 0b110010011011101100100011 (0xA3F6A606)
        // |
        let program = vec![
            0x09, 0x81, 0xBD, 0x93, 0x02, 0x81, 0xA3, 0xF6, 0xA6, 0x06, 0x19,
        ];
        let mut avm = Avm::for_program(&program)?;
        let avm = execute_program(&mut avm)?;

        // result:
        // 0b110010011011101110111111
        assert_eq!(1, avm.data_stack.len());
        assert_eq!(Some(AvmData::Uint64(13220799)), avm.data_stack.pop());
        Ok(())
    }

    #[test]
    fn test_bit_and() -> Result<(), AvmError> {
        // #pragma version 9
        // pushint 0b000000001000100110111101 (0xBD9302)
        // pushint 0b110010011011101100100011 (0xA3F6A606)
        // &
        let program = vec![
            0x09, 0x81, 0xBD, 0x93, 0x02, 0x81, 0xA3, 0xF6, 0xA6, 0x06, 0x1a,
        ];
        let mut avm = Avm::for_program(&program)?;
        let avm = execute_program(&mut avm)?;

        // result:
        // 0b1000100100100001
        assert_eq!(1, avm.data_stack.len());
        assert_eq!(Some(AvmData::Uint64(35105)), avm.data_stack.pop());
        Ok(())
    }

    #[test]
    fn test_bit_xor() -> Result<(), AvmError> {
        // #pragma version 9
        // pushint 0b000000001000100110111101 (0xBD9302)
        // pushint 0b110010011011101100100011 (0xA3F6A606)
        // ^
        let program = vec![
            0x09, 0x81, 0xBD, 0x93, 0x02, 0x81, 0xA3, 0xF6, 0xA6, 0x06, 0x1b,
        ];
        let mut avm = Avm::for_program(&program)?;
        let avm = execute_program(&mut avm)?;

        // result:
        // 0b110010010011001010011110
        assert_eq!(1, avm.data_stack.len());
        assert_eq!(Some(AvmData::Uint64(13185694)), avm.data_stack.pop());
        Ok(())
    }

    #[test]
    fn test_bit_not() -> Result<(), AvmError> {
        // #pragma version 9
        // pushint 0b000000001000100110111101 (0xBD9302)
        // !
        let program = vec![0x09, 0x81, 0xBD, 0x93, 0x02, 0x1c];
        let mut avm = Avm::for_program(&program)?;
        let avm = execute_program(&mut avm)?;

        // result:
        // 0b1111111111111111111111111111111111111111111111110111011001000010
        assert_eq!(1, avm.data_stack.len());
        assert_eq!(
            Some(AvmData::Uint64(18446744073709516354)),
            avm.data_stack.pop()
        );
        Ok(())
    }

    #[test]
    fn test_mulw() -> Result<(), AvmError> {
        let program = [
            vec![0x0a], // #pragma version 10
            vec![
                0x81, 0xb4, 0xce, 0xb6, 0xf4, 0x81, 0xe3, 0xca, 0xff, 0xff, 0x01,
            ], // pushint 18446509981324650292
            vec![
                0x81, 0xa4, 0xca, 0xc4, 0x90, 0xee, 0xe2, 0xd8, 0xfd, 0xff, 0x01,
            ], // pushint 18445445648759203108
            vec![0x1d], // mulw
        ]
        .concat();
        let mut avm = Avm::for_program(&program)?;
        let avm = execute_program(&mut avm)?;

        // the result (in hex 0xfffa8e32d6bd7f742c811b4ac78a0750)
        // is pushed onto the stack as two 64-bit values
        assert_eq!(2, avm.data_stack.len());
        assert_eq!(
            // low
            Some(AvmData::Uint64(0x2c811b4ac78a0750)),
            avm.data_stack.pop()
        );
        assert_eq!(
            // high
            Some(AvmData::Uint64(0xfffa8e32d6bd7f74)),
            avm.data_stack.pop()
        );
        Ok(())
    }

    #[test]
    fn test_addw() -> Result<(), AvmError> {
        let program = [
            vec![0x0a], // #pragma version 10
            vec![
                0x81, 0xb4, 0xce, 0xb6, 0xf4, 0x81, 0xe3, 0xca, 0xff, 0xff, 0x01,
            ], // pushint 18446509981324650292
            vec![
                0x81, 0xa4, 0xca, 0xc4, 0x90, 0xee, 0xe2, 0xd8, 0xfd, 0xff, 0x01,
            ], // pushint 18445445648759203108
            vec![0x1e], // addw
        ]
        .concat();
        let mut avm = Avm::for_program(&program)?;
        let avm = execute_program(&mut avm)?;

        // the result (in hex 0x0000000000000001fffa8e2f009ecc58)
        // is pushed onto the stack as two 64-bit values
        assert_eq!(2, avm.data_stack.len());
        assert_eq!(
            // low
            Some(AvmData::Uint64(0xfffa8e2f009ecc58)),
            avm.data_stack.pop()
        );
        assert_eq!(
            // high
            Some(AvmData::Uint64(0x0000000000000001)),
            avm.data_stack.pop()
        );
        Ok(())
    }

    #[test]
    fn test_divmodw() -> Result<(), AvmError> {
        // ab = 0xfffa8e32d6bd7f742c811b4ac78a0750
        // cd = 0x00000000002ccd18797b150ba583e510
        //  a = 0xfffa8e32d6bd7f74 as varint: f4fef5b5adc6a3fdff01
        //  b = 0x2c811b4ac78a0750 as varint: d08ea8bcace9c6c02c
        //  c = 0x00000000002ccd18 as varint: 989ab301
        //  d = 0x797b150ba583e510 as varint: 90ca8facbaa1c5bd79
        let program = [
            vec![0x0a], // #pragma version 10
            vec![
                0x81, 0xf4, 0xfe, 0xf5, 0xb5, 0xad, 0xc6, 0xa3, 0xfd, 0xff, 0x01,
            ], // pushint a
            vec![0x81, 0xd0, 0x8e, 0xa8, 0xbc, 0xac, 0xe9, 0xc6, 0xc0, 0x2c], // pushint b
            vec![0x81, 0x98, 0x9a, 0xb3, 0x01], // pushint c
            vec![0x81, 0x90, 0xca, 0x8f, 0xac, 0xba, 0xa1, 0xc5, 0xbd, 0x79], // pushint d
            vec![0x1f], // divmodw
        ]
        .concat();
        let mut avm = Avm::for_program(&program)?;
        let avm = execute_program(&mut avm)?;

        // the result are two u128s (split into four u64s):
        // result of division: 0x0000000000000000000005b6b2aa6607 = 6282239698439
        // result of modulo:   0x000000000029b2807db11bcb770a63e0 = 50408910078442914141856736
        assert_eq!(4, avm.data_stack.len());
        assert_eq!(
            // modulo low
            Some(AvmData::Uint64(0x7db11bcb770a63e0)),
            avm.data_stack.pop()
        );
        assert_eq!(
            // modulo high
            Some(AvmData::Uint64(0x000000000029b280)),
            avm.data_stack.pop()
        );
        assert_eq!(
            // division low
            Some(AvmData::Uint64(0x000005b6b2aa6607)),
            avm.data_stack.pop()
        );
        assert_eq!(
            // division high
            Some(AvmData::Uint64(0x0000000000000000)),
            avm.data_stack.pop()
        );
        Ok(())
    }

    #[test]
    fn test_store() -> Result<(), AvmError> {
        let program = [
            vec![0x0a],       // #pragma version 10
            vec![0x81, 0x05], // pushint 5
            vec![0x35, 0x02], // store 2
        ]
        .concat();
        let mut avm = Avm::for_program(&program)?;
        let avm = execute_program(&mut avm)?;

        assert_eq!(0, avm.data_stack.len());
        assert_eq!(AvmData::Uint64(5), avm.scratch[2]);
        Ok(())
    }

    #[test]
    fn test_load_unset_location() -> Result<(), AvmError> {
        // since position 2 wasn't written before, it is the
        // default value: 0 as uint64
        let program = [
            vec![0x0a],       // #pragma version 10
            vec![0x34, 0x02], // load 2
        ]
        .concat();
        let mut avm = Avm::for_program(&program)?;
        let avm = execute_program(&mut avm)?;

        assert_eq!(1, avm.data_stack.len());
        assert_eq!(Some(AvmData::Uint64(0)), avm.data_stack.pop());
        Ok(())
    }

    #[test]
    fn test_store_and_load() -> Result<(), AvmError> {
        // since position 2 wasn't written before, it is the
        // default value: 0 as uint64
        let program = [
            vec![0x0a],                               // #pragma version 10
            vec![0x81, 0x01],                         // pushint 1
            vec![0x80, 0x04, 0xde, 0xad, 0xbe, 0xef], // pushbytes 0xdeadbeef
            vec![0x35, 0x05],                         // store 5
            vec![0x35, 0x03],                         // store 3
            vec![0x34, 0x05],                         // load 5
            vec![0x34, 0x03],                         // load 3
            vec![0x34, 0x05],                         // load 5
        ]
        .concat();
        let mut avm = Avm::for_program(&program)?;
        let avm = execute_program(&mut avm)?;

        assert_eq!(3, avm.data_stack.len());
        assert_eq!(
            Some(AvmData::Bytes(vec![0xde, 0xad, 0xbe, 0xef])),
            avm.data_stack.pop()
        );
        assert_eq!(Some(AvmData::Uint64(1)), avm.data_stack.pop());
        assert_eq!(
            Some(AvmData::Bytes(vec![0xde, 0xad, 0xbe, 0xef])),
            avm.data_stack.pop()
        );
        Ok(())
    }

    #[test]
    fn test_loads() -> Result<(), AvmError> {
        let program = [
            vec![0x0a],                               // #pragma version 10
            vec![0x80, 0x04, 0xde, 0xad, 0xbe, 0xef], // pushbytes 0xdeadbeef
            vec![0x35, 0x09],                         // store 9
            vec![0x81, 0x09],                         // pushint 9
            vec![0x3e],                               // loads
        ]
        .concat();
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
    fn test_loads_out_of_bounds() -> Result<(), AvmError> {
        // index 256 is the first position that is no longer
        // within the scratch space's bounds
        let program = [
            vec![0x0a],             // #pragma version 10
            vec![0x81, 0x80, 0x02], // pushint 256
            vec![0x3e],             // loads
        ]
        .concat();
        let mut avm = Avm::for_program(&program)?;
        let err = execute_program(&mut avm).unwrap_err();
        assert_eq!(AvmError::ScratchAccessOutOfBounds(256), err);
        Ok(())
    }

    #[test]
    fn test_stores() -> Result<(), AvmError> {
        let program = [
            vec![0x0a],                               // #pragma version 10
            vec![0x81, 0x05],                         // pushint 5
            vec![0x80, 0x04, 0xde, 0xad, 0xbe, 0xef], // pushbytes 0xdeadbeef
            vec![0x3f],                               // stores
        ]
        .concat();
        let mut avm = Avm::for_program(&program)?;
        let avm = execute_program(&mut avm)?;

        assert_eq!(0, avm.data_stack.len());
        assert_eq!(AvmData::Bytes(vec![0xde, 0xad, 0xbe, 0xef]), avm.scratch[5]);
        Ok(())
    }

    #[test]
    fn test_stores_out_of_bounds() -> Result<(), AvmError> {
        // index 256 is the first position that is no longer
        // within the scratch space's bounds
        let program = [
            vec![0x0a],                               // #pragma version 10
            vec![0x81, 0x80, 0x02],                   // pushint 256
            vec![0x80, 0x04, 0xde, 0xad, 0xbe, 0xef], // pushbytes 0xdeadbeef
            vec![0x3f],                               // stores
        ]
        .concat();
        let mut avm = Avm::for_program(&program)?;
        let err = execute_program(&mut avm).unwrap_err();
        assert_eq!(AvmError::ScratchAccessOutOfBounds(256), err);
        Ok(())
    }

    #[test]
    fn test_bz() -> Result<(), AvmError> {
        let program = [
            vec![0x0a],             // #pragma version 10
            vec![0x81, 0x03],       // pushint 3
            vec![0x81, 0x02],       // pushint 2
            vec![0x0c],             // <
            vec![0x41, 0x00, 0x06], // bz 0x0006
            vec![0x81, 0x01],       // pushint 1
            vec![0x43],             // return
            vec![0x42, 0x00, 0x03], // b 0x0003
            vec![0x81, 0x00],       // pushint 0
            vec![0x43],             // return
        ]
        .concat();
        let mut avm = Avm::for_program(&program)?;
        let avm = execute_program(&mut avm)?;

        assert_eq!(1, avm.data_stack.len());
        assert_eq!(Some(AvmData::Uint64(0)), avm.data_stack.pop());
        Ok(())
    }

    #[test]
    fn test_bnz() -> Result<(), AvmError> {
        let program = [
            vec![0x0a],             // #pragma version 10
            vec![0x81, 0x02],       // pushint 2
            vec![0x81, 0x03],       // pushint 3
            vec![0x0c],             // <
            vec![0x40, 0x00, 0x06], // bnz 0x0006
            vec![0x81, 0x00],       // pushint 0
            vec![0x43],             // return
            vec![0x42, 0x00, 0x03], // b 0x0003
            vec![0x81, 0x01],       // pushint 1
            vec![0x43],             // return
        ]
        .concat();
        let mut avm = Avm::for_program(&program)?;
        let avm = execute_program(&mut avm)?;

        assert_eq!(1, avm.data_stack.len());
        assert_eq!(Some(AvmData::Uint64(1)), avm.data_stack.pop());
        Ok(())
    }

    #[test]
    fn test_b() -> Result<(), AvmError> {
        let program = [
            vec![0x0a],             // #pragma version 10
            vec![0x42, 0x00, 0x04], // bnz 0x0004
            vec![0x81, 0x01],       // pushint 1
            vec![0x81, 0x02],       // pushint 2
            vec![0x81, 0x01],       // pushint 1
            vec![0x43],             // return
        ]
        .concat();
        let mut avm = Avm::for_program(&program)?;
        let avm = execute_program(&mut avm)?;

        assert_eq!(1, avm.data_stack.len());
        assert_eq!(Some(AvmData::Uint64(1)), avm.data_stack.pop());
        Ok(())
    }

    #[test]
    fn test_return() -> Result<(), AvmError> {
        // #pragma version 10
        // pushint 1
        // pushint 2
        // pushint 3
        // return
        let program = vec![0x0a, 0x81, 0x01, 0x81, 0x02, 0x81, 0x03, 0x43];
        let mut avm = Avm::for_program(&program)?;
        let avm = execute_program(&mut avm)?;

        assert_eq!(1, avm.data_stack.len());
        assert_eq!(Some(AvmData::Uint64(3)), avm.data_stack.pop());
        Ok(())
    }

    #[test]
    fn test_assert() -> Result<(), AvmError> {
        let program = [
            vec![0x0a],       // #pragma version 10
            vec![0x81, 0x05], // pushint 5
            vec![0x44],       // assert
        ]
        .concat();
        let mut avm = Avm::for_program(&program)?;
        let avm = execute_program(&mut avm)?;

        assert_eq!(0, avm.data_stack.len());
        Ok(())
    }

    #[test]
    fn test_assert_negative() -> Result<(), AvmError> {
        let program = [
            vec![0x0a],       // #pragma version 10
            vec![0x81, 0x00], // pushint 0
            vec![0x44],       // assert
        ]
        .concat();
        let mut avm = Avm::for_program(&program)?;
        let err = execute_program(&mut avm).unwrap_err();
        assert_eq!(AvmError::AssertionFailed(3), err);
        Ok(())
    }

    #[test]
    fn test_bury() -> Result<(), AvmError> {
        let program = [
            vec![0x0a],       // #pragma version 10
            vec![0x81, 0x01], // pushint 1
            vec![0x81, 0x02], // pushint 2
            vec![0x81, 0x03], // pushint 3
            vec![0x81, 0x04], // pushint 4
            vec![0x81, 0x05], // pushint 5
            vec![0x45, 0x04], // bury 4
        ]
        .concat();
        let mut avm = Avm::for_program(&program)?;
        let avm = execute_program(&mut avm)?;

        assert_eq!(4, avm.data_stack.len());
        assert_eq!(Some(AvmData::Uint64(4)), avm.data_stack.pop());
        assert_eq!(Some(AvmData::Uint64(3)), avm.data_stack.pop());
        assert_eq!(Some(AvmData::Uint64(5)), avm.data_stack.pop());
        assert_eq!(Some(AvmData::Uint64(1)), avm.data_stack.pop());
        Ok(())
    }

    #[test]
    fn test_bury_zero() -> Result<(), AvmError> {
        let program = [
            vec![0x0a],       // #pragma version 10
            vec![0x81, 0x01], // pushint 1
            vec![0x81, 0x02], // pushint 2
            vec![0x45, 0x00], // bury 0
        ]
        .concat();
        let mut avm = Avm::for_program(&program)?;
        let err = execute_program(&mut avm).unwrap_err();
        assert_eq!(AvmError::InvalidStackAccess, err);
        Ok(())
    }

    #[test]
    fn test_bury_outside_range() -> Result<(), AvmError> {
        let program = [
            vec![0x0a],       // #pragma version 10
            vec![0x81, 0x01], // pushint 1
            vec![0x81, 0x02], // pushint 2
            vec![0x45, 0x03], // bury 3
        ]
        .concat();
        let mut avm = Avm::for_program(&program)?;
        let err = execute_program(&mut avm).unwrap_err();
        assert_eq!(AvmError::InvalidStackAccess, err);
        Ok(())
    }

    #[test]
    fn test_popn() -> Result<(), AvmError> {
        let program = [
            vec![0x0a],       // #pragma version 10
            vec![0x81, 0x01], // pushint 1
            vec![0x81, 0x02], // pushint 2
            vec![0x81, 0x03], // pushint 3
            vec![0x81, 0x04], // pushint 4
            vec![0x81, 0x05], // pushint 5
            vec![0x46, 0x03], // popn 3
        ]
        .concat();
        let mut avm = Avm::for_program(&program)?;
        let avm = execute_program(&mut avm)?;

        assert_eq!(2, avm.data_stack.len());
        assert_eq!(Some(AvmData::Uint64(2)), avm.data_stack.pop());
        assert_eq!(Some(AvmData::Uint64(1)), avm.data_stack.pop());
        Ok(())
    }

    #[test]
    fn test_popn_underflow() -> Result<(), AvmError> {
        let program = [
            vec![0x0a],       // #pragma version 10
            vec![0x81, 0x01], // pushint 1
            vec![0x81, 0x02], // pushint 2
            vec![0x46, 0x03], // popn 3
        ]
        .concat();
        let mut avm = Avm::for_program(&program)?;
        let err = execute_program(&mut avm).unwrap_err();
        assert_eq!(AvmError::StackUnderflow, err);
        Ok(())
    }

    #[test]
    fn test_dupn() -> Result<(), AvmError> {
        let program = [
            vec![0x0a],       // #pragma version 10
            vec![0x81, 0x01], // pushint 1
            vec![0x47, 0x03], // dupn 3
        ]
        .concat();
        let mut avm = Avm::for_program(&program)?;
        let avm = execute_program(&mut avm)?;

        assert_eq!(4, avm.data_stack.len());
        assert_eq!(Some(AvmData::Uint64(1)), avm.data_stack.pop());
        assert_eq!(Some(AvmData::Uint64(1)), avm.data_stack.pop());
        assert_eq!(Some(AvmData::Uint64(1)), avm.data_stack.pop());
        assert_eq!(Some(AvmData::Uint64(1)), avm.data_stack.pop());
        Ok(())
    }

    #[test]
    fn test_dup() -> Result<(), AvmError> {
        // #pragma version 10
        // pushint 1
        // dup
        let program = vec![0x0a, 0x81, 0x01, 0x49];
        let mut avm = Avm::for_program(&program)?;
        let avm = execute_program(&mut avm)?;

        assert_eq!(2, avm.data_stack.len());
        assert_eq!(Some(AvmData::Uint64(1)), avm.data_stack.pop());
        assert_eq!(Some(AvmData::Uint64(1)), avm.data_stack.pop());
        Ok(())
    }

    #[test]
    fn test_dup_on_empty_stack() -> Result<(), AvmError> {
        // #pragma version 10
        // dup
        let program = vec![0x0a, 0x49];
        let mut avm = Avm::for_program(&program)?;
        let err = execute_program(&mut avm).unwrap_err();
        assert_eq!(AvmError::StackUnderflow, err);
        Ok(())
    }

    #[test]
    fn test_dup2() -> Result<(), AvmError> {
        // #pragma version 10
        // pushint 1
        // pushbytes 0xde
        // dup2
        let program = vec![0x0a, 0x81, 0x01, 0x80, 0x01, 0xde, 0x4a];
        let mut avm = Avm::for_program(&program)?;
        let avm = execute_program(&mut avm)?;

        assert_eq!(4, avm.data_stack.len());
        assert_eq!(Some(AvmData::Bytes(vec![0xde])), avm.data_stack.pop());
        assert_eq!(Some(AvmData::Uint64(1)), avm.data_stack.pop());
        assert_eq!(Some(AvmData::Bytes(vec![0xde])), avm.data_stack.pop());
        assert_eq!(Some(AvmData::Uint64(1)), avm.data_stack.pop());
        Ok(())
    }

    #[test]
    fn test_dup2_on_too_short_stack() -> Result<(), AvmError> {
        // #pragma version 10
        // pushbytes 0xde
        // dup2
        let program = vec![0x0a, 0x80, 0x01, 0xde, 0x4a];
        let mut avm = Avm::for_program(&program)?;
        let err = execute_program(&mut avm).unwrap_err();
        assert_eq!(AvmError::StackUnderflow, err);
        Ok(())
    }

    #[test]
    fn test_dig() -> Result<(), AvmError> {
        let program = [
            vec![0x0a],       // #pragma version 10
            vec![0x81, 0x01], // pushint 1
            vec![0x81, 0x02], // pushint 2
            vec![0x81, 0x03], // pushint 3
            vec![0x81, 0x04], // pushint 4
            vec![0x81, 0x05], // pushint 5
            vec![0x4b, 0x03], // dig 3
        ]
        .concat();
        let mut avm = Avm::for_program(&program)?;
        let avm = execute_program(&mut avm)?;

        assert_eq!(6, avm.data_stack.len());
        assert_eq!(Some(AvmData::Uint64(2)), avm.data_stack.pop());
        assert_eq!(Some(AvmData::Uint64(5)), avm.data_stack.pop());
        assert_eq!(Some(AvmData::Uint64(4)), avm.data_stack.pop());
        assert_eq!(Some(AvmData::Uint64(3)), avm.data_stack.pop());
        assert_eq!(Some(AvmData::Uint64(2)), avm.data_stack.pop());
        assert_eq!(Some(AvmData::Uint64(1)), avm.data_stack.pop());
        Ok(())
    }

    #[test]
    fn test_dig_zero() -> Result<(), AvmError> {
        let program = [
            vec![0x0a],       // #pragma version 10
            vec![0x81, 0x01], // pushint 1
            vec![0x81, 0x02], // pushint 2
            vec![0x4b, 0x00], // dig 0
        ]
        .concat();
        let mut avm = Avm::for_program(&program)?;
        let avm = execute_program(&mut avm)?;

        assert_eq!(3, avm.data_stack.len());
        assert_eq!(Some(AvmData::Uint64(2)), avm.data_stack.pop());
        assert_eq!(Some(AvmData::Uint64(2)), avm.data_stack.pop());
        assert_eq!(Some(AvmData::Uint64(1)), avm.data_stack.pop());
        Ok(())
    }

    #[test]
    fn test_dig_outside_range() -> Result<(), AvmError> {
        let program = [
            vec![0x0a],       // #pragma version 10
            vec![0x81, 0x01], // pushint 1
            vec![0x81, 0x02], // pushint 2
            vec![0x4b, 0x02], // dig 2
        ]
        .concat();
        let mut avm = Avm::for_program(&program)?;
        let err = execute_program(&mut avm).unwrap_err();
        assert_eq!(AvmError::InvalidStackAccess, err);
        Ok(())
    }

    #[test]
    fn test_swap() -> Result<(), AvmError> {
        let program = [
            vec![0x0a],       // #pragma version 10
            vec![0x81, 0x01], // pushint 1
            vec![0x81, 0x02], // pushint 2
            vec![0x4c],       // swap
        ]
        .concat();
        let mut avm = Avm::for_program(&program)?;
        let avm = execute_program(&mut avm)?;

        assert_eq!(2, avm.data_stack.len());
        assert_eq!(Some(AvmData::Uint64(1)), avm.data_stack.pop());
        assert_eq!(Some(AvmData::Uint64(2)), avm.data_stack.pop());
        Ok(())
    }

    #[test]
    fn test_swap_with_too_few_elements() -> Result<(), AvmError> {
        let program = [
            vec![0x0a],       // #pragma version 10
            vec![0x81, 0x01], // pushint 1
            vec![0x4c],       // swap
        ]
        .concat();
        let mut avm = Avm::for_program(&program)?;
        let err = execute_program(&mut avm).unwrap_err();
        assert_eq!(AvmError::StackUnderflow, err);
        Ok(())
    }

    #[test]
    fn test_select_choose_first() -> Result<(), AvmError> {
        let program = [
            vec![0x0a],       // #pragma version 10
            vec![0x81, 0x01], // pushint 1
            vec![0x81, 0x02], // pushint 2
            vec![0x81, 0x00], // pushint 0
            vec![0x4d],       // select
        ]
        .concat();
        let mut avm = Avm::for_program(&program)?;
        let avm = execute_program(&mut avm)?;

        assert_eq!(1, avm.data_stack.len());
        assert_eq!(Some(AvmData::Uint64(1)), avm.data_stack.pop());
        Ok(())
    }

    #[test]
    fn test_select_choose_second() -> Result<(), AvmError> {
        let program = [
            vec![0x0a],       // #pragma version 10
            vec![0x81, 0x01], // pushint 1
            vec![0x81, 0x02], // pushint 2
            vec![0x81, 0x01], // pushint 1
            vec![0x4d],       // select
        ]
        .concat();
        let mut avm = Avm::for_program(&program)?;
        let avm = execute_program(&mut avm)?;

        assert_eq!(1, avm.data_stack.len());
        assert_eq!(Some(AvmData::Uint64(2)), avm.data_stack.pop());
        Ok(())
    }

    #[test]
    fn test_cover_at_bottom() -> Result<(), AvmError> {
        let program = [
            vec![0x0a],       // #pragma version 10
            vec![0x81, 0x01], // pushint 1
            vec![0x81, 0x02], // pushint 2
            vec![0x81, 0x03], // pushint 3
            vec![0x81, 0x04], // pushint 4
            vec![0x4e, 0x03], // cover 3
        ]
        .concat();
        let mut avm = Avm::for_program(&program)?;
        let avm = execute_program(&mut avm)?;

        assert_eq!(4, avm.data_stack.len());
        assert_eq!(Some(AvmData::Uint64(3)), avm.data_stack.pop());
        assert_eq!(Some(AvmData::Uint64(2)), avm.data_stack.pop());
        assert_eq!(Some(AvmData::Uint64(1)), avm.data_stack.pop());
        assert_eq!(Some(AvmData::Uint64(4)), avm.data_stack.pop());
        Ok(())
    }

    #[test]
    fn test_cover_at_top() -> Result<(), AvmError> {
        let program = [
            vec![0x0a],       // #pragma version 10
            vec![0x81, 0x01], // pushint 1
            vec![0x81, 0x02], // pushint 2
            vec![0x81, 0x03], // pushint 3
            vec![0x81, 0x04], // pushint 4
            vec![0x4e, 0x00], // cover 0
        ]
        .concat();
        let mut avm = Avm::for_program(&program)?;
        let avm = execute_program(&mut avm)?;

        assert_eq!(4, avm.data_stack.len());
        assert_eq!(Some(AvmData::Uint64(4)), avm.data_stack.pop());
        assert_eq!(Some(AvmData::Uint64(3)), avm.data_stack.pop());
        assert_eq!(Some(AvmData::Uint64(2)), avm.data_stack.pop());
        assert_eq!(Some(AvmData::Uint64(1)), avm.data_stack.pop());
        Ok(())
    }

    #[test]
    fn test_cover_outside_range() -> Result<(), AvmError> {
        let program = [
            vec![0x0a],       // #pragma version 10
            vec![0x81, 0x01], // pushint 1
            vec![0x81, 0x02], // pushint 2
            vec![0x4e, 0x02], // cover 2
        ]
        .concat();
        let mut avm = Avm::for_program(&program)?;
        let err = execute_program(&mut avm).unwrap_err();
        assert_eq!(AvmError::InvalidStackAccess, err);
        Ok(())
    }

    #[test]
    fn test_uncover_at_bottom() -> Result<(), AvmError> {
        let program = [
            vec![0x0a],       // #pragma version 10
            vec![0x81, 0x01], // pushint 1
            vec![0x81, 0x02], // pushint 2
            vec![0x81, 0x03], // pushint 3
            vec![0x81, 0x04], // pushint 4
            vec![0x4f, 0x03], // uncover 3
        ]
        .concat();
        let mut avm = Avm::for_program(&program)?;
        let avm = execute_program(&mut avm)?;

        assert_eq!(4, avm.data_stack.len());
        assert_eq!(Some(AvmData::Uint64(1)), avm.data_stack.pop());
        assert_eq!(Some(AvmData::Uint64(4)), avm.data_stack.pop());
        assert_eq!(Some(AvmData::Uint64(3)), avm.data_stack.pop());
        assert_eq!(Some(AvmData::Uint64(2)), avm.data_stack.pop());
        Ok(())
    }

    #[test]
    fn test_uncover_at_top() -> Result<(), AvmError> {
        let program = [
            vec![0x0a],       // #pragma version 10
            vec![0x81, 0x03], // pushint 3
            vec![0x81, 0x04], // pushint 4
            vec![0x4f, 0x00], // uncover 0
        ]
        .concat();
        let mut avm = Avm::for_program(&program)?;
        let avm = execute_program(&mut avm)?;

        assert_eq!(2, avm.data_stack.len());
        assert_eq!(Some(AvmData::Uint64(4)), avm.data_stack.pop());
        assert_eq!(Some(AvmData::Uint64(3)), avm.data_stack.pop());
        Ok(())
    }

    #[test]
    fn test_uncover_outside_range() -> Result<(), AvmError> {
        let program = [
            vec![0x0a],       // #pragma version 10
            vec![0x81, 0x01], // pushint 1
            vec![0x81, 0x02], // pushint 2
            vec![0x4f, 0x02], // uncover 2
        ]
        .concat();
        let mut avm = Avm::for_program(&program)?;
        let err = execute_program(&mut avm).unwrap_err();
        assert_eq!(AvmError::InvalidStackAccess, err);
        Ok(())
    }

    #[test]
    fn test_substring() -> Result<(), AvmError> {
        let program = [
            vec![0x0a],                               // #pragma version 10
            vec![0x80, 0x04, 0xde, 0xad, 0xbe, 0xef], // pushbytes 0xdeadbeef
            vec![0x51, 0x01, 0x04],                   // substring 1 4
        ]
        .concat();
        let mut avm = Avm::for_program(&program)?;
        let avm = execute_program(&mut avm)?;

        assert_eq!(1, avm.data_stack.len());
        assert_eq!(
            Some(AvmData::Bytes(vec![0xad, 0xbe, 0xef])),
            avm.data_stack.pop()
        );
        Ok(())
    }

    #[test]
    fn test_substring_empty() -> Result<(), AvmError> {
        let program = [
            vec![0x0a],                               // #pragma version 10
            vec![0x80, 0x04, 0xde, 0xad, 0xbe, 0xef], // pushbytes 0xdeadbeef
            vec![0x51, 0x00, 0x00],                   // substring 0 0
        ]
        .concat();
        let mut avm = Avm::for_program(&program)?;
        let avm = execute_program(&mut avm)?;

        assert_eq!(1, avm.data_stack.len());
        assert_eq!(Some(AvmData::Bytes(vec![])), avm.data_stack.pop());
        Ok(())
    }

    #[test]
    fn test_substring_outside_range() -> Result<(), AvmError> {
        let program = [
            vec![0x0a],                               // #pragma version 10
            vec![0x80, 0x04, 0xde, 0xad, 0xbe, 0xef], // pushbytes 0xdeadbeef
            vec![0x51, 0x01, 0x05],                   // substring 1 5
        ]
        .concat();
        let mut avm = Avm::for_program(&program)?;
        let err = execute_program(&mut avm).unwrap_err();
        assert_eq!(AvmError::InvalidSubstringAccess(1, 5, 4), err);
        Ok(())
    }

    #[test]
    fn test_substring_end_before_start() -> Result<(), AvmError> {
        let program = [
            vec![0x0a],                               // #pragma version 10
            vec![0x80, 0x04, 0xde, 0xad, 0xbe, 0xef], // pushbytes 0xdeadbeef
            vec![0x51, 0x03, 0x02],                   // substring 3 2
        ]
        .concat();
        let mut avm = Avm::for_program(&program)?;
        let err = execute_program(&mut avm).unwrap_err();
        assert_eq!(AvmError::InvalidSubstringAccess(3, 2, 4), err);
        Ok(())
    }

    #[test]
    fn test_substring3() -> Result<(), AvmError> {
        let program = [
            vec![0x0a],                               // #pragma version 10
            vec![0x80, 0x04, 0xde, 0xad, 0xbe, 0xef], // pushbytes 0xdeadbeef
            vec![0x81, 0x01],                         // pushint 1
            vec![0x81, 0x02],                         // pushint 2
            vec![0x52],                               // substring3
        ]
        .concat();
        let mut avm = Avm::for_program(&program)?;
        let avm = execute_program(&mut avm)?;

        assert_eq!(1, avm.data_stack.len());
        assert_eq!(Some(AvmData::Bytes(vec![0xad])), avm.data_stack.pop());
        Ok(())
    }

    #[test]
    fn test_loop_with_bnz() -> Result<(), AvmError> {
        let program = [
            vec![0x0a],             // #pragma version 10
            vec![0x81, 0x00],       // pushint 0
            vec![0x81, 0x01],       // pushint 1
            vec![0x08],             // +
            vec![0x49],             // dup
            vec![0x81, 0x05],       // pushint 5
            vec![0x0c],             // <
            vec![0x40, 0xff, 0xf6], // bnz 0xfff6 (-10 in two's complement)
        ]
        .concat();
        let mut avm = Avm::for_program(&program)?;
        let avm = execute_program(&mut avm)?;

        assert_eq!(1, avm.data_stack.len());
        assert_eq!(Some(AvmData::Uint64(5)), avm.data_stack.pop());
        Ok(())
    }
}
