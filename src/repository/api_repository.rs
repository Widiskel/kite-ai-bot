use std::sync::Arc;

use crate::{
    model::{
        exception::{api_error::ApiError, setup_error::SetupError},
        spinner_data::SpinnerData,
    },
    service::api::api_service::{ApiService, HttpMethod},
    utils::{
        configuration::Config, constants, exception_handler::ExceptionHandler, helper::Helper,
        spinner::Spinner,
    },
};
use reqwest::StatusCode;
use serde_json::json;

pub struct ApiRepository {
    pub acc: Arc<String>,
    pub api_service: ApiService,
}

impl ApiRepository {
    pub fn new(acc: &Arc<String>) -> Result<Self, SetupError> {
        let acc_idx = Helper::get_data_index_from_file(acc, "accounts.json").unwrap_or(0);
        let proxy_list = Helper::read_data_from_file("proxy_list.json").unwrap_or(vec![]);

        let proxy = if let Some(proxy) = proxy_list.get(acc_idx as usize) {
            Some(proxy)
        } else {
            None
        };

        match ApiService::new(proxy.cloned(), None) {
            Ok(api_service) => Ok(ApiRepository {
                acc: Arc::clone(acc),
                api_service,
            }),
            Err(err) => Err(err),
        }
    }

    pub async fn get_user_stats(&self, address: &str) {
        Spinner::log(&self.acc, "Getting User Stats...", 1000).await;

        let body = Some(json!({
            "address": address,
        }));

        match self
            .api_service
            .fetch(
                &format!(
                    "https://quests-usage-dev.prod.zettablock.com/api/user/{}/stats",
                    address
                ),
                Some(HttpMethod::GET),
                body,
                None,
                None,
            )
            .await
        {
            Ok(res) if res.status.is_success() => {
                Spinner::log(&self.acc, "Successfully retrieved user stats.", 1000).await;
                let stats = res.data;
                SpinnerData::update(&self.acc, |data| data.stats = stats.clone());
            }
            Ok(res) => {
                let error = ExceptionHandler::create_api_eror(res);
                ExceptionHandler::api_error(&self.acc, error).await;
            }
            Err(e) => {
                let error = ApiError {
                    code: StatusCode::INTERNAL_SERVER_ERROR,
                    message: format!("Request Failed: {}", e),
                };
                ExceptionHandler::api_error(&self.acc, error).await;
            }
        }
    }

    pub async fn chat_with_professor_agent(&self, address: &String) {
        Spinner::log(&self.acc, "Chatting With Professor Agent", 1000).await;

        if Config::get().real_mode {
            let message = Helper::pick_random_from_arr(&constants::PROFFESOR_MESSAGE_LIST);
            let body = Some(json!({
                "message": message,
                "stream": false
            }));
            Spinner::log(
                &self.acc,
                &format!("Sending Message {} to Professor Agent", message.unwrap()).to_string(),
                3000,
            )
            .await;

            Spinner::log(
                &self.acc,
                &format!(
                    "Message {} Send to Professor Agent \n \nAwaiting For Agent To Response",
                    message.unwrap()
                )
                .to_string(),
                1000,
            )
            .await;
            match self
                .api_service
                .fetch(
                    &format!(
                        "https://{}.stag-vxzy.zettablock.com/main",
                        constants::PROFESSOR_AGENT.to_lowercase().replace("_", "-")
                    )
                    .as_str(),
                    Some(HttpMethod::POST),
                    body,
                    None,
                    None,
                )
                .await
            {
                Ok(res) if res.status.is_success() => {
                    let ai_res = res.data;
                    let response_message = ai_res["choices"][0]["message"]["content"]
                        .as_str()
                        .unwrap_or_default();

                    Spinner::log(
                        &self.acc,
                        &format!(
                            "Receiving Message {} from Professor Agent",
                            response_message
                        )
                        .to_string(),
                        3000,
                    )
                    .await;

                    self.report_usage_onchain(
                        address,
                        constants::PROFESSOR_AGENT,
                        message.unwrap(),
                        response_message,
                    )
                    .await;
                }
                Ok(res) => {
                    let error = ExceptionHandler::create_api_eror(res);
                    ExceptionHandler::api_error(&self.acc, error).await;
                }
                Err(e) => {
                    let error = ApiError {
                        code: StatusCode::INTERNAL_SERVER_ERROR,
                        message: format!("Request Failed: {}", e),
                    };
                    ExceptionHandler::api_error(&self.acc, error).await;
                }
            }
        } else {
            let (question, answer) =
                Helper::pick_random_set(&constants::PROFFESOR_QUESTION_ANSWER_LIST);

            Spinner::log(
                &self.acc,
                &format!("Sending Message {} to Professor Agent", question).to_string(),
                3000,
            )
            .await;

            Spinner::log(
                &self.acc,
                &format!(
                    "Message {} Send to Professor Agent \n \nAwaiting For Agent To Response",
                    question,
                )
                .to_string(),
                1000,
            )
            .await;

            Spinner::log(
                &self.acc,
                &format!("Receiving Message {} from Professor Agent", answer).to_string(),
                3000,
            )
            .await;

            self.report_usage_onchain(address, constants::PROFESSOR_AGENT, question, answer)
                .await;
        }
    }

    pub async fn chat_with_buddy_agent(&self, address: &String) {
        Spinner::log(&self.acc, "Chatting With Crypto Buddy Agent", 1000).await;

        if Config::get().real_mode {
            let message = Helper::pick_random_from_arr(&constants::CRYPTO_BUDDY_MESSAGE_LIST);
            let body = Some(json!({
                "message": message,
                "stream": false
            }));
            Spinner::log(
                &self.acc,
                &format!("Sending Message {} to Crypto Buddy Agent", message.unwrap()).to_string(),
                3000,
            )
            .await;

            Spinner::log(
                &self.acc,
                &format!(
                    "Message {} Send to Crypto Buddy Agent \n \nAwaiting For Agent To Response",
                    message.unwrap()
                )
                .to_string(),
                1000,
            )
            .await;
            match self
                .api_service
                .fetch(
                    &format!(
                        "https://{}.stag-vxzy.zettablock.com/main",
                        constants::CRYPTO_BUDDY.to_lowercase().replace("_", "-")
                    )
                    .as_str(),
                    Some(HttpMethod::POST),
                    body,
                    None,
                    None,
                )
                .await
            {
                Ok(res) if res.status.is_success() => {
                    let ai_res = res.data;
                    let response_message = ai_res["choices"][0]["message"]["content"]
                        .as_str()
                        .unwrap_or_default();

                    Spinner::log(
                        &self.acc,
                        &format!(
                            "Receiving Message {} from Crypto Buddy Agent",
                            response_message
                        )
                        .to_string(),
                        3000,
                    )
                    .await;

                    self.report_usage_onchain(
                        address,
                        constants::CRYPTO_BUDDY,
                        message.unwrap(),
                        response_message,
                    )
                    .await;
                }
                Ok(res) => {
                    let error = ExceptionHandler::create_api_eror(res);
                    ExceptionHandler::api_error(&self.acc, error).await;
                }
                Err(e) => {
                    let error = ApiError {
                        code: StatusCode::INTERNAL_SERVER_ERROR,
                        message: format!("Request Failed: {}", e),
                    };
                    ExceptionHandler::api_error(&self.acc, error).await;
                }
            }
        } else {
            let (question, answer) =
                Helper::pick_random_set(&constants::CRYPTO_BUDDY_QUESTION_ANSWER_LIST);

            Spinner::log(
                &self.acc,
                &format!("Sending Message {} to Crypto Buddy Agent", question).to_string(),
                3000,
            )
            .await;

            Spinner::log(
                &self.acc,
                &format!(
                    "Message {} Send to Crypto Buddy Agent \n \nAwaiting For Agent To Response",
                    question,
                )
                .to_string(),
                1000,
            )
            .await;

            Spinner::log(
                &self.acc,
                &format!("Receiving Message {} from Crypto Buddy Agent", answer).to_string(),
                3000,
            )
            .await;

            self.report_usage_onchain(address, constants::CRYPTO_BUDDY, question, answer)
                .await;
        }
    }

    pub async fn chat_with_sherlock_agent(&self, address: &String) {
        Spinner::log(&self.acc, "Chatting With Sherlock Agent", 1000).await;

        if Config::get().real_mode {
            let message = Helper::pick_random_from_arr(&constants::SHERLOCK_MESSAGE_LIST);
            let body = Some(json!({
                "message": message,
                "stream": false
            }));
            Spinner::log(
                &self.acc,
                &format!("Sending Message {} to Sherlock Agent", message.unwrap()).to_string(),
                3000,
            )
            .await;

            Spinner::log(
                &self.acc,
                &format!(
                    "Message {} Send to Sherlock Agent \n \nAwaiting For Agent To Response",
                    message.unwrap()
                )
                .to_string(),
                1000,
            )
            .await;
            match self
                .api_service
                .fetch(
                    &format!(
                        "https://{}.stag-vxzy.zettablock.com/main",
                        constants::SHERLOCK.to_lowercase().replace("_", "-")
                    )
                    .as_str(),
                    Some(HttpMethod::POST),
                    body,
                    None,
                    None,
                )
                .await
            {
                Ok(res) if res.status.is_success() => {
                    let ai_res = res.data;
                    let response_message = ai_res["choices"][0]["message"]["content"]
                        .as_str()
                        .unwrap_or_default();

                    Spinner::log(
                        &self.acc,
                        &format!("Receiving Message {} from Sherlock Agent", response_message)
                            .to_string(),
                        3000,
                    )
                    .await;

                    self.report_usage_onchain(
                        address,
                        constants::SHERLOCK,
                        message.unwrap(),
                        response_message,
                    )
                    .await;
                }
                Ok(res) => {
                    let error = ExceptionHandler::create_api_eror(res);
                    ExceptionHandler::api_error(&self.acc, error).await;
                }
                Err(e) => {
                    let error = ApiError {
                        code: StatusCode::INTERNAL_SERVER_ERROR,
                        message: format!("Request Failed: {}", e),
                    };
                    ExceptionHandler::api_error(&self.acc, error).await;
                }
            }
        } else {
            let (question, answer) =
                Helper::pick_random_set(&constants::SHERLOCK_QUESTION_ANSWER_LIST);

            Spinner::log(
                &self.acc,
                &format!("Sending Message {} to Sherlock Agent", question).to_string(),
                3000,
            )
            .await;

            Spinner::log(
                &self.acc,
                &format!(
                    "Message {} Send to Sherlock Agent \n \nAwaiting For Agent To Response",
                    question,
                )
                .to_string(),
                1000,
            )
            .await;

            Spinner::log(
                &self.acc,
                &format!("Receiving Message {} from Sherlock Agent", answer).to_string(),
                3000,
            )
            .await;

            self.report_usage_onchain(address, constants::SHERLOCK, question, answer)
                .await;
        }
    }

    pub async fn report_usage_onchain(
        &self,
        address: &String,
        agent: &str,
        request: &str,
        response: &str,
    ) {
        Spinner::log(&self.acc, "Reporting Onchain Usage...", 1000).await;
        let body = Some(json!({
            "wallet_address": address,
            "agent_id": agent,
            "request_text": request,
            "response_text": response,
            "request_metadata": {}
        }));

        match self
            .api_service
            .fetch(
                "https://quests-usage-dev.prod.zettablock.com/api/report_usage",
                Some(HttpMethod::POST),
                body,
                None,
                None,
            )
            .await
        {
            Ok(res) if res.status.is_success() => {
                Spinner::log(&self.acc, "Successfully Report Onchain Ussage...", 1000).await;
                self.get_user_stats(address).await;
            }
            Ok(res) => {
                let error = ExceptionHandler::create_api_eror(res);
                ExceptionHandler::api_error(&self.acc, error).await;
            }
            Err(e) => {
                let error = ApiError {
                    code: StatusCode::INTERNAL_SERVER_ERROR,
                    message: format!("Request Failed: {}", e),
                };
                ExceptionHandler::api_error(&self.acc, error).await;
            }
        }
    }
}
