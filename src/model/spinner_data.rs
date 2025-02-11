use dashmap::DashMap;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::sync::Arc;
use tokio::sync::OnceCell;

use super::user_balance::UserBalance;

static SPINNER_DATA_MAP: OnceCell<Arc<DashMap<String, SpinnerData>>> = OnceCell::const_new();

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SpinnerData {
    pub address: String,
    pub stats: Value,
    pub balance: UserBalance,
}

impl SpinnerData {
    pub async fn init() {
        SPINNER_DATA_MAP.set(Arc::new(DashMap::new())).unwrap();
    }

    fn default() -> SpinnerData {
        SpinnerData {
            address: "".to_string(),
            stats: json!({}),
            balance: UserBalance { gas: Decimal::ZERO },
        }
    }

    fn storage() -> &'static Arc<DashMap<String, SpinnerData>> {
        SPINNER_DATA_MAP
            .get()
            .expect("SPINNER_DATA_MAP not initialized")
    }

    pub fn get_or_create(key: &str) -> SpinnerData {
        let storage = Self::storage();

        if let Some(existing) = storage.get(key) {
            return existing.clone();
        }

        let new_data = SpinnerData::default();

        storage.insert(key.to_string(), new_data.clone());
        new_data
    }

    pub fn get(key: &str) -> Option<SpinnerData> {
        Self::storage().get(key).map(|entry| entry.clone())
    }

    pub fn update<F>(key: &str, updater: F)
    where
        F: FnOnce(&mut SpinnerData),
    {
        let storage = Self::storage();

        if let Some(mut entry) = storage.get_mut(key) {
            updater(&mut entry);
        } else {
            let mut new_data = SpinnerData::default();
            updater(&mut new_data);
            storage.insert(key.to_string(), new_data);
        }
    }
}
