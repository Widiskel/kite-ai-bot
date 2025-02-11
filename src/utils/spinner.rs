use dashmap::DashMap;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use log::info;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::OnceCell;
use tokio::time::sleep;

use crate::{
    model::spinner_data::SpinnerData, service::db::rustqlite::RustQLite,
    utils::configuration::Config,
};

use super::helper::Helper;

static MULTI_PROGRESS: OnceCell<Arc<MultiProgress>> = OnceCell::const_new();
static SPINNERS: OnceCell<Arc<DashMap<String, ProgressBar>>> = OnceCell::const_new();

#[derive(Clone)]
pub struct Spinner;

impl Spinner {
    pub async fn init() {
        let multi_progress = Arc::new(MultiProgress::new());
        MULTI_PROGRESS.set(multi_progress).unwrap();

        let spinners = Arc::new(DashMap::new());
        SPINNERS.set(spinners).unwrap();

        SpinnerData::init().await;
    }

    pub async fn log(acc: &str, msg: &str, delay: u64) {
        let acc_idx = Helper::get_data_index_from_file(acc, "accounts.json");

        info!("Account {} : {}", acc_idx.unwrap_or(0) + 1, msg);
        let multi_progress = MULTI_PROGRESS.get().expect("MultiProgress not initialized");
        let spinners = SPINNERS.get().expect("Spinners not initialized");
        let spinner_data = SpinnerData::get_or_create(acc);

        let pb = spinners.entry(acc.to_string()).or_insert_with(|| {
            let progress_bar = multi_progress.add(ProgressBar::new_spinner());
            progress_bar.set_style(
                ProgressStyle::default_spinner()
                    .template("{spinner:.green} {msg}")
                    .unwrap()
                    .progress_chars("##-"),
            );
            progress_bar.set_prefix(acc.to_string());
            progress_bar.enable_steady_tick(Duration::from_millis(100));
            progress_bar
        });

        let mut remaining_duration = Duration::from_millis(delay);

        while remaining_duration > Duration::from_millis(0) {
            let formatted_message = format!(
                r#"
================= Account {} ===============
Address             : {}
Balance             : {:?}
Interaction (Today) : {:?} ({:?}/{:?})

Status : {}
Delay : {}
==========================================
"#,
                acc_idx.unwrap_or(0) + 1,
                spinner_data.address,
                spinner_data.balance.gas,
                spinner_data
                    .stats
                    .get("total_interactions")
                    .and_then(|bal| bal.as_u64())
                    .map_or_else(|| 0, |b| b),
                RustQLite::get_logs_today(&spinner_data.address, "interact")
                    .await
                    .len(),
                Config::get().interaction,
                msg,
                Helper::ms_to_time(remaining_duration.as_millis() as u64)
            );

            pb.set_message(formatted_message.clone());
            pb.tick();

            let refresh_duration = Duration::from_millis(100);
            remaining_duration = remaining_duration.saturating_sub(refresh_duration);

            sleep(refresh_duration).await;
        }

        // pb.abandon();
    }
}
