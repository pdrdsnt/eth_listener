use std::{collections::VecDeque, iter::chain};

use crate::{
    events::UnifiedPoolEventResponse,
    listener::{self, create_wsprovider},
};
use alloy::primitives::{B256, map::HashMap};
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender, unbounded_channel};

const MAX_DEDUP_ENTRIES: usize = 50_000;

pub struct ListenersManager {
    chain_id: u64,
    events_tx: UnboundedSender<UnifiedPoolEventResponse>,
    events_rx: UnboundedReceiver<UnifiedPoolEventResponse>,
    providers: HashMap<u8, String>,
    dedup: HashMap<(B256, u64), u8>,
    dedup_order: VecDeque<(B256, u64)>,
}

impl ListenersManager {
    pub fn new(chain_id: u64) -> Self {
        let (tx, rx) = unbounded_channel::<UnifiedPoolEventResponse>();
        Self {
            chain_id,
            events_tx: tx,
            events_rx: rx,
            providers: HashMap::default(),
            dedup: HashMap::default(),
            dedup_order: VecDeque::with_capacity(MAX_DEDUP_ENTRIES),
        }
    }

    pub async fn handle_listeners(&mut self, upstream: UnboundedSender<UnifiedPoolEventResponse>) {
        while let Some(event) = self.events_rx.recv().await {
            let Some(key) = Self::extract_log_key(&event) else {
                continue;
            };

            if !self.filter_log(key) {
                continue;
            }

            if upstream.send(event).is_err() {
                break;
            }
        }
    }

    pub async fn add_listener(&mut self, url: String) {
        let Ok(next_id) = u8::try_from(self.providers.len()) else {
            return;
        };

        if let Ok(wsprovider) = create_wsprovider(url.clone()).await {
            self.providers.insert(next_id, url.clone());
            let listener = listener::Listener::new(url, self.chain_id, wsprovider);
            let tx = self.events_tx.clone();
            tokio::spawn(async move { listener.work(tx).await });
        }
    }

    pub fn filter_log(&mut self, key: (B256, u64)) -> bool {
        let total_providers = self.providers.len() as u8;
        if total_providers <= 1 {
            return true;
        }

        //se todos os provedores deram o mesmo log, nos ja podemos remove-lo
        //dedup hashmap monitora quantos logs iguais recebemos
        if let Some(count) = self.dedup.get_mut(&key) {
            *count += 1;

            //println!("duplicated log reveived : {:?}", &key);

            if *count >= total_providers {
                self.dedup.remove(&key);
            }
            return false;
        }

        if self.dedup_order.len() >= MAX_DEDUP_ENTRIES {
            if let Some(oldest) = self.dedup_order.pop_front() {
                self.dedup.remove(&oldest);
            }
        }

        //println!("fist log reveived : {:?}", &key);

        self.dedup.insert(key, 1);
        self.dedup_order.push_back(key);
        true
    }

    fn extract_log_key(event: &UnifiedPoolEventResponse) -> Option<(B256, u64)> {
        let (tx_hash, log_index) = match event {
            UnifiedPoolEventResponse::V2Mint(log) => (log.transaction_hash, log.log_index),
            UnifiedPoolEventResponse::V2Burn(log) => (log.transaction_hash, log.log_index),
            UnifiedPoolEventResponse::V2Swap(log) => (log.transaction_hash, log.log_index),
            UnifiedPoolEventResponse::V2Sync(log) => (log.transaction_hash, log.log_index),
            UnifiedPoolEventResponse::V2Approval(log) => (log.transaction_hash, log.log_index),
            UnifiedPoolEventResponse::V2Transfer(log) => (log.transaction_hash, log.log_index),
            UnifiedPoolEventResponse::V3Mint(log) => (log.transaction_hash, log.log_index),
            UnifiedPoolEventResponse::V3Swap(log) => (log.transaction_hash, log.log_index),
            UnifiedPoolEventResponse::V3Collect(log) => (log.transaction_hash, log.log_index),
            UnifiedPoolEventResponse::V3Burn(log) => (log.transaction_hash, log.log_index),
            UnifiedPoolEventResponse::V3Flash(log) => (log.transaction_hash, log.log_index),
            UnifiedPoolEventResponse::V4Donate(log) => (log.transaction_hash, log.log_index),
            UnifiedPoolEventResponse::V4Initialize(log) => (log.transaction_hash, log.log_index),
            UnifiedPoolEventResponse::V4Modify(log) => (log.transaction_hash, log.log_index),
            UnifiedPoolEventResponse::V4Swap(log) => (log.transaction_hash, log.log_index),
            UnifiedPoolEventResponse::ERC20Transfer(log) => (log.transaction_hash, log.log_index),
            UnifiedPoolEventResponse::ERC20Approval(log) => (log.transaction_hash, log.log_index),
        };

        Some((tx_hash?, log_index?))
    }
}

pub async fn create_base_manager() -> ListenersManager {
    let base_provider_url = "wss://base-rpc.publicnode.com";
    let other_base_provider_url = "wss://base.drpc.org";

    let base_id = 8453;
    let mut chain_manager = ListenersManager::new(base_id);
    chain_manager
        .add_listener(base_provider_url.to_string())
        .await;

    chain_manager
        .add_listener(other_base_provider_url.to_string())
        .await;

    chain_manager
}
