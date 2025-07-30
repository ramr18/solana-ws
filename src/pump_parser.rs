use crate::types::{PumpEvent, PumpData, TokenMetadata};
use chrono::Utc;
use log::warn;

pub fn parse_pump_tx(signature: &str, _raw_log: &str) -> Option<PumpEvent> {
    warn!("Using mock parser, implement actual logic for production.");
    Some(PumpEvent {
        event_type: "token_created".to_string(),
        timestamp: Utc::now().to_rfc3339(),
        transaction_signature: signature.to_string(),
        token: TokenMetadata {
            mint_address: "ABC123".into(),
            name: "MyToken".into(),
            symbol: "MTK".into(),
            creator: "DEF456".into(),
            supply: 1000000000,
            decimals: 6,
        },
        pump_data: PumpData {
            bonding_curve: "GHI789".into(),
            virtual_sol_reserves: 30000000000,
            virtual_token_reserves: 1073000000000000,
        },
    })
}
