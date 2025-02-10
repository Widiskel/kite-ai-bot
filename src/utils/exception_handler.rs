use log::info;

use crate::{
    model::{
        api_response::ApiResponse,
        exception::{
            api_error::ApiError, operation_error::OperationError, setup_error::SetupError,
        },
    },
    utils::spinner::Spinner,
};
use core::panic;
use std::error::Error;

pub struct ExceptionHandler;

impl ExceptionHandler {
    pub async fn operation_error(acc: &str, error: OperationError) {
        let error_msg = format!("{}", error);
        Spinner::log(&acc, &error_msg, 5000).await;
    }

    pub fn create_api_eror(res: ApiResponse) -> ApiError {
        let message = res
            .data
            .get("error")
            .and_then(|v| v.as_str())
            .or_else(|| res.data.get("detail").and_then(|v| v.as_str()))
            .unwrap_or(res.status.as_str())
            .to_string();

        info!("{}", message);

        ApiError {
            code: res.status,
            message: message,
        }
    }
    pub async fn api_error(acc: &str, error: ApiError) {
        let error_msg = format!("{}", error);
        Spinner::log(&acc, &error_msg, 5000).await;
    }

    pub async fn setup_error(acc: &str, error: SetupError) {
        let error_msg = format!("{}, exiting in 3 seconds...", error);
        Spinner::log(&acc, &error_msg, 3000).await;
        panic!("{}", error.message);
    }

    pub async fn unknown_error(error: Box<dyn Error + Send + Sync>) {
        let error_msg = format!("Unknown Error: {}, retrying in 10 seconds...", error);
        Spinner::log("Unknown", &error_msg, 10000).await;
    }
}
