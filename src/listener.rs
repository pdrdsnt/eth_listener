use std::time::Duration;

use alloy::{
    providers::{Provider, ProviderBuilder, WsConnect},
    rpc::{client::RpcClient, types::Filter},
    transports::{RpcError, TransportErrorKind},
};
use futures_util::StreamExt;
use tokio::sync::mpsc::UnboundedSender;

use crate::events::{UnifiedPoolEventResponse, generate_pool_events, generate_pools_events_map};

pub struct Listener<P: Provider> {
    url: String,
    chain_id: u64,
    provider: P,
}

impl<P: Provider> Listener<P> {
    pub fn new(url: String, chain_id: u64, provider: P) -> Listener<P> {
        Self {
            url,
            chain_id,
            provider,
        }
    }

    pub async fn work(self, sender: UnboundedSender<UnifiedPoolEventResponse>) {
        let events_map = generate_pools_events_map();
        let filter = Filter::new().events(generate_pool_events());
        let Ok(poller) = self.provider.subscribe_logs(&filter).await else {
            return;
        };

        let mut stream = poller.into_stream();

        while let Some(log) = stream.next().await {
            let Some(topic) = log.topic0() else {
                return;
            };

            if let Some(event) = events_map.get(topic) {
                if let Some(res) = event.decode_log(&log) {
                    sender.send(res);
                }
            }
        }

        println!(
            "log stream finished. chain: {} - provider: {}",
            self.chain_id, self.url
        )
    }
}

pub async fn ws_sub<P: Provider + Clone>(
    provider: P,
    filter: Filter,
) -> Result<alloy::pubsub::Subscription<alloy::rpc::types::Log>, RpcError<TransportErrorKind>> {
    let r = provider.subscribe_logs(&filter).await?;
    return Ok(r);
}

pub async fn new_wsprovider(url: String) -> Result<WsProvider, RpcError<TransportErrorKind>> {
    create_wsprovider(url).await
}

pub async fn create_wsprovider(url: String) -> Result<WsProvider, RpcError<TransportErrorKind>> {
    let wsconnect = WsConnect::new(&url);
    let connection = RpcClient::connect_pubsub(wsconnect);
    match tokio::time::timeout(Duration::from_secs(10), connection).await {
        Ok(Ok(result)) => Ok(ProviderBuilder::new().connect_client(result)),
        Ok(Err(err)) => Err(err),
        Err(_) => Err(RpcError::from(TransportErrorKind::custom_str("timeout "))),
    }
}

pub type WsProvider = alloy::providers::fillers::FillProvider<
    alloy::providers::fillers::JoinFill<
        alloy::providers::Identity,
        alloy::providers::fillers::JoinFill<
            alloy::providers::fillers::GasFiller,
            alloy::providers::fillers::JoinFill<
                alloy::providers::fillers::BlobGasFiller,
                alloy::providers::fillers::JoinFill<
                    alloy::providers::fillers::NonceFiller,
                    alloy::providers::fillers::ChainIdFiller,
                >,
            >,
        >,
    >,
    alloy::providers::RootProvider,
>;
