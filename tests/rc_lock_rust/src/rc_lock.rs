// Generated by Molecule 0.7.0
#![allow(unused_imports)]

use ckb_types::molecule;
use ckb_types::packed::*;
use ckb_types::prelude::*;
// these lines above are manually added
// replace "::molecule" to "molecule" in below code

use super::xudt_rce_mol::*;

use super::xudt_rce::*;
use molecule::prelude::*;
#[derive(Clone)]
pub struct SmtProofEntryVecOpt(molecule::bytes::Bytes);
impl ::core::fmt::LowerHex for SmtProofEntryVecOpt {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        use molecule::hex_string;
        if f.alternate() {
            write!(f, "0x")?;
        }
        write!(f, "{}", hex_string(self.as_slice()))
    }
}
impl ::core::fmt::Debug for SmtProofEntryVecOpt {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        write!(f, "{}({:#x})", Self::NAME, self)
    }
}
impl ::core::fmt::Display for SmtProofEntryVecOpt {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        if let Some(v) = self.to_opt() {
            write!(f, "{}(Some({}))", Self::NAME, v)
        } else {
            write!(f, "{}(None)", Self::NAME)
        }
    }
}
impl ::core::default::Default for SmtProofEntryVecOpt {
    fn default() -> Self {
        let v: Vec<u8> = vec![];
        SmtProofEntryVecOpt::new_unchecked(v.into())
    }
}
impl SmtProofEntryVecOpt {
    pub fn is_none(&self) -> bool {
        self.0.is_empty()
    }
    pub fn is_some(&self) -> bool {
        !self.0.is_empty()
    }
    pub fn to_opt(&self) -> Option<SmtProofEntryVec> {
        if self.is_none() {
            None
        } else {
            Some(SmtProofEntryVec::new_unchecked(self.0.clone()))
        }
    }
    pub fn as_reader<'r>(&'r self) -> SmtProofEntryVecOptReader<'r> {
        SmtProofEntryVecOptReader::new_unchecked(self.as_slice())
    }
}
impl molecule::prelude::Entity for SmtProofEntryVecOpt {
    type Builder = SmtProofEntryVecOptBuilder;
    const NAME: &'static str = "SmtProofEntryVecOpt";
    fn new_unchecked(data: molecule::bytes::Bytes) -> Self {
        SmtProofEntryVecOpt(data)
    }
    fn as_bytes(&self) -> molecule::bytes::Bytes {
        self.0.clone()
    }
    fn as_slice(&self) -> &[u8] {
        &self.0[..]
    }
    fn from_slice(slice: &[u8]) -> molecule::error::VerificationResult<Self> {
        SmtProofEntryVecOptReader::from_slice(slice).map(|reader| reader.to_entity())
    }
    fn from_compatible_slice(slice: &[u8]) -> molecule::error::VerificationResult<Self> {
        SmtProofEntryVecOptReader::from_compatible_slice(slice).map(|reader| reader.to_entity())
    }
    fn new_builder() -> Self::Builder {
        ::core::default::Default::default()
    }
    fn as_builder(self) -> Self::Builder {
        Self::new_builder().set(self.to_opt())
    }
}
#[derive(Clone, Copy)]
pub struct SmtProofEntryVecOptReader<'r>(&'r [u8]);
impl<'r> ::core::fmt::LowerHex for SmtProofEntryVecOptReader<'r> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        use molecule::hex_string;
        if f.alternate() {
            write!(f, "0x")?;
        }
        write!(f, "{}", hex_string(self.as_slice()))
    }
}
impl<'r> ::core::fmt::Debug for SmtProofEntryVecOptReader<'r> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        write!(f, "{}({:#x})", Self::NAME, self)
    }
}
impl<'r> ::core::fmt::Display for SmtProofEntryVecOptReader<'r> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        if let Some(v) = self.to_opt() {
            write!(f, "{}(Some({}))", Self::NAME, v)
        } else {
            write!(f, "{}(None)", Self::NAME)
        }
    }
}
impl<'r> SmtProofEntryVecOptReader<'r> {
    pub fn is_none(&self) -> bool {
        self.0.is_empty()
    }
    pub fn is_some(&self) -> bool {
        !self.0.is_empty()
    }
    pub fn to_opt(&self) -> Option<SmtProofEntryVecReader<'r>> {
        if self.is_none() {
            None
        } else {
            Some(SmtProofEntryVecReader::new_unchecked(self.as_slice()))
        }
    }
}
impl<'r> molecule::prelude::Reader<'r> for SmtProofEntryVecOptReader<'r> {
    type Entity = SmtProofEntryVecOpt;
    const NAME: &'static str = "SmtProofEntryVecOptReader";
    fn to_entity(&self) -> Self::Entity {
        Self::Entity::new_unchecked(self.as_slice().to_owned().into())
    }
    fn new_unchecked(slice: &'r [u8]) -> Self {
        SmtProofEntryVecOptReader(slice)
    }
    fn as_slice(&self) -> &'r [u8] {
        self.0
    }
    fn verify(slice: &[u8], compatible: bool) -> molecule::error::VerificationResult<()> {
        if !slice.is_empty() {
            SmtProofEntryVecReader::verify(&slice[..], compatible)?;
        }
        Ok(())
    }
}
#[derive(Debug, Default)]
pub struct SmtProofEntryVecOptBuilder(pub(crate) Option<SmtProofEntryVec>);
impl SmtProofEntryVecOptBuilder {
    pub fn set(mut self, v: Option<SmtProofEntryVec>) -> Self {
        self.0 = v;
        self
    }
}
impl molecule::prelude::Builder for SmtProofEntryVecOptBuilder {
    type Entity = SmtProofEntryVecOpt;
    const NAME: &'static str = "SmtProofEntryVecOptBuilder";
    fn expected_length(&self) -> usize {
        self.0
            .as_ref()
            .map(|ref inner| inner.as_slice().len())
            .unwrap_or(0)
    }
    fn write<W: ::molecule::io::Write>(&self, writer: &mut W) -> ::molecule::io::Result<()> {
        self.0
            .as_ref()
            .map(|ref inner| writer.write_all(inner.as_slice()))
            .unwrap_or(Ok(()))
    }
    fn build(&self) -> Self::Entity {
        let mut inner = Vec::with_capacity(self.expected_length());
        self.write(&mut inner)
            .unwrap_or_else(|_| panic!("{} build should be ok", Self::NAME));
        SmtProofEntryVecOpt::new_unchecked(inner.into())
    }
}
#[derive(Clone)]
pub struct RcLockWitnessLock(molecule::bytes::Bytes);
impl ::core::fmt::LowerHex for RcLockWitnessLock {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        use molecule::hex_string;
        if f.alternate() {
            write!(f, "0x")?;
        }
        write!(f, "{}", hex_string(self.as_slice()))
    }
}
impl ::core::fmt::Debug for RcLockWitnessLock {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        write!(f, "{}({:#x})", Self::NAME, self)
    }
}
impl ::core::fmt::Display for RcLockWitnessLock {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        write!(f, "{} {{ ", Self::NAME)?;
        write!(f, "{}: {}", "signature", self.signature())?;
        write!(f, ", {}: {}", "proofs", self.proofs())?;
        let extra_count = self.count_extra_fields();
        if extra_count != 0 {
            write!(f, ", .. ({} fields)", extra_count)?;
        }
        write!(f, " }}")
    }
}
impl ::core::default::Default for RcLockWitnessLock {
    fn default() -> Self {
        let v: Vec<u8> = vec![12, 0, 0, 0, 12, 0, 0, 0, 12, 0, 0, 0];
        RcLockWitnessLock::new_unchecked(v.into())
    }
}
impl RcLockWitnessLock {
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
    pub fn signature(&self) -> BytesOpt {
        let slice = self.as_slice();
        let start = molecule::unpack_number(&slice[4..]) as usize;
        let end = molecule::unpack_number(&slice[8..]) as usize;
        BytesOpt::new_unchecked(self.0.slice(start..end))
    }
    pub fn proofs(&self) -> SmtProofEntryVecOpt {
        let slice = self.as_slice();
        let start = molecule::unpack_number(&slice[8..]) as usize;
        if self.has_extra_fields() {
            let end = molecule::unpack_number(&slice[12..]) as usize;
            SmtProofEntryVecOpt::new_unchecked(self.0.slice(start..end))
        } else {
            SmtProofEntryVecOpt::new_unchecked(self.0.slice(start..))
        }
    }
    pub fn as_reader<'r>(&'r self) -> RcLockWitnessLockReader<'r> {
        RcLockWitnessLockReader::new_unchecked(self.as_slice())
    }
}
impl molecule::prelude::Entity for RcLockWitnessLock {
    type Builder = RcLockWitnessLockBuilder;
    const NAME: &'static str = "RcLockWitnessLock";
    fn new_unchecked(data: molecule::bytes::Bytes) -> Self {
        RcLockWitnessLock(data)
    }
    fn as_bytes(&self) -> molecule::bytes::Bytes {
        self.0.clone()
    }
    fn as_slice(&self) -> &[u8] {
        &self.0[..]
    }
    fn from_slice(slice: &[u8]) -> molecule::error::VerificationResult<Self> {
        RcLockWitnessLockReader::from_slice(slice).map(|reader| reader.to_entity())
    }
    fn from_compatible_slice(slice: &[u8]) -> molecule::error::VerificationResult<Self> {
        RcLockWitnessLockReader::from_compatible_slice(slice).map(|reader| reader.to_entity())
    }
    fn new_builder() -> Self::Builder {
        ::core::default::Default::default()
    }
    fn as_builder(self) -> Self::Builder {
        Self::new_builder()
            .signature(self.signature())
            .proofs(self.proofs())
    }
}
#[derive(Clone, Copy)]
pub struct RcLockWitnessLockReader<'r>(&'r [u8]);
impl<'r> ::core::fmt::LowerHex for RcLockWitnessLockReader<'r> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        use molecule::hex_string;
        if f.alternate() {
            write!(f, "0x")?;
        }
        write!(f, "{}", hex_string(self.as_slice()))
    }
}
impl<'r> ::core::fmt::Debug for RcLockWitnessLockReader<'r> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        write!(f, "{}({:#x})", Self::NAME, self)
    }
}
impl<'r> ::core::fmt::Display for RcLockWitnessLockReader<'r> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        write!(f, "{} {{ ", Self::NAME)?;
        write!(f, "{}: {}", "signature", self.signature())?;
        write!(f, ", {}: {}", "proofs", self.proofs())?;
        let extra_count = self.count_extra_fields();
        if extra_count != 0 {
            write!(f, ", .. ({} fields)", extra_count)?;
        }
        write!(f, " }}")
    }
}
impl<'r> RcLockWitnessLockReader<'r> {
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
    pub fn signature(&self) -> BytesOptReader<'r> {
        let slice = self.as_slice();
        let start = molecule::unpack_number(&slice[4..]) as usize;
        let end = molecule::unpack_number(&slice[8..]) as usize;
        BytesOptReader::new_unchecked(&self.as_slice()[start..end])
    }
    pub fn proofs(&self) -> SmtProofEntryVecOptReader<'r> {
        let slice = self.as_slice();
        let start = molecule::unpack_number(&slice[8..]) as usize;
        if self.has_extra_fields() {
            let end = molecule::unpack_number(&slice[12..]) as usize;
            SmtProofEntryVecOptReader::new_unchecked(&self.as_slice()[start..end])
        } else {
            SmtProofEntryVecOptReader::new_unchecked(&self.as_slice()[start..])
        }
    }
}
impl<'r> molecule::prelude::Reader<'r> for RcLockWitnessLockReader<'r> {
    type Entity = RcLockWitnessLock;
    const NAME: &'static str = "RcLockWitnessLockReader";
    fn to_entity(&self) -> Self::Entity {
        Self::Entity::new_unchecked(self.as_slice().to_owned().into())
    }
    fn new_unchecked(slice: &'r [u8]) -> Self {
        RcLockWitnessLockReader(slice)
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
        BytesOptReader::verify(&slice[offsets[0]..offsets[1]], compatible)?;
        SmtProofEntryVecOptReader::verify(&slice[offsets[1]..offsets[2]], compatible)?;
        Ok(())
    }
}
#[derive(Debug, Default)]
pub struct RcLockWitnessLockBuilder {
    pub(crate) signature: BytesOpt,
    pub(crate) proofs: SmtProofEntryVecOpt,
}
impl RcLockWitnessLockBuilder {
    pub const FIELD_COUNT: usize = 2;
    pub fn signature(mut self, v: BytesOpt) -> Self {
        self.signature = v;
        self
    }
    pub fn proofs(mut self, v: SmtProofEntryVecOpt) -> Self {
        self.proofs = v;
        self
    }
}
impl molecule::prelude::Builder for RcLockWitnessLockBuilder {
    type Entity = RcLockWitnessLock;
    const NAME: &'static str = "RcLockWitnessLockBuilder";
    fn expected_length(&self) -> usize {
        molecule::NUMBER_SIZE * (Self::FIELD_COUNT + 1)
            + self.signature.as_slice().len()
            + self.proofs.as_slice().len()
    }
    fn write<W: ::molecule::io::Write>(&self, writer: &mut W) -> ::molecule::io::Result<()> {
        let mut total_size = molecule::NUMBER_SIZE * (Self::FIELD_COUNT + 1);
        let mut offsets = Vec::with_capacity(Self::FIELD_COUNT);
        offsets.push(total_size);
        total_size += self.signature.as_slice().len();
        offsets.push(total_size);
        total_size += self.proofs.as_slice().len();
        writer.write_all(&molecule::pack_number(total_size as molecule::Number))?;
        for offset in offsets.into_iter() {
            writer.write_all(&molecule::pack_number(offset as molecule::Number))?;
        }
        writer.write_all(self.signature.as_slice())?;
        writer.write_all(self.proofs.as_slice())?;
        Ok(())
    }
    fn build(&self) -> Self::Entity {
        let mut inner = Vec::with_capacity(self.expected_length());
        self.write(&mut inner)
            .unwrap_or_else(|_| panic!("{} build should be ok", Self::NAME));
        RcLockWitnessLock::new_unchecked(inner.into())
    }
}
