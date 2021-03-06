// Generated by Molecule 0.7.3

#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(clippy::if_same_then_else)]

use super::ckb_types::prelude::*;
use super::molecule::{self, prelude::*};
extern crate alloc;
pub use alloc::vec::*;
// these lines above are manually added

use super::common::*;
#[derive(Clone)]
pub struct Registry(molecule::bytes::Bytes);
impl ::core::fmt::LowerHex for Registry {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        use molecule::hex_string;
        if f.alternate() {
            write!(f, "0x")?;
        }
        write!(f, "{}", hex_string(self.as_slice()))
    }
}
impl ::core::fmt::Debug for Registry {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        write!(f, "{}({:#x})", Self::NAME, self)
    }
}
impl ::core::fmt::Display for Registry {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        write!(f, "{} {{ ", Self::NAME)?;
        write!(f, "{}: {}", "lock_hash", self.lock_hash())?;
        write!(f, ", {}: {}", "state", self.state())?;
        write!(f, " }}")
    }
}
impl ::core::default::Default for Registry {
    fn default() -> Self {
        let v: Vec<u8> = vec![
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0,
        ];
        Registry::new_unchecked(v.into())
    }
}
impl Registry {
    pub const FIELD_COUNT: usize = 2;
    pub const FIELD_SIZES: [usize; 2] = [32, 32];
    pub const TOTAL_SIZE: usize = 64;

    pub fn lock_hash(&self) -> Byte32 {
        Byte32::new_unchecked(self.0.slice(0..32))
    }

    pub fn state(&self) -> Byte32 {
        Byte32::new_unchecked(self.0.slice(32..64))
    }

    pub fn as_reader<'r>(&'r self) -> RegistryReader<'r> {
        RegistryReader::new_unchecked(self.as_slice())
    }
}
impl molecule::prelude::Entity for Registry {
    type Builder = RegistryBuilder;

    const NAME: &'static str = "Registry";

    fn new_unchecked(data: molecule::bytes::Bytes) -> Self {
        Registry(data)
    }

    fn as_bytes(&self) -> molecule::bytes::Bytes {
        self.0.clone()
    }

    fn as_slice(&self) -> &[u8] {
        &self.0[..]
    }

    fn from_slice(slice: &[u8]) -> molecule::error::VerificationResult<Self> {
        RegistryReader::from_slice(slice).map(|reader| reader.to_entity())
    }

    fn from_compatible_slice(slice: &[u8]) -> molecule::error::VerificationResult<Self> {
        RegistryReader::from_compatible_slice(slice).map(|reader| reader.to_entity())
    }

    fn new_builder() -> Self::Builder {
        ::core::default::Default::default()
    }

    fn as_builder(self) -> Self::Builder {
        Self::new_builder()
            .lock_hash(self.lock_hash())
            .state(self.state())
    }
}
#[derive(Clone, Copy)]
pub struct RegistryReader<'r>(&'r [u8]);
impl<'r> ::core::fmt::LowerHex for RegistryReader<'r> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        use molecule::hex_string;
        if f.alternate() {
            write!(f, "0x")?;
        }
        write!(f, "{}", hex_string(self.as_slice()))
    }
}
impl<'r> ::core::fmt::Debug for RegistryReader<'r> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        write!(f, "{}({:#x})", Self::NAME, self)
    }
}
impl<'r> ::core::fmt::Display for RegistryReader<'r> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        write!(f, "{} {{ ", Self::NAME)?;
        write!(f, "{}: {}", "lock_hash", self.lock_hash())?;
        write!(f, ", {}: {}", "state", self.state())?;
        write!(f, " }}")
    }
}
impl<'r> RegistryReader<'r> {
    pub const FIELD_COUNT: usize = 2;
    pub const FIELD_SIZES: [usize; 2] = [32, 32];
    pub const TOTAL_SIZE: usize = 64;

    pub fn lock_hash(&self) -> Byte32Reader<'r> {
        Byte32Reader::new_unchecked(&self.as_slice()[0..32])
    }

    pub fn state(&self) -> Byte32Reader<'r> {
        Byte32Reader::new_unchecked(&self.as_slice()[32..64])
    }
}
impl<'r> molecule::prelude::Reader<'r> for RegistryReader<'r> {
    type Entity = Registry;

    const NAME: &'static str = "RegistryReader";

    fn to_entity(&self) -> Self::Entity {
        Self::Entity::new_unchecked(self.as_slice().to_owned().into())
    }

    fn new_unchecked(slice: &'r [u8]) -> Self {
        RegistryReader(slice)
    }

    fn as_slice(&self) -> &'r [u8] {
        self.0
    }

    fn verify(slice: &[u8], _compatible: bool) -> molecule::error::VerificationResult<()> {
        use molecule::verification_error as ve;
        let slice_len = slice.len();
        if slice_len != Self::TOTAL_SIZE {
            return ve!(Self, TotalSizeNotMatch, Self::TOTAL_SIZE, slice_len);
        }
        Ok(())
    }
}
#[derive(Debug, Default)]
pub struct RegistryBuilder {
    pub(crate) lock_hash: Byte32,
    pub(crate) state:     Byte32,
}
impl RegistryBuilder {
    pub const FIELD_COUNT: usize = 2;
    pub const FIELD_SIZES: [usize; 2] = [32, 32];
    pub const TOTAL_SIZE: usize = 64;

    pub fn lock_hash(mut self, v: Byte32) -> Self {
        self.lock_hash = v;
        self
    }

    pub fn state(mut self, v: Byte32) -> Self {
        self.state = v;
        self
    }
}
impl molecule::prelude::Builder for RegistryBuilder {
    type Entity = Registry;

    const NAME: &'static str = "RegistryBuilder";

    fn expected_length(&self) -> usize {
        Self::TOTAL_SIZE
    }

    fn write<W: molecule::io::Write>(&self, writer: &mut W) -> molecule::io::Result<()> {
        writer.write_all(self.lock_hash.as_slice())?;
        writer.write_all(self.state.as_slice())?;
        Ok(())
    }

    fn build(&self) -> Self::Entity {
        let mut inner = Vec::with_capacity(self.expected_length());
        self.write(&mut inner)
            .unwrap_or_else(|_| panic!("{} build should be ok", Self::NAME));
        Registry::new_unchecked(inner.into())
    }
}
#[derive(Clone)]
pub struct RegistryVec(molecule::bytes::Bytes);
impl ::core::fmt::LowerHex for RegistryVec {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        use molecule::hex_string;
        if f.alternate() {
            write!(f, "0x")?;
        }
        write!(f, "{}", hex_string(self.as_slice()))
    }
}
impl ::core::fmt::Debug for RegistryVec {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        write!(f, "{}({:#x})", Self::NAME, self)
    }
}
impl ::core::fmt::Display for RegistryVec {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        write!(f, "{} [", Self::NAME)?;
        for i in 0..self.len() {
            if i == 0 {
                write!(f, "{}", self.get_unchecked(i))?;
            } else {
                write!(f, ", {}", self.get_unchecked(i))?;
            }
        }
        write!(f, "]")
    }
}
impl ::core::default::Default for RegistryVec {
    fn default() -> Self {
        let v: Vec<u8> = vec![0, 0, 0, 0];
        RegistryVec::new_unchecked(v.into())
    }
}
impl RegistryVec {
    pub const ITEM_SIZE: usize = 64;

    pub fn total_size(&self) -> usize {
        molecule::NUMBER_SIZE + Self::ITEM_SIZE * self.item_count()
    }

    pub fn item_count(&self) -> usize {
        molecule::unpack_number(self.as_slice()) as usize
    }

    pub fn len(&self) -> usize {
        self.item_count()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn get(&self, idx: usize) -> Option<Registry> {
        if idx >= self.len() {
            None
        } else {
            Some(self.get_unchecked(idx))
        }
    }

    pub fn get_unchecked(&self, idx: usize) -> Registry {
        let start = molecule::NUMBER_SIZE + Self::ITEM_SIZE * idx;
        let end = start + Self::ITEM_SIZE;
        Registry::new_unchecked(self.0.slice(start..end))
    }

    pub fn as_reader<'r>(&'r self) -> RegistryVecReader<'r> {
        RegistryVecReader::new_unchecked(self.as_slice())
    }
}
impl molecule::prelude::Entity for RegistryVec {
    type Builder = RegistryVecBuilder;

    const NAME: &'static str = "RegistryVec";

    fn new_unchecked(data: molecule::bytes::Bytes) -> Self {
        RegistryVec(data)
    }

    fn as_bytes(&self) -> molecule::bytes::Bytes {
        self.0.clone()
    }

    fn as_slice(&self) -> &[u8] {
        &self.0[..]
    }

    fn from_slice(slice: &[u8]) -> molecule::error::VerificationResult<Self> {
        RegistryVecReader::from_slice(slice).map(|reader| reader.to_entity())
    }

    fn from_compatible_slice(slice: &[u8]) -> molecule::error::VerificationResult<Self> {
        RegistryVecReader::from_compatible_slice(slice).map(|reader| reader.to_entity())
    }

    fn new_builder() -> Self::Builder {
        ::core::default::Default::default()
    }

    fn as_builder(self) -> Self::Builder {
        Self::new_builder().extend(self.into_iter())
    }
}
#[derive(Clone, Copy)]
pub struct RegistryVecReader<'r>(&'r [u8]);
impl<'r> ::core::fmt::LowerHex for RegistryVecReader<'r> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        use molecule::hex_string;
        if f.alternate() {
            write!(f, "0x")?;
        }
        write!(f, "{}", hex_string(self.as_slice()))
    }
}
impl<'r> ::core::fmt::Debug for RegistryVecReader<'r> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        write!(f, "{}({:#x})", Self::NAME, self)
    }
}
impl<'r> ::core::fmt::Display for RegistryVecReader<'r> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        write!(f, "{} [", Self::NAME)?;
        for i in 0..self.len() {
            if i == 0 {
                write!(f, "{}", self.get_unchecked(i))?;
            } else {
                write!(f, ", {}", self.get_unchecked(i))?;
            }
        }
        write!(f, "]")
    }
}
impl<'r> RegistryVecReader<'r> {
    pub const ITEM_SIZE: usize = 64;

    pub fn total_size(&self) -> usize {
        molecule::NUMBER_SIZE + Self::ITEM_SIZE * self.item_count()
    }

    pub fn item_count(&self) -> usize {
        molecule::unpack_number(self.as_slice()) as usize
    }

    pub fn len(&self) -> usize {
        self.item_count()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn get(&self, idx: usize) -> Option<RegistryReader<'r>> {
        if idx >= self.len() {
            None
        } else {
            Some(self.get_unchecked(idx))
        }
    }

    pub fn get_unchecked(&self, idx: usize) -> RegistryReader<'r> {
        let start = molecule::NUMBER_SIZE + Self::ITEM_SIZE * idx;
        let end = start + Self::ITEM_SIZE;
        RegistryReader::new_unchecked(&self.as_slice()[start..end])
    }
}
impl<'r> molecule::prelude::Reader<'r> for RegistryVecReader<'r> {
    type Entity = RegistryVec;

    const NAME: &'static str = "RegistryVecReader";

    fn to_entity(&self) -> Self::Entity {
        Self::Entity::new_unchecked(self.as_slice().to_owned().into())
    }

    fn new_unchecked(slice: &'r [u8]) -> Self {
        RegistryVecReader(slice)
    }

    fn as_slice(&self) -> &'r [u8] {
        self.0
    }

    fn verify(slice: &[u8], _compatible: bool) -> molecule::error::VerificationResult<()> {
        use molecule::verification_error as ve;
        let slice_len = slice.len();
        if slice_len < molecule::NUMBER_SIZE {
            return ve!(Self, HeaderIsBroken, molecule::NUMBER_SIZE, slice_len);
        }
        let item_count = molecule::unpack_number(slice) as usize;
        if item_count == 0 {
            if slice_len != molecule::NUMBER_SIZE {
                return ve!(Self, TotalSizeNotMatch, molecule::NUMBER_SIZE, slice_len);
            }
            return Ok(());
        }
        let total_size = molecule::NUMBER_SIZE + Self::ITEM_SIZE * item_count;
        if slice_len != total_size {
            return ve!(Self, TotalSizeNotMatch, total_size, slice_len);
        }
        Ok(())
    }
}
#[derive(Debug, Default)]
pub struct RegistryVecBuilder(pub(crate) Vec<Registry>);
impl RegistryVecBuilder {
    pub const ITEM_SIZE: usize = 64;

    pub fn set(mut self, v: Vec<Registry>) -> Self {
        self.0 = v;
        self
    }

    pub fn push(mut self, v: Registry) -> Self {
        self.0.push(v);
        self
    }

    pub fn extend<T: ::core::iter::IntoIterator<Item = Registry>>(mut self, iter: T) -> Self {
        for elem in iter {
            self.0.push(elem);
        }
        self
    }

    pub fn replace(&mut self, index: usize, v: Registry) -> Option<Registry> {
        self.0
            .get_mut(index)
            .map(|item| ::core::mem::replace(item, v))
    }
}
impl molecule::prelude::Builder for RegistryVecBuilder {
    type Entity = RegistryVec;

    const NAME: &'static str = "RegistryVecBuilder";

    fn expected_length(&self) -> usize {
        molecule::NUMBER_SIZE + Self::ITEM_SIZE * self.0.len()
    }

    fn write<W: molecule::io::Write>(&self, writer: &mut W) -> molecule::io::Result<()> {
        writer.write_all(&molecule::pack_number(self.0.len() as molecule::Number))?;
        for inner in &self.0[..] {
            writer.write_all(inner.as_slice())?;
        }
        Ok(())
    }

    fn build(&self) -> Self::Entity {
        let mut inner = Vec::with_capacity(self.expected_length());
        self.write(&mut inner)
            .unwrap_or_else(|_| panic!("{} build should be ok", Self::NAME));
        RegistryVec::new_unchecked(inner.into())
    }
}
pub struct RegistryVecIterator(RegistryVec, usize, usize);
impl ::core::iter::Iterator for RegistryVecIterator {
    type Item = Registry;

    fn next(&mut self) -> Option<Self::Item> {
        if self.1 >= self.2 {
            None
        } else {
            let ret = self.0.get_unchecked(self.1);
            self.1 += 1;
            Some(ret)
        }
    }
}
impl ::core::iter::ExactSizeIterator for RegistryVecIterator {
    fn len(&self) -> usize {
        self.2 - self.1
    }
}
impl ::core::iter::IntoIterator for RegistryVec {
    type IntoIter = RegistryVecIterator;
    type Item = Registry;

    fn into_iter(self) -> Self::IntoIter {
        let len = self.len();
        RegistryVecIterator(self, 0, len)
    }
}
impl<'r> RegistryVecReader<'r> {
    pub fn iter<'t>(&'t self) -> RegistryVecReaderIterator<'t, 'r> {
        RegistryVecReaderIterator(&self, 0, self.len())
    }
}
pub struct RegistryVecReaderIterator<'t, 'r>(&'t RegistryVecReader<'r>, usize, usize);
impl<'t: 'r, 'r> ::core::iter::Iterator for RegistryVecReaderIterator<'t, 'r> {
    type Item = RegistryReader<'t>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.1 >= self.2 {
            None
        } else {
            let ret = self.0.get_unchecked(self.1);
            self.1 += 1;
            Some(ret)
        }
    }
}
impl<'t: 'r, 'r> ::core::iter::ExactSizeIterator for RegistryVecReaderIterator<'t, 'r> {
    fn len(&self) -> usize {
        self.2 - self.1
    }
}
#[derive(Clone)]
pub struct CotaNFTRegistryEntries(molecule::bytes::Bytes);
impl ::core::fmt::LowerHex for CotaNFTRegistryEntries {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        use molecule::hex_string;
        if f.alternate() {
            write!(f, "0x")?;
        }
        write!(f, "{}", hex_string(self.as_slice()))
    }
}
impl ::core::fmt::Debug for CotaNFTRegistryEntries {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        write!(f, "{}({:#x})", Self::NAME, self)
    }
}
impl ::core::fmt::Display for CotaNFTRegistryEntries {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        write!(f, "{} {{ ", Self::NAME)?;
        write!(f, "{}: {}", "registries", self.registries())?;
        write!(f, ", {}: {}", "proof", self.proof())?;
        let extra_count = self.count_extra_fields();
        if extra_count != 0 {
            write!(f, ", .. ({} fields)", extra_count)?;
        }
        write!(f, " }}")
    }
}
impl ::core::default::Default for CotaNFTRegistryEntries {
    fn default() -> Self {
        let v: Vec<u8> = vec![
            20, 0, 0, 0, 12, 0, 0, 0, 16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ];
        CotaNFTRegistryEntries::new_unchecked(v.into())
    }
}
impl CotaNFTRegistryEntries {
    pub const FIELD_COUNT: usize = 2;

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

    pub fn registries(&self) -> RegistryVec {
        let slice = self.as_slice();
        let start = molecule::unpack_number(&slice[4..]) as usize;
        let end = molecule::unpack_number(&slice[8..]) as usize;
        RegistryVec::new_unchecked(self.0.slice(start..end))
    }

    pub fn proof(&self) -> Bytes {
        let slice = self.as_slice();
        let start = molecule::unpack_number(&slice[8..]) as usize;
        if self.has_extra_fields() {
            let end = molecule::unpack_number(&slice[12..]) as usize;
            Bytes::new_unchecked(self.0.slice(start..end))
        } else {
            Bytes::new_unchecked(self.0.slice(start..))
        }
    }

    pub fn as_reader<'r>(&'r self) -> CotaNFTRegistryEntriesReader<'r> {
        CotaNFTRegistryEntriesReader::new_unchecked(self.as_slice())
    }
}
impl molecule::prelude::Entity for CotaNFTRegistryEntries {
    type Builder = CotaNFTRegistryEntriesBuilder;

    const NAME: &'static str = "CotaNFTRegistryEntries";

    fn new_unchecked(data: molecule::bytes::Bytes) -> Self {
        CotaNFTRegistryEntries(data)
    }

    fn as_bytes(&self) -> molecule::bytes::Bytes {
        self.0.clone()
    }

    fn as_slice(&self) -> &[u8] {
        &self.0[..]
    }

    fn from_slice(slice: &[u8]) -> molecule::error::VerificationResult<Self> {
        CotaNFTRegistryEntriesReader::from_slice(slice).map(|reader| reader.to_entity())
    }

    fn from_compatible_slice(slice: &[u8]) -> molecule::error::VerificationResult<Self> {
        CotaNFTRegistryEntriesReader::from_compatible_slice(slice).map(|reader| reader.to_entity())
    }

    fn new_builder() -> Self::Builder {
        ::core::default::Default::default()
    }

    fn as_builder(self) -> Self::Builder {
        Self::new_builder()
            .registries(self.registries())
            .proof(self.proof())
    }
}
#[derive(Clone, Copy)]
pub struct CotaNFTRegistryEntriesReader<'r>(&'r [u8]);
impl<'r> ::core::fmt::LowerHex for CotaNFTRegistryEntriesReader<'r> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        use molecule::hex_string;
        if f.alternate() {
            write!(f, "0x")?;
        }
        write!(f, "{}", hex_string(self.as_slice()))
    }
}
impl<'r> ::core::fmt::Debug for CotaNFTRegistryEntriesReader<'r> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        write!(f, "{}({:#x})", Self::NAME, self)
    }
}
impl<'r> ::core::fmt::Display for CotaNFTRegistryEntriesReader<'r> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        write!(f, "{} {{ ", Self::NAME)?;
        write!(f, "{}: {}", "registries", self.registries())?;
        write!(f, ", {}: {}", "proof", self.proof())?;
        let extra_count = self.count_extra_fields();
        if extra_count != 0 {
            write!(f, ", .. ({} fields)", extra_count)?;
        }
        write!(f, " }}")
    }
}
impl<'r> CotaNFTRegistryEntriesReader<'r> {
    pub const FIELD_COUNT: usize = 2;

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

    pub fn registries(&self) -> RegistryVecReader<'r> {
        let slice = self.as_slice();
        let start = molecule::unpack_number(&slice[4..]) as usize;
        let end = molecule::unpack_number(&slice[8..]) as usize;
        RegistryVecReader::new_unchecked(&self.as_slice()[start..end])
    }

    pub fn proof(&self) -> BytesReader<'r> {
        let slice = self.as_slice();
        let start = molecule::unpack_number(&slice[8..]) as usize;
        if self.has_extra_fields() {
            let end = molecule::unpack_number(&slice[12..]) as usize;
            BytesReader::new_unchecked(&self.as_slice()[start..end])
        } else {
            BytesReader::new_unchecked(&self.as_slice()[start..])
        }
    }
}
impl<'r> molecule::prelude::Reader<'r> for CotaNFTRegistryEntriesReader<'r> {
    type Entity = CotaNFTRegistryEntries;

    const NAME: &'static str = "CotaNFTRegistryEntriesReader";

    fn to_entity(&self) -> Self::Entity {
        Self::Entity::new_unchecked(self.as_slice().to_owned().into())
    }

    fn new_unchecked(slice: &'r [u8]) -> Self {
        CotaNFTRegistryEntriesReader(slice)
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
        RegistryVecReader::verify(&slice[offsets[0]..offsets[1]], compatible)?;
        BytesReader::verify(&slice[offsets[1]..offsets[2]], compatible)?;
        Ok(())
    }
}
#[derive(Debug, Default)]
pub struct CotaNFTRegistryEntriesBuilder {
    pub(crate) registries: RegistryVec,
    pub(crate) proof:      Bytes,
}
impl CotaNFTRegistryEntriesBuilder {
    pub const FIELD_COUNT: usize = 2;

    pub fn registries(mut self, v: RegistryVec) -> Self {
        self.registries = v;
        self
    }

    pub fn proof(mut self, v: Bytes) -> Self {
        self.proof = v;
        self
    }
}
impl molecule::prelude::Builder for CotaNFTRegistryEntriesBuilder {
    type Entity = CotaNFTRegistryEntries;

    const NAME: &'static str = "CotaNFTRegistryEntriesBuilder";

    fn expected_length(&self) -> usize {
        molecule::NUMBER_SIZE * (Self::FIELD_COUNT + 1)
            + self.registries.as_slice().len()
            + self.proof.as_slice().len()
    }

    fn write<W: molecule::io::Write>(&self, writer: &mut W) -> molecule::io::Result<()> {
        let mut total_size = molecule::NUMBER_SIZE * (Self::FIELD_COUNT + 1);
        let mut offsets = Vec::with_capacity(Self::FIELD_COUNT);
        offsets.push(total_size);
        total_size += self.registries.as_slice().len();
        offsets.push(total_size);
        total_size += self.proof.as_slice().len();
        writer.write_all(&molecule::pack_number(total_size as molecule::Number))?;
        for offset in offsets.into_iter() {
            writer.write_all(&molecule::pack_number(offset as molecule::Number))?;
        }
        writer.write_all(self.registries.as_slice())?;
        writer.write_all(self.proof.as_slice())?;
        Ok(())
    }

    fn build(&self) -> Self::Entity {
        let mut inner = Vec::with_capacity(self.expected_length());
        self.write(&mut inner)
            .unwrap_or_else(|_| panic!("{} build should be ok", Self::NAME));
        CotaNFTRegistryEntries::new_unchecked(inner.into())
    }
}
