
//! Autogenerated weights for `pallet_checked_inflation`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-08-03, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `Gabriels-MacBook-Pro-3.local`, CPU: `<UNKNOWN>`
//! EXECUTION: `Some(Wasm)`, WASM-EXECUTION: `Compiled`, CHAIN: `Some("dev")`, DB CACHE: `1024`

// Executed Command:
    // ./target/release/invarch-collator
    // benchmark
    // pallet
    // --chain=dev
    // --execution=wasm
    // --wasm-execution=compiled
    // --pallet=pallet_checked_inflation
    // --extrinsic=*
    // --steps
    // 50
    // --repeat
    // 20
    // --output=../InvArch-Frames/pallet-checked-inflation/src/weights.rs
    // --template=weights-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weight functions needed for `pallet_checked_inflation`.
pub trait WeightInfo {
	  fn set_first_year_supply() -> Weight;
	  fn halt_unhalt_pallet() -> Weight;
  }

  /// Weights for `pallet_checked_inflation` using the Substrate node and recommended hardware.
  pub struct SubstrateWeight<T>(PhantomData<T>);
          impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	            /// Storage: CheckedInflation YearStartIssuance (r:0 w:1)
	            /// Proof: CheckedInflation YearStartIssuance (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	        fn set_first_year_supply() -> Weight {
		      // Proof Size summary in bytes:
		      //  Measured:  `0`
		      //  Estimated: `0`
		      // Minimum execution time: 3_000_000 picoseconds.
		      Weight::from_parts(4_000_000, 0)
			        .saturating_add(T::DbWeight::get().writes(1_u64))
	        }
	            /// Storage: CheckedInflation Halted (r:1 w:1)
	            /// Proof: CheckedInflation Halted (max_values: Some(1), max_size: Some(1), added: 496, mode: MaxEncodedLen)
	        fn halt_unhalt_pallet() -> Weight {
		      // Proof Size summary in bytes:
		      //  Measured:  `42`
		      //  Estimated: `1486`
		      // Minimum execution time: 10_000_000 picoseconds.
		      Weight::from_parts(10_000_000, 1486)
			        .saturating_add(T::DbWeight::get().reads(1_u64))
			        .saturating_add(T::DbWeight::get().writes(1_u64))
	        }
  }

  // For backwards compatibility and tests.
  impl WeightInfo for () {
	        /// Storage: CheckedInflation YearStartIssuance (r:0 w:1)
	        /// Proof: CheckedInflation YearStartIssuance (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	    fn set_first_year_supply() -> Weight {
		  // Proof Size summary in bytes:
		  //  Measured:  `0`
		  //  Estimated: `0`
		  // Minimum execution time: 3_000_000 picoseconds.
		  Weight::from_parts(4_000_000, 0)
			    .saturating_add(RocksDbWeight::get().writes(1_u64))
	    }
	        /// Storage: CheckedInflation Halted (r:1 w:1)
	        /// Proof: CheckedInflation Halted (max_values: Some(1), max_size: Some(1), added: 496, mode: MaxEncodedLen)
	    fn halt_unhalt_pallet() -> Weight {
		  // Proof Size summary in bytes:
		  //  Measured:  `42`
		  //  Estimated: `1486`
		  // Minimum execution time: 10_000_000 picoseconds.
		  Weight::from_parts(10_000_000, 1486)
			    .saturating_add(RocksDbWeight::get().reads(1_u64))
			    .saturating_add(RocksDbWeight::get().writes(1_u64))
	    }
  }
