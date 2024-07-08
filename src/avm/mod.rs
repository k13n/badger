use crate::{
    encoding::{VarBytes, VarUint64},
    AvmError,
};
use opcodes::OP_SPECS;
use std::vec::Vec;

mod opcodes;

const FALSE: AvmData = AvmData::Uint64(0);
const TRUE: AvmData = AvmData::Uint64(1);

const LABEL_UINT64: &'static str = "uint64";
const LABEL_BYTES: &'static str = "bytes";

#[derive(Debug)]
pub struct Avm<'a> {
    pub data_stack: Vec<AvmData>,
    pub program: &'a [u8],
    pub pc: usize,
    pub version: AvmVersion,
    pub intc: Vec<u64>,
    pub bytec: Vec<&'a [u8]>,
}

impl<'a> Avm<'a> {
    pub fn for_program(program: &'a [u8]) -> Result<Self, AvmError> {
        if program.is_empty() {
            return Err(AvmError::EmptyProgram);
        }

        let mut pc = 0;

        // the first byte indicates the AVM version of the program
        let version = AvmVersion::try_from(program[pc])?;
        pc += 1;

        Ok(Avm {
            data_stack: vec![],
            program,
            pc,
            version,
            intc: vec![],
            bytec: vec![],
        })
    }

    fn read_byte(&mut self) -> Result<u8, AvmError> {
        if self.pc < self.program.len() {
            let byte = self.program[self.pc];
            self.pc += 1;
            Ok(byte)
        } else {
            Err(AvmError::PcOutOfBounds)
        }
    }

    fn read_varint(&mut self) -> Result<VarUint64, AvmError> {
        let number: VarUint64 = self.program[self.pc..].try_into()?;
        self.pc += number.nbytes;
        Ok(number)
    }

    fn read_varbytes(&mut self) -> Result<VarBytes<'a>, AvmError> {
        let bytes: VarBytes = self.program[self.pc..].try_into()?;
        self.pc += bytes.nbytes;
        Ok(bytes)
    }

    fn pop_uint64(&mut self) -> Result<u64, AvmError> {
        match self.data_stack.pop() {
            Some(AvmData::Uint64(v)) => Ok(v),
            Some(AvmData::Bytes(_)) => Err(AvmError::IncompatibleTypes(LABEL_BYTES, LABEL_UINT64)),
            None => Err(AvmError::StackUnderflow),
        }
    }

    fn pop_bytes(&mut self) -> Result<Vec<u8>, AvmError> {
        match self.data_stack.pop() {
            Some(AvmData::Bytes(v)) => Ok(v),
            Some(AvmData::Uint64(_)) => Err(AvmError::IncompatibleTypes(LABEL_UINT64, LABEL_BYTES)),
            None => Err(AvmError::StackUnderflow),
        }
    }
}

pub fn execute_program<'a>(avm: &'a mut Avm<'a>) -> Result<&'a mut Avm<'a>, AvmError> {
    let mut cost = 0;
    while cost < 700 && avm.pc < avm.program.len() {
        let opcode = avm.read_byte()?;
        let spec_idx = OP_SPECS
            .iter()
            .position(|spec| spec.opcode == opcode && avm.version >= spec.version);
        let opspec = match spec_idx {
            Some(idx) => &OP_SPECS[idx],
            None => return Err(AvmError::UnknownOpcode(opcode)),
        };

        (opspec.eval)(avm)?;
        cost += opspec.cost;
    }

    Ok(avm)
}

#[derive(Debug, PartialEq, PartialOrd)]
pub enum AvmVersion {
    V1,
    V2,
    V3,
    V4,
    V5,
    V6,
    V7,
    V8,
    V9,
    V10,
}

impl TryFrom<u8> for AvmVersion {
    type Error = AvmError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(AvmVersion::V1),
            2 => Ok(AvmVersion::V2),
            3 => Ok(AvmVersion::V3),
            4 => Ok(AvmVersion::V4),
            5 => Ok(AvmVersion::V5),
            6 => Ok(AvmVersion::V6),
            7 => Ok(AvmVersion::V7),
            8 => Ok(AvmVersion::V8),
            9 => Ok(AvmVersion::V9),
            10 => Ok(AvmVersion::V10),
            _ => Err(AvmError::InvalidAvmVerison(value)),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum AvmData {
    Bytes(Vec<u8>),
    Uint64(u64),
}

impl From<bool> for AvmData {
    fn from(value: bool) -> Self {
        if value {
            TRUE
        } else {
            FALSE
        }
    }
}

impl From<VarUint64> for AvmData {
    fn from(value: VarUint64) -> Self {
        AvmData::Uint64(value.value)
    }
}

impl From<u64> for AvmData {
    fn from(value: u64) -> Self {
        AvmData::Uint64(value)
    }
}

impl<'a> From<VarBytes<'a>> for AvmData {
    fn from(value: VarBytes<'a>) -> Self {
        AvmData::Bytes(value.value.to_vec())
    }
}

impl From<Vec<u8>> for AvmData {
    fn from(value: Vec<u8>) -> Self {
        AvmData::Bytes(value)
    }
}
