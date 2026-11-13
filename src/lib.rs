use futures_util::{FutureExt, StreamExt};
use tokio::sync::mpsc::unbounded_channel;

use crate::{
    events::UnifiedPoolEventResponse,
    listener::{Listener, create_wsprovider},
    listeners_manager::{ListenersManager, create_base_manager},
};

mod events;
mod listener;
mod listeners_manager;
mod sol;

#[tokio::test]
pub async fn it_works() {
    let mut base = create_base_manager().await;
    let (tx, mut rx) = unbounded_channel();
    let _ = tokio::spawn(async move {
        base.handle_listeners(tx.clone()).await;
    });

    while let Some(received) = rx.recv().await {
        match received {
            UnifiedPoolEventResponse::V2Mint(log) => {
                println!("event received {:?}", log.data())
            }
            UnifiedPoolEventResponse::V2Burn(log) => {
                println!("event received {:?}", log.data())
            }
            UnifiedPoolEventResponse::V2Swap(log) => {
                println!("event received {:?}", log.data())
            }
            UnifiedPoolEventResponse::V2Sync(log) => {
                println!("event received {:?}", log.data())
            }
            UnifiedPoolEventResponse::V2Approval(log) => {
                println!("event received {:?}", log.data())
            }
            UnifiedPoolEventResponse::V2Transfer(log) => {
                println!("event received {:?}", log.data())
            }
            UnifiedPoolEventResponse::V3Mint(log) => {
                println!("event received {:?}", log.data())
            }
            UnifiedPoolEventResponse::V3Swap(log) => {
                println!("event received {:?}", log.data())
            }
            UnifiedPoolEventResponse::V3Collect(log) => {
                println!("event received {:?}", log.data())
            }
            UnifiedPoolEventResponse::V3Burn(log) => {
                println!("event received {:?}", log.data())
            }
            UnifiedPoolEventResponse::V3Flash(log) => {
                println!("event received {:?}", log.data())
            }
            UnifiedPoolEventResponse::V4Donate(log) => {
                println!("event received {:?}", log.data())
            }
            UnifiedPoolEventResponse::V4Initialize(log) => {
                println!("event received {:?}", log.data())
            }
            UnifiedPoolEventResponse::V4Modify(log) => {
                println!("event received {:?}", log.data())
            }
            UnifiedPoolEventResponse::V4Swap(log) => {
                println!("event received {:?}", log.data())
            }
            UnifiedPoolEventResponse::ERC20Transfer(log) => {
                println!("event received {:?}", log.data())
            }
            UnifiedPoolEventResponse::ERC20Approval(log) => {
                println!("event received {:?}", log.data())
            }
        }
    }
}
