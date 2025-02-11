use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserBalance {
    pub symbol: String,
    pub gas: Decimal,
}
