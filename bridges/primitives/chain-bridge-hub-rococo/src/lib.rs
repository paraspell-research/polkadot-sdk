// Copyright 2022 Parity Technologies (UK) Ltd.
// This file is part of Parity Bridges Common.

// Parity Bridges Common is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Parity Bridges Common is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Parity Bridges Common.  If not, see <http://www.gnu.org/licenses/>.

//! Module with configuration which reflects BridgeHubRococo runtime setup (AccountId, Headers,
//! Hashes...)

#![cfg_attr(not(feature = "std"), no_std)]

use bp_messages::*;
pub use bp_polkadot_core::*;
use bp_runtime::{
	decl_bridge_finality_runtime_apis, decl_bridge_messages_runtime_apis, Chain, Parachain,
};
use frame_support::{
	dispatch::DispatchClass,
	parameter_types,
	sp_runtime::{MultiAddress, MultiSigner},
	weights::{
		constants::ExtrinsicBaseWeight, WeightToFeeCoefficient, WeightToFeeCoefficients,
		WeightToFeePolynomial,
	},
	RuntimeDebug,
};
use sp_std::prelude::*;

/// BridgeHubRococo parachain.
#[derive(RuntimeDebug)]
pub struct BridgeHubRococo;

impl Chain for BridgeHubRococo {
	type BlockNumber = BlockNumber;
	type Hash = Hash;
	type Hasher = Hasher;
	type Header = Header;

	type AccountId = AccountId;
	type Balance = Balance;
	type Index = Index;
	type Signature = Signature;

	fn max_extrinsic_size() -> u32 {
		*BlockLength::get().max.get(DispatchClass::Normal)
	}

	fn max_extrinsic_weight() -> Weight {
		BlockWeights::get()
			.get(DispatchClass::Normal)
			.max_extrinsic
			.unwrap_or(Weight::MAX)
	}
}

impl Parachain for BridgeHubRococo {
	const PARACHAIN_ID: u32 = BRIDGE_HUB_ROCOCO_PARACHAIN_ID;
}

/// [`WeightToFee`] should reflect cumulus/bridge-hub-rococo-runtime [`WeightToFee`]
pub struct WeightToFee;
impl WeightToFeePolynomial for WeightToFee {
	type Balance = Balance;
	fn polynomial() -> WeightToFeeCoefficients<Self::Balance> {
		pub const CENTS: Balance = polkadot_runtime_constants::currency::CENTS;

		// in Rococo, extrinsic base weight (smallest non-zero weight) is mapped to 1/10 CENT:
		// in BridgeHub, we map to 1/10 of that, or 1/100 CENT
		let p = CENTS;
		let q = 100 * Balance::from(ExtrinsicBaseWeight::get().ref_time());
		smallvec::smallvec![WeightToFeeCoefficient {
			degree: 1,
			negative: false,
			coeff_frac: Perbill::from_rational(p % q, q),
			coeff_integer: p / q,
		}]
	}
}

/// Public key of the chain account that may be used to verify signatures.
pub type AccountSigner = MultiSigner;

/// The address format for describing accounts.
pub type Address = MultiAddress<AccountId, ()>;

/// Identifier of BridgeHubRococo in the Rococo relay chain.
pub const BRIDGE_HUB_ROCOCO_PARACHAIN_ID: u32 = 1013;

/// Name of the With-BridgeHubRococo messages pallet instance that is deployed at bridged chains.
pub const WITH_BRIDGE_HUB_ROCOCO_MESSAGES_PALLET_NAME: &str = "BridgeRococoMessages";

parameter_types! {
	pub const SS58Prefix: u16 = 42;
}

decl_bridge_finality_runtime_apis!(bridge_hub_rococo);
decl_bridge_messages_runtime_apis!(bridge_hub_rococo);