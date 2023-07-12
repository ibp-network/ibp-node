#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Dummy weight function needed for pallet_ibp.
pub trait WeightInfo {
	fn dummy_weight() -> Weight;
}

/// Weights for pallet_ibp using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: TemplateModule Something (r:0 w:1)
	/// Proof: TemplateModule Something (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	fn dummy_weight() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 8_000_000 picoseconds.
		Weight::from_parts(9_000_000, 0)
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	/// Storage: TemplateModule Something (r:0 w:1)
	/// Proof: TemplateModule Something (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	fn dummy_weight() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 8_000_000 picoseconds.
		Weight::from_parts(9_000_000, 0)
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
}