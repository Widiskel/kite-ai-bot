use kite_ai_bot::{
    model::{exception::operation_error::OperationError, spinner_data::SpinnerData},
    repository::api_repository::ApiRepository,
    service::{db::rustqlite::RustQLite, evm_service::evm_service::EvmService},
    utils::{
        configuration::Config,
        exception_handler::ExceptionHandler,
        helper::Helper,
        logger::{self},
        network::Network,
        spinner::Spinner,
    },
};
use rust_decimal::{prelude::FromPrimitive, Decimal};
use std::sync::Arc;

#[tokio::main]
async fn main() {
    println!("{}", Helper::show_skel_logo());
    println!("BOT STARTED");

    Config::init().expect("Failed to initialize Configuration");
    logger::init_logger().expect("Failed to initialize logger");
    RustQLite::init().await;
    Spinner::init().await;

    let account_list = Helper::read_data_from_file("accounts.json");
    let proxy_list = Helper::read_data_from_file("proxy_list.json");
    if account_list.is_none() {
        panic!("No Valid Accounts Found");
    }

    if let Some(accounts) = &account_list {
        if let Some(proxies) = &proxy_list {
            if proxies.len() > 0 && accounts.len() != proxies.len() {
                panic!(
                    "You have {} accounts but only {} proxies.",
                    accounts.len(),
                    proxies.len()
                );
            }
        } else {
            panic!("Proxy file not detected, please provice proxy_list.json");
        }
    } else {
        panic!("Accounts file not detected, please provice accounts.json");
    }

    let accounts = account_list.unwrap();
    let mut tasks = vec![];

    for key in accounts.iter() {
        let account = key.clone();

        let task = tokio::spawn(async move {
            operation(&account).await;
        });

        tasks.push(task);
    }

    for task in tasks {
        let _ = task.await;
    }
}

async fn operation(acc: &str) {
    let acc = Arc::new(acc.to_string());

    loop {
        Spinner::log(&acc, "Initializing Wallet...", 1000).await;
        let network = Network::KITEAI;
        let mut evm_service = match EvmService::new(&acc, &network) {
            Ok(service) => service,
            Err(err) => {
                ExceptionHandler::operation_error(&acc, OperationError::from(err)).await;
                continue;
            }
        };
        let api_repository = match ApiRepository::new(&acc) {
            Ok(repository) => repository,
            Err(err) => {
                ExceptionHandler::setup_error(&acc, err).await;
                continue;
            }
        };

        if let Err(error) = evm_service.get_balance().await {
            ExceptionHandler::operation_error(&acc, OperationError::from(error)).await;
            continue;
        }
        api_repository
            .get_user_stats(&evm_service.formatted_address)
            .await;

        if Config::get().use_onchain {
            if SpinnerData::get_or_create(&acc).balance.gas > Decimal::from_i32(0).unwrap() {
                match evm_service.transfer().await {
                    Ok(()) => {}
                    Err(error) => {
                        ExceptionHandler::operation_error(&acc, OperationError::from(error)).await
                    }
                }
            } else {
                ExceptionHandler::operation_error(
                    &acc,
                    OperationError::new(
                        &format!(
                            "You dont have {} to use On Chain Feature",
                            Network::get_rpc_details(&network).symbol
                        )
                        .as_str(),
                    ),
                )
                .await
            }
        }

        let delay = 60000;
        if RustQLite::get_logs_today(&evm_service.formatted_address, "interact")
            .await
            .len()
            <= Config::get().interaction as usize
        {
            api_repository
                .chat_with_professor_agent(&evm_service.formatted_address)
                .await;
            Spinner::log(&acc, "Delaying 1 Min Before Chat Other Agent...", 60000).await;
            api_repository
                .chat_with_sherlock_agent(&evm_service.formatted_address)
                .await;
            Spinner::log(&acc, "Delaying 1 Min Before Chat Other Agent...", 60000).await;
            api_repository
                .chat_with_buddy_agent(&evm_service.formatted_address)
                .await;

            Spinner::log(&acc, "Account Processing Complete...", delay).await;
        } else {
            let delay = 60000 * 60 * 24;
            Spinner::log(&acc, "Account Processing Complete...", delay).await;
        }
    }
}
