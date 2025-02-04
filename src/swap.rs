//! Defines the high level of a swap between a Arbitrating blockchain and a Accordant blockchain
//! and its concrete instances of swaps.

use std::fmt::{self, Debug};
use std::io;

use crate::consensus::{self, CanonicalBytes, Decodable, Encodable};
use crate::role::{Accordant, Arbitrating};

use lightning_encoding::strategies::AsStrict;

pub mod btcxmr;

fixed_hash::construct_fixed_hash!(
    /// A unique swap identifier represented as an 32 bytes hash.
    #[cfg_attr(
        feature = "serde",
        derive(Serialize, Deserialize),
        serde(crate = "serde_crate"),
    )]
    pub struct SwapId(32);
);

impl Encodable for SwapId {
    fn consensus_encode<W: io::Write>(&self, s: &mut W) -> Result<usize, io::Error> {
        self.0.consensus_encode(s)
    }
}

impl Decodable for SwapId {
    fn consensus_decode<D: io::Read>(d: &mut D) -> Result<Self, consensus::Error> {
        let bytes: [u8; 32] = Decodable::consensus_decode(d)?;
        Ok(Self::from_slice(&bytes))
    }
}

impl_strict_encoding!(SwapId);

impl lightning_encoding::Strategy for SwapId {
    type Strategy = AsStrict;
}

/// Specify the context of a swap, fixing the arbitrating blockchain, the accordant blockchain and
/// the link between them.
pub trait Swap: Debug + Clone {
    /// The arbitrating blockchain concrete implementation used for the swap.
    type Ar: Arbitrating;

    /// The accordant blockchain concrete implementation used for the swap.
    type Ac: Accordant;

    ///// The concrete type to link both blockchain cryptographic groups used in by the signatures.
    type Proof: Clone + Debug + CanonicalBytes;

    /// Commitment type used in the commit/reveal scheme during swap setup.
    type Commitment: Clone + PartialEq + Eq + Debug + fmt::Display + CanonicalBytes;
}
