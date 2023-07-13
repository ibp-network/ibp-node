use ibp_runtime::{
	AccountId, AuraConfig, BalancesConfig, GenesisConfig, GrandpaConfig, SudoConfig, SystemConfig,
	WASM_BINARY,
};
use sc_service::{ChainType, Properties};
use sc_telemetry::serde_json::json;
use sp_consensus_aura::sr25519::AuthorityId as AuraId;
use sp_consensus_grandpa::AuthorityId as GrandpaId;
use sp_core::{crypto::Ss58Codec, Pair, Public};
use sp_runtime::AccountId32;

// The URL for the telemetry server.
// const STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";

/// Specialized `ChainSpec`. This is a specialization of the general Substrate ChainSpec type.
pub type ChainSpec = sc_service::GenericChainSpec<GenesisConfig>;

/// Generate a crypto pair from seed.
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
	TPublic::Pair::from_string(seed, None)
		.expect("static values are valid; qed")
		.public()
}

/// Generate an Aura authority key.
pub fn authority_keys_from_seed(s: &str) -> (AuraId, GrandpaId) {
	(get_from_seed::<AuraId>(s), get_from_seed::<GrandpaId>(s))
}

pub fn development_config() -> Result<ChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?;
	let helikon =
		AccountId32::from_ss58check("5EvkgeMS6NPmAimqXCk65mUF662FppxCVcmi7tnNCksoDhiw").unwrap();
	let sudo =
		AccountId32::from_ss58check("5E1kNfEhzURMNfGJhjawLr8MGfRJdtGBug7rpNtNwq2wyCjZ").unwrap();
	let treasury =
		AccountId32::from_ss58check("5F9ovDUZUBLWrtjyzFpiNRyZtyd3KPpBMWR1WK8bECwBTwYA").unwrap();
	Ok(ChainSpec::from_genesis(
		// Name
		"Development",
		// ID
		"dev",
		ChainType::Development,
		move || {
			testnet_genesis(
				wasm_binary,
				// Initial PoA authorities
				vec![
					authority_keys_from_seed(
						"0x7a86ad3dd384784a485abe272c7e5d36b435f6e7f64252c0e898d56cb8b7199b",
					),
					authority_keys_from_seed(
						"0x1c46b9f783201cffc6bfc46166423fa44ff5f316069ad88a4b7280b7dda5b3dd",
					),
				],
				// Sudo account
				helikon.clone(),
				// Pre-funded accounts
				vec![
					helikon.clone(),
					treasury.clone(),
					sudo.clone(),
					/*
					get_account_id_from_seed::<sr25519::Public>("//Alice"),
					get_account_id_from_seed::<sr25519::Public>("//Bob"),
					get_account_id_from_seed::<sr25519::Public>("//Alice//stash"),
					get_account_id_from_seed::<sr25519::Public>("//Bob//stash"),
					*/
				],
				true,
			)
		},
		// Bootnodes
		vec![],
		// Telemetry
		None,
		// Protocol ID
		None,
		None,
		// Properties
		None,
		// Extensions
		None,
	))
}

pub fn ibp_testnet_config() -> Result<ChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?;
	let mut properties: Properties = Properties::new();
	properties.insert("tokenSymbol".to_string(), json!("IBT"));
	let helikon =
		AccountId32::from_ss58check("5EvkgeMS6NPmAimqXCk65mUF662FppxCVcmi7tnNCksoDhiw").unwrap();
	let sudo =
		AccountId32::from_ss58check("5E1kNfEhzURMNfGJhjawLr8MGfRJdtGBug7rpNtNwq2wyCjZ").unwrap();
	let treasury =
		AccountId32::from_ss58check("5F9ovDUZUBLWrtjyzFpiNRyZtyd3KPpBMWR1WK8bECwBTwYA").unwrap();
	Ok(ChainSpec::from_genesis(
		// Name
		"IBP Testnet",
		// ID
		"ibp_testnet",
		ChainType::Local,
		move || {
			testnet_genesis(
				wasm_binary,
				// Initial PoA authorities
				vec![
					authority_keys_from_seed(
						"0x7a86ad3dd384784a485abe272c7e5d36b435f6e7f64252c0e898d56cb8b7199b",
					),
					authority_keys_from_seed(
						"0x1c46b9f783201cffc6bfc46166423fa44ff5f316069ad88a4b7280b7dda5b3dd",
					),
				],
				// Sudo account
				sudo.clone(),
				// Pre-funded accounts
				vec![
					helikon.clone(),
					sudo.clone(),
					treasury.clone(),
					/*
					get_account_id_from_seed::<sr25519::Public>("//Alice"),
					get_account_id_from_seed::<sr25519::Public>("//Bob"),
					get_account_id_from_seed::<sr25519::Public>("//Charlie"),
					get_account_id_from_seed::<sr25519::Public>("//Dave"),
					get_account_id_from_seed::<sr25519::Public>("//Eve"),
					get_account_id_from_seed::<sr25519::Public>("//Ferdie"),
					get_account_id_from_seed::<sr25519::Public>("//Alice//stash"),
					get_account_id_from_seed::<sr25519::Public>("//Bob//stash"),
					get_account_id_from_seed::<sr25519::Public>("//Charlie//stash"),
					get_account_id_from_seed::<sr25519::Public>("//Dave//stash"),
					get_account_id_from_seed::<sr25519::Public>("//Eve//stash"),
					get_account_id_from_seed::<sr25519::Public>("//Ferdie//stash"),
					*/
				],
				true,
			)
		},
		// Bootnodes
		vec![],
		// Telemetry
		None,
		// Protocol ID
		None,
		// Properties
		None,
		Some(properties),
		// Extensions
		None,
	))
}

/// Configure initial storage state for FRAME modules.
fn testnet_genesis(
	wasm_binary: &[u8],
	initial_authorities: Vec<(AuraId, GrandpaId)>,
	root_key: AccountId,
	endowed_accounts: Vec<AccountId>,
	_enable_println: bool,
) -> GenesisConfig {
	GenesisConfig {
		system: SystemConfig {
			// Add Wasm runtime to storage.
			code: wasm_binary.to_vec(),
		},
		balances: BalancesConfig {
			// Configure endowed accounts with initial balance of 1 << 60.
			balances: endowed_accounts.iter().cloned().map(|k| (k, 1 << 60)).collect(),
		},
		aura: AuraConfig {
			authorities: initial_authorities.iter().map(|x| (x.0.clone())).collect(),
		},
		grandpa: GrandpaConfig {
			authorities: initial_authorities.iter().map(|x| (x.1.clone(), 1)).collect(),
		},
		sudo: SudoConfig {
			// Assign network admin rights.
			key: Some(root_key),
		},
		transaction_payment: Default::default(),
	}
}
