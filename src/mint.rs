// Generated by Molecule 0.7.2

#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(clippy::if_same_then_else)]

use super::ckb_types::prelude::*;
use super::molecule::{self, prelude::*};
extern crate alloc;
pub use alloc::vec::*;
// these lines above are manually added
// replace "::molecule" to "molecule" in below code

use super::common::*;
use molecule::prelude::*;
#[derive(Clone)]
pub struct DefineCotaNFTEntries(molecule::bytes::Bytes);
impl ::core::fmt::LowerHex for DefineCotaNFTEntries {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        use molecule::hex_string;
        if f.alternate() {
            write!(f, "0x")?;
        }
        write!(f, "{}", hex_string(self.as_slice()))
    }
}
impl ::core::fmt::Debug for DefineCotaNFTEntries {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        write!(f, "{}({:#x})", Self::NAME, self)
    }
}
impl ::core::fmt::Display for DefineCotaNFTEntries {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        write!(f, "{} {{ ", Self::NAME)?;
        write!(f, "{}: {}", "define_key", self.define_key())?;
        write!(f, ", {}: {}", "define_old_value", self.define_old_value())?;
        write!(f, ", {}: {}", "define_new_value", self.define_new_value())?;
        write!(f, ", {}: {}", "withdrawal_keys", self.withdrawal_keys())?;
        write!(f, ", {}: {}", "withdrawal_values", self.withdrawal_values())?;
        write!(f, ", {}: {}", "proof", self.proof())?;
        write!(f, ", {}: {}", "action", self.action())?;
        let extra_count = self.count_extra_fields();
        if extra_count != 0 {
            write!(f, ", .. ({} fields)", extra_count)?;
        }
        write!(f, " }}")
    }
}
impl ::core::default::Default for DefineCotaNFTEntries {
    fn default() -> Self {
        let v: Vec<u8> = vec![
            120, 0, 0, 0, 32, 0, 0, 0, 54, 0, 0, 0, 79, 0, 0, 0, 104, 0, 0, 0, 108, 0, 0, 0, 112,
            0, 0, 0, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 25, 0, 0, 0, 16, 0, 0, 0, 20, 0, 0, 0, 24, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 25,
            0, 0, 0, 16, 0, 0, 0, 20, 0, 0, 0, 24, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ];
        DefineCotaNFTEntries::new_unchecked(v.into())
    }
}
impl DefineCotaNFTEntries {
    pub const FIELD_COUNT: usize = 7;

    pub fn total_size(&self) -> usize {
        molecule::unpack_number(self.as_slice()) as usize
    }

    pub fn field_count(&self) -> usize {
        if self.total_size() == molecule::NUMBER_SIZE {
            0
        } else {
            (molecule::unpack_number(&self.as_slice()[molecule::NUMBER_SIZE..]) as usize / 4) - 1
        }
    }

    pub fn count_extra_fields(&self) -> usize {
        self.field_count() - Self::FIELD_COUNT
    }

    pub fn has_extra_fields(&self) -> bool {
        Self::FIELD_COUNT != self.field_count()
    }

    pub fn define_key(&self) -> DefineCotaNFTKey {
        let slice = self.as_slice();
        let start = molecule::unpack_number(&slice[4..]) as usize;
        let end = molecule::unpack_number(&slice[8..]) as usize;
        DefineCotaNFTKey::new_unchecked(self.0.slice(start..end))
    }

    pub fn define_old_value(&self) -> DefineCotaNFTValue {
        let slice = self.as_slice();
        let start = molecule::unpack_number(&slice[8..]) as usize;
        let end = molecule::unpack_number(&slice[12..]) as usize;
        DefineCotaNFTValue::new_unchecked(self.0.slice(start..end))
    }

    pub fn define_new_value(&self) -> DefineCotaNFTValue {
        let slice = self.as_slice();
        let start = molecule::unpack_number(&slice[12..]) as usize;
        let end = molecule::unpack_number(&slice[16..]) as usize;
        DefineCotaNFTValue::new_unchecked(self.0.slice(start..end))
    }

    pub fn withdrawal_keys(&self) -> WithdrawalCotaNFTKeyVec {
        let slice = self.as_slice();
        let start = molecule::unpack_number(&slice[16..]) as usize;
        let end = molecule::unpack_number(&slice[20..]) as usize;
        WithdrawalCotaNFTKeyVec::new_unchecked(self.0.slice(start..end))
    }

    pub fn withdrawal_values(&self) -> WithdrawalCotaNFTValueVec {
        let slice = self.as_slice();
        let start = molecule::unpack_number(&slice[20..]) as usize;
        let end = molecule::unpack_number(&slice[24..]) as usize;
        WithdrawalCotaNFTValueVec::new_unchecked(self.0.slice(start..end))
    }

    pub fn proof(&self) -> Bytes {
        let slice = self.as_slice();
        let start = molecule::unpack_number(&slice[24..]) as usize;
        let end = molecule::unpack_number(&slice[28..]) as usize;
        Bytes::new_unchecked(self.0.slice(start..end))
    }

    pub fn action(&self) -> Bytes {
        let slice = self.as_slice();
        let start = molecule::unpack_number(&slice[28..]) as usize;
        if self.has_extra_fields() {
            let end = molecule::unpack_number(&slice[32..]) as usize;
            Bytes::new_unchecked(self.0.slice(start..end))
        } else {
            Bytes::new_unchecked(self.0.slice(start..))
        }
    }

    pub fn as_reader<'r>(&'r self) -> DefineCotaNFTEntriesReader<'r> {
        DefineCotaNFTEntriesReader::new_unchecked(self.as_slice())
    }
}
impl molecule::prelude::Entity for DefineCotaNFTEntries {
    type Builder = DefineCotaNFTEntriesBuilder;

    const NAME: &'static str = "DefineCotaNFTEntries";

    fn new_unchecked(data: molecule::bytes::Bytes) -> Self {
        DefineCotaNFTEntries(data)
    }

    fn as_bytes(&self) -> molecule::bytes::Bytes {
        self.0.clone()
    }

    fn as_slice(&self) -> &[u8] {
        &self.0[..]
    }

    fn from_slice(slice: &[u8]) -> molecule::error::VerificationResult<Self> {
        DefineCotaNFTEntriesReader::from_slice(slice).map(|reader| reader.to_entity())
    }

    fn from_compatible_slice(slice: &[u8]) -> molecule::error::VerificationResult<Self> {
        DefineCotaNFTEntriesReader::from_compatible_slice(slice).map(|reader| reader.to_entity())
    }

    fn new_builder() -> Self::Builder {
        ::core::default::Default::default()
    }

    fn as_builder(self) -> Self::Builder {
        Self::new_builder()
            .define_key(self.define_key())
            .define_old_value(self.define_old_value())
            .define_new_value(self.define_new_value())
            .withdrawal_keys(self.withdrawal_keys())
            .withdrawal_values(self.withdrawal_values())
            .proof(self.proof())
            .action(self.action())
    }
}
#[derive(Clone, Copy)]
pub struct DefineCotaNFTEntriesReader<'r>(&'r [u8]);
impl<'r> ::core::fmt::LowerHex for DefineCotaNFTEntriesReader<'r> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        use molecule::hex_string;
        if f.alternate() {
            write!(f, "0x")?;
        }
        write!(f, "{}", hex_string(self.as_slice()))
    }
}
impl<'r> ::core::fmt::Debug for DefineCotaNFTEntriesReader<'r> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        write!(f, "{}({:#x})", Self::NAME, self)
    }
}
impl<'r> ::core::fmt::Display for DefineCotaNFTEntriesReader<'r> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        write!(f, "{} {{ ", Self::NAME)?;
        write!(f, "{}: {}", "define_key", self.define_key())?;
        write!(f, ", {}: {}", "define_old_value", self.define_old_value())?;
        write!(f, ", {}: {}", "define_new_value", self.define_new_value())?;
        write!(f, ", {}: {}", "withdrawal_keys", self.withdrawal_keys())?;
        write!(f, ", {}: {}", "withdrawal_values", self.withdrawal_values())?;
        write!(f, ", {}: {}", "proof", self.proof())?;
        write!(f, ", {}: {}", "action", self.action())?;
        let extra_count = self.count_extra_fields();
        if extra_count != 0 {
            write!(f, ", .. ({} fields)", extra_count)?;
        }
        write!(f, " }}")
    }
}
impl<'r> DefineCotaNFTEntriesReader<'r> {
    pub const FIELD_COUNT: usize = 7;

    pub fn total_size(&self) -> usize {
        molecule::unpack_number(self.as_slice()) as usize
    }

    pub fn field_count(&self) -> usize {
        if self.total_size() == molecule::NUMBER_SIZE {
            0
        } else {
            (molecule::unpack_number(&self.as_slice()[molecule::NUMBER_SIZE..]) as usize / 4) - 1
        }
    }

    pub fn count_extra_fields(&self) -> usize {
        self.field_count() - Self::FIELD_COUNT
    }

    pub fn has_extra_fields(&self) -> bool {
        Self::FIELD_COUNT != self.field_count()
    }

    pub fn define_key(&self) -> DefineCotaNFTKeyReader<'r> {
        let slice = self.as_slice();
        let start = molecule::unpack_number(&slice[4..]) as usize;
        let end = molecule::unpack_number(&slice[8..]) as usize;
        DefineCotaNFTKeyReader::new_unchecked(&self.as_slice()[start..end])
    }

    pub fn define_old_value(&self) -> DefineCotaNFTValueReader<'r> {
        let slice = self.as_slice();
        let start = molecule::unpack_number(&slice[8..]) as usize;
        let end = molecule::unpack_number(&slice[12..]) as usize;
        DefineCotaNFTValueReader::new_unchecked(&self.as_slice()[start..end])
    }

    pub fn define_new_value(&self) -> DefineCotaNFTValueReader<'r> {
        let slice = self.as_slice();
        let start = molecule::unpack_number(&slice[12..]) as usize;
        let end = molecule::unpack_number(&slice[16..]) as usize;
        DefineCotaNFTValueReader::new_unchecked(&self.as_slice()[start..end])
    }

    pub fn withdrawal_keys(&self) -> WithdrawalCotaNFTKeyVecReader<'r> {
        let slice = self.as_slice();
        let start = molecule::unpack_number(&slice[16..]) as usize;
        let end = molecule::unpack_number(&slice[20..]) as usize;
        WithdrawalCotaNFTKeyVecReader::new_unchecked(&self.as_slice()[start..end])
    }

    pub fn withdrawal_values(&self) -> WithdrawalCotaNFTValueVecReader<'r> {
        let slice = self.as_slice();
        let start = molecule::unpack_number(&slice[20..]) as usize;
        let end = molecule::unpack_number(&slice[24..]) as usize;
        WithdrawalCotaNFTValueVecReader::new_unchecked(&self.as_slice()[start..end])
    }

    pub fn proof(&self) -> BytesReader<'r> {
        let slice = self.as_slice();
        let start = molecule::unpack_number(&slice[24..]) as usize;
        let end = molecule::unpack_number(&slice[28..]) as usize;
        BytesReader::new_unchecked(&self.as_slice()[start..end])
    }

    pub fn action(&self) -> BytesReader<'r> {
        let slice = self.as_slice();
        let start = molecule::unpack_number(&slice[28..]) as usize;
        if self.has_extra_fields() {
            let end = molecule::unpack_number(&slice[32..]) as usize;
            BytesReader::new_unchecked(&self.as_slice()[start..end])
        } else {
            BytesReader::new_unchecked(&self.as_slice()[start..])
        }
    }
}
impl<'r> molecule::prelude::Reader<'r> for DefineCotaNFTEntriesReader<'r> {
    type Entity = DefineCotaNFTEntries;

    const NAME: &'static str = "DefineCotaNFTEntriesReader";

    fn to_entity(&self) -> Self::Entity {
        Self::Entity::new_unchecked(self.as_slice().to_owned().into())
    }

    fn new_unchecked(slice: &'r [u8]) -> Self {
        DefineCotaNFTEntriesReader(slice)
    }

    fn as_slice(&self) -> &'r [u8] {
        self.0
    }

    fn verify(slice: &[u8], compatible: bool) -> molecule::error::VerificationResult<()> {
        use molecule::verification_error as ve;
        let slice_len = slice.len();
        if slice_len < molecule::NUMBER_SIZE {
            return ve!(Self, HeaderIsBroken, molecule::NUMBER_SIZE, slice_len);
        }
        let total_size = molecule::unpack_number(slice) as usize;
        if slice_len != total_size {
            return ve!(Self, TotalSizeNotMatch, total_size, slice_len);
        }
        if slice_len == molecule::NUMBER_SIZE && Self::FIELD_COUNT == 0 {
            return Ok(());
        }
        if slice_len < molecule::NUMBER_SIZE * 2 {
            return ve!(Self, HeaderIsBroken, molecule::NUMBER_SIZE * 2, slice_len);
        }
        let offset_first = molecule::unpack_number(&slice[molecule::NUMBER_SIZE..]) as usize;
        if offset_first % molecule::NUMBER_SIZE != 0 || offset_first < molecule::NUMBER_SIZE * 2 {
            return ve!(Self, OffsetsNotMatch);
        }
        if slice_len < offset_first {
            return ve!(Self, HeaderIsBroken, offset_first, slice_len);
        }
        let field_count = offset_first / molecule::NUMBER_SIZE - 1;
        if field_count < Self::FIELD_COUNT {
            return ve!(Self, FieldCountNotMatch, Self::FIELD_COUNT, field_count);
        } else if !compatible && field_count > Self::FIELD_COUNT {
            return ve!(Self, FieldCountNotMatch, Self::FIELD_COUNT, field_count);
        };
        let mut offsets: Vec<usize> = slice[molecule::NUMBER_SIZE..offset_first]
            .chunks_exact(molecule::NUMBER_SIZE)
            .map(|x| molecule::unpack_number(x) as usize)
            .collect();
        offsets.push(total_size);
        if offsets.windows(2).any(|i| i[0] > i[1]) {
            return ve!(Self, OffsetsNotMatch);
        }
        DefineCotaNFTKeyReader::verify(&slice[offsets[0]..offsets[1]], compatible)?;
        DefineCotaNFTValueReader::verify(&slice[offsets[1]..offsets[2]], compatible)?;
        DefineCotaNFTValueReader::verify(&slice[offsets[2]..offsets[3]], compatible)?;
        WithdrawalCotaNFTKeyVecReader::verify(&slice[offsets[3]..offsets[4]], compatible)?;
        WithdrawalCotaNFTValueVecReader::verify(&slice[offsets[4]..offsets[5]], compatible)?;
        BytesReader::verify(&slice[offsets[5]..offsets[6]], compatible)?;
        BytesReader::verify(&slice[offsets[6]..offsets[7]], compatible)?;
        Ok(())
    }
}
#[derive(Debug, Default)]
pub struct DefineCotaNFTEntriesBuilder {
    pub(crate) define_key:        DefineCotaNFTKey,
    pub(crate) define_old_value:  DefineCotaNFTValue,
    pub(crate) define_new_value:  DefineCotaNFTValue,
    pub(crate) withdrawal_keys:   WithdrawalCotaNFTKeyVec,
    pub(crate) withdrawal_values: WithdrawalCotaNFTValueVec,
    pub(crate) proof:             Bytes,
    pub(crate) action:            Bytes,
}
impl DefineCotaNFTEntriesBuilder {
    pub const FIELD_COUNT: usize = 7;

    pub fn define_key(mut self, v: DefineCotaNFTKey) -> Self {
        self.define_key = v;
        self
    }

    pub fn define_old_value(mut self, v: DefineCotaNFTValue) -> Self {
        self.define_old_value = v;
        self
    }

    pub fn define_new_value(mut self, v: DefineCotaNFTValue) -> Self {
        self.define_new_value = v;
        self
    }

    pub fn withdrawal_keys(mut self, v: WithdrawalCotaNFTKeyVec) -> Self {
        self.withdrawal_keys = v;
        self
    }

    pub fn withdrawal_values(mut self, v: WithdrawalCotaNFTValueVec) -> Self {
        self.withdrawal_values = v;
        self
    }

    pub fn proof(mut self, v: Bytes) -> Self {
        self.proof = v;
        self
    }

    pub fn action(mut self, v: Bytes) -> Self {
        self.action = v;
        self
    }
}
impl molecule::prelude::Builder for DefineCotaNFTEntriesBuilder {
    type Entity = DefineCotaNFTEntries;

    const NAME: &'static str = "DefineCotaNFTEntriesBuilder";

    fn expected_length(&self) -> usize {
        molecule::NUMBER_SIZE * (Self::FIELD_COUNT + 1)
            + self.define_key.as_slice().len()
            + self.define_old_value.as_slice().len()
            + self.define_new_value.as_slice().len()
            + self.withdrawal_keys.as_slice().len()
            + self.withdrawal_values.as_slice().len()
            + self.proof.as_slice().len()
            + self.action.as_slice().len()
    }

    fn write<W: molecule::io::Write>(&self, writer: &mut W) -> molecule::io::Result<()> {
        let mut total_size = molecule::NUMBER_SIZE * (Self::FIELD_COUNT + 1);
        let mut offsets = Vec::with_capacity(Self::FIELD_COUNT);
        offsets.push(total_size);
        total_size += self.define_key.as_slice().len();
        offsets.push(total_size);
        total_size += self.define_old_value.as_slice().len();
        offsets.push(total_size);
        total_size += self.define_new_value.as_slice().len();
        offsets.push(total_size);
        total_size += self.withdrawal_keys.as_slice().len();
        offsets.push(total_size);
        total_size += self.withdrawal_values.as_slice().len();
        offsets.push(total_size);
        total_size += self.proof.as_slice().len();
        offsets.push(total_size);
        total_size += self.action.as_slice().len();
        writer.write_all(&molecule::pack_number(total_size as molecule::Number))?;
        for offset in offsets.into_iter() {
            writer.write_all(&molecule::pack_number(offset as molecule::Number))?;
        }
        writer.write_all(self.define_key.as_slice())?;
        writer.write_all(self.define_old_value.as_slice())?;
        writer.write_all(self.define_new_value.as_slice())?;
        writer.write_all(self.withdrawal_keys.as_slice())?;
        writer.write_all(self.withdrawal_values.as_slice())?;
        writer.write_all(self.proof.as_slice())?;
        writer.write_all(self.action.as_slice())?;
        Ok(())
    }

    fn build(&self) -> Self::Entity {
        let mut inner = Vec::with_capacity(self.expected_length());
        self.write(&mut inner)
            .unwrap_or_else(|_| panic!("{} build should be ok", Self::NAME));
        DefineCotaNFTEntries::new_unchecked(inner.into())
    }
}
