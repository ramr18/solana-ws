use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct TokenEvent {
    pub event_type: String,
    pub timestamp: String,
    pub transaction_signature: String,
    pub token: TokenDetails,
    pub pump_data: PumpData,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TokenDetails {
    pub mint_address: String,
    pub name: String,
    pub symbol: String,
    pub creator: String,
    pub supply: u64,
    pub decimals: u8,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PumpData {
    pub bonding_curve: String,
    pub virtual_sol_reserves: u64,
    pub virtual_token_reserves: u64,
}
