use crate::{
    model::{
        exception::operation_error::OperationError, spinner_data::SpinnerData,
        user_balance::UserBalance,
    },
    utils::{
        network::{Network, RPC},
        spinner::Spinner,
    },
};
use ethers::{
    core::k256::ecdsa::SigningKey,
    prelude::*,
    providers::Provider,
    signers::coins_bip39::English,
    types::transaction::eip2718::TypedTransaction,
    utils::{format_ether, parse_ether, parse_units},
};
use log::info;
use rust_decimal::Decimal;
use std::{str::FromStr, sync::Arc};

pub struct EvmService {
    pub acc: Arc<String>,
    pub address: H160,
    pub rpc: RPC,
    pub formatted_address: String,
    pub client: Arc<SignerMiddleware<Arc<Provider<Http>>, LocalWallet>>,
    pub balance: UserBalance,
}

impl EvmService {
    pub fn new(
        acc: &Arc<String>,
        network: Network,
    ) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let rpc = network.get_rpc_details();
        let provider: Arc<Provider<Http>> = Arc::new(Provider::<Http>::try_from(rpc.rpc_url)?);

        let wallet: Wallet<SigningKey> = if acc.split_whitespace().count() > 3 {
            let wallet: Wallet<SigningKey> = MnemonicBuilder::<English>::default()
                .phrase(&*acc.to_string())
                .build()?;

            wallet
        } else {
            LocalWallet::from_str(&acc)?
        };

        let formatted_address = format!("0x{:x}", wallet.address());
        SpinnerData::update(&acc, |data| {
            data.address = formatted_address.to_owned();
        });
        let client = Arc::new(SignerMiddleware::new(
            provider.clone(),
            wallet.clone().with_chain_id(rpc.chain_id),
        ));

        Ok(EvmService {
            acc: acc.to_owned(),
            rpc,
            client,
            address: wallet.address(),
            formatted_address,
            balance: UserBalance { gas: Decimal::ZERO },
        })
    }

    pub async fn get_balance(&mut self) -> Result<(), OperationError> {
        Spinner::log(&self.acc, "Getting Wallet Balance...", 1000).await;
        match self.client.get_balance(self.address, None).await {
            Ok(gas_balance) => {
                self.balance = UserBalance {
                    gas: format_ether(gas_balance)
                        .parse::<Decimal>()
                        .unwrap_or(Decimal::ZERO),
                };
                SpinnerData::update(&self.acc, |data| data.balance = self.balance.to_owned());
                Spinner::log(&self.acc, "Successfully Get Wallet Balance...", 1000).await;
            }
            Err(err) => {
                return Err(OperationError {
                    message: format!("Error getting balance: {}", err),
                });
            }
        }
        Ok(())
    }

    pub async fn transfer(&self) -> Result<(), OperationError> {
        Spinner::log(&self.acc, "Trying to Self Transfer...", 1000).await;
        let tx = self
            .build_tx_body::<Bytes>(None, parse_ether(0)?, self.formatted_address.to_owned())
            .await?;

        match self.execute_tx(tx).await {
            Ok(_tx_result) => {
                Spinner::log(&self.acc, "Transfer Successful...", 1000).await;
                Ok(())
            }
            Err(err) => {
                return Err(OperationError {
                    message: format!("Error During Self Transfer: {}", err),
                });
            }
        }
    }

    pub async fn get_optimal_nonce(&self) -> Result<U256, OperationError> {
        let latest_nonce = self
            .client
            .get_transaction_count(self.address, Some(BlockId::Number(BlockNumber::Latest)))
            .await
            .map_err(|e| OperationError::new(&format!("Failed to fetch latest nonce: {}", e)))?;

        let pending_nonce = self
            .client
            .get_transaction_count(self.address, Some(BlockId::Number(BlockNumber::Pending)))
            .await
            .map_err(|e| OperationError::new(&format!("Failed to fetch pending nonce: {}", e)))?;

        Ok(std::cmp::max(latest_nonce, pending_nonce))
    }

    pub async fn build_tx_body<T: Into<Bytes>>(
        &self,
        data: Option<T>,
        amount: U256,
        to: String,
    ) -> Result<TypedTransaction, OperationError> {
        let from: H160 = self.formatted_address.parse().unwrap();
        let to: H160 = to.parse().unwrap();
        let amount_in_wei = parse_ether(amount)? as U256;

        let gas_price_in_gwei: U256 = U256::from(parse_units("5.0", "gwei").unwrap());
        let optimal_nonce = self.get_optimal_nonce().await?;
        let data: Bytes = match data {
            Some(d) => d.into(),
            None => {
                info!("No data provided. Using default empty Bytes.");
                Bytes::new()
            }
        };

        info!("Building Transaction");
        info!("From      : {}", from);
        info!("To        : {}", from);
        info!("Value     : {}", amount_in_wei);
        info!("Data      : {}", data);
        info!("Nonce     : {}", optimal_nonce);
        info!("Gas Price : {}", gas_price_in_gwei);

        let tx: TypedTransaction = if data.is_empty() {
            TransactionRequest::new()
                .from(from)
                .to(to)
                .value(amount_in_wei)
                .nonce(optimal_nonce)
                .gas_price(gas_price_in_gwei)
                .into()
        } else {
            TransactionRequest::new()
                .from(from)
                .to(to)
                .value(amount_in_wei)
                .data(data)
                .nonce(optimal_nonce)
                .gas_price(gas_price_in_gwei)
                .into()
        };

        info!("Transaction Data : {:?}", tx);
        Ok(tx)
    }

    pub async fn execute_tx(
        &self,
        tx: TypedTransaction,
    ) -> Result<Option<TransactionReceipt>, Box<dyn std::error::Error + Send + Sync>> {
        Spinner::log(&self.acc, "Executing Tx ...", 1000).await;
        info!("Transaction : {:?}", tx);
        let block_id: BlockId = (self.client.get_block_number().await? - 1).into();
        let transaction = self.client.send_transaction(tx, Some(block_id)).await?;
        info!("Pending Transaction : {:?}", transaction);
        let tx_hash = transaction.tx_hash();
        Spinner::log(
            &self.acc,
            format!("Transaction Executed, Hash : {}", tx_hash).as_str(),
            2000,
        )
        .await;

        info!("Transaction Hash: {}", format!("{:#x}", tx_hash));

        Spinner::log(
            &self.acc,
            "Transaction Executed, Waiting For Block Confirmation...",
            1000,
        )
        .await;
        Ok(match transaction.await {
            Ok(Some(receipt)) => {
                Spinner::log(
                    &self.acc,
                    format!(
                        "Transaction Confirmed : {}tx/{:?}",
                        self.rpc.explorer, receipt.transaction_hash
                    )
                    .as_str(),
                    5000,
                )
                .await;
                return Ok(Some(receipt));
            }
            Ok(None) => {
                Spinner::log(
                    &self.acc,
                    "Transaction was submitted but not found in a block yet.",
                    5000,
                )
                .await;
                None
            }
            Err(err) => {
                return Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("Error During Executing Tx : {}", err),
                )));
            }
        })
    }
}
