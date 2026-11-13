use crate::sol::{self, IERC20, IPoolManager, IUniswapV2Pair, V3Pool};

use alloy::{primitives::Address, rpc::types::Log};
use alloy_sol_types::SolEvent;
use std::collections::{HashMap, HashSet};

pub struct Chunk {
    addrs: HashSet<Address>,
    tombstones: HashSet<Address>,
    id: u32,
}

pub struct PoolEvents {
    map: HashMap<alloy::primitives::FixedBytes<32>, UnifiedPoolEvent>,
}

pub fn generate_pools_events_map() -> HashMap<alloy::primitives::FixedBytes<32>, UnifiedPoolEvent> {
    let mut map = HashMap::new();

    let v2_sync_hash = IUniswapV2Pair::Sync::SIGNATURE_HASH;
    map.insert(v2_sync_hash, UnifiedPoolEvent::V2Mint);

    let v2_mint_hash = IUniswapV2Pair::Mint::SIGNATURE_HASH;
    map.insert(v2_mint_hash, UnifiedPoolEvent::V2Mint);

    let v2_approval_hash = IUniswapV2Pair::Approval::SIGNATURE_HASH;

    map.insert(v2_approval_hash, UnifiedPoolEvent::V2Approval);

    let v2_swap_hash = IUniswapV2Pair::Swap::SIGNATURE_HASH;

    map.insert(v2_swap_hash, UnifiedPoolEvent::V2Swap);

    let v2_burn_hash = sol::IUniswapV2Pair::Burn::SIGNATURE_HASH;

    map.insert(v2_burn_hash, UnifiedPoolEvent::V2Burn);

    let v3_burn_hash = sol::V3Pool::Burn::SIGNATURE_HASH;
    map.insert(v3_burn_hash, UnifiedPoolEvent::V3Burn);

    let v3_collect_hash = sol::V3Pool::Collect::SIGNATURE_HASH;
    map.insert(v3_collect_hash, UnifiedPoolEvent::V3Collect);

    let v3_mint_hash = sol::V3Pool::Mint::SIGNATURE_HASH;
    map.insert(v3_mint_hash, UnifiedPoolEvent::V3Mint);

    let v3_flash_hash = sol::V3Pool::Flash::SIGNATURE_HASH;
    map.insert(v3_flash_hash, UnifiedPoolEvent::V3Flash);

    let v3_swap_hash = sol::V3Pool::Swap::SIGNATURE_HASH;
    map.insert(v3_swap_hash, UnifiedPoolEvent::V3Swap);

    let v4_modify_liquidity = sol::StateView::ModifyLiquidity::SIGNATURE_HASH;
    map.insert(v4_modify_liquidity, UnifiedPoolEvent::V4Modify);

    let v4_swap = sol::IPoolManager::Swap::SIGNATURE_HASH;
    map.insert(v4_swap, UnifiedPoolEvent::V4Swap);

    let v4_init = sol::IPoolManager::ModifyLiquidity::SIGNATURE_HASH;
    map.insert(v4_init, UnifiedPoolEvent::V4Initialize);

    let v4_donate = sol::IPoolManager::Donate::SIGNATURE_HASH;
    map.insert(v4_donate, UnifiedPoolEvent::V4Donate);

    map.into()
}

pub fn generate_pool_events() -> Vec<&'static str> {
    let v2_events = sol::IUniswapV2Pair::IUniswapV2PairEvents::SIGNATURES.clone();
    let v3_events = sol::IUniswapV2Pair::IUniswapV2PairEvents::SIGNATURES.clone();
    let v4_events = sol::StateView::StateViewEvents::SIGNATURES.clone();
    print!("==v4 events: {:?}", &v4_events);
    [v2_events, v3_events, v4_events].concat();
    let v2_events = IUniswapV2Pair::IUniswapV2PairEvents::SIGNATURES;
    let v3_events = V3Pool::V3PoolEvents::SIGNATURES;
    let v4_events = IPoolManager::IPoolManagerEvents::SIGNATURES;
    let erc20_events = IERC20::IERC20Events::SIGNATURES;

    let mut all_events = Vec::new();
    all_events.extend_from_slice(v2_events);
    all_events.extend_from_slice(v3_events);
    all_events.extend_from_slice(v4_events);
    all_events.extend_from_slice(erc20_events);
    all_events
}

#[derive(Debug, Clone)]
pub enum UnifiedPoolEvent {
    // UNISWAP V2
    V2Mint,
    V2Burn,
    V2Swap,
    V2Sync,
    V2Approval,
    V2Transfer,

    // UNISWAP V3
    V3Mint,
    V3Swap,
    V3Collect,
    V3Burn,
    V3Flash,

    // UNISWAP V4 STATEVIEW
    V4Donate,
    V4Initialize,
    V4Modify,
    V4Swap,

    // ERC20
    ERC20Transfer,
    ERC20Approval,
}

impl UnifiedPoolEvent {
    pub fn decode_log(&self, log: &Log) -> Option<UnifiedPoolEventResponse> {
        match self {
            UnifiedPoolEvent::V2Mint => log
                .log_decode::<IUniswapV2Pair::Mint>()
                .ok()
                .map(UnifiedPoolEventResponse::V2Mint),
            UnifiedPoolEvent::V2Burn => log
                .log_decode::<IUniswapV2Pair::Burn>()
                .ok()
                .map(UnifiedPoolEventResponse::V2Burn),
            UnifiedPoolEvent::V2Swap => log
                .log_decode::<IUniswapV2Pair::Swap>()
                .ok()
                .map(UnifiedPoolEventResponse::V2Swap),
            UnifiedPoolEvent::V2Sync => log
                .log_decode::<IUniswapV2Pair::Sync>()
                .ok()
                .map(UnifiedPoolEventResponse::V2Sync),
            UnifiedPoolEvent::V2Approval => log
                .log_decode::<IUniswapV2Pair::Approval>()
                .ok()
                .map(UnifiedPoolEventResponse::V2Approval),
            UnifiedPoolEvent::V2Transfer => log
                .log_decode::<IUniswapV2Pair::Transfer>()
                .ok()
                .map(UnifiedPoolEventResponse::V2Transfer),
            UnifiedPoolEvent::V3Mint => log
                .log_decode::<V3Pool::Mint>()
                .ok()
                .map(UnifiedPoolEventResponse::V3Mint),
            UnifiedPoolEvent::V3Swap => log
                .log_decode::<V3Pool::Swap>()
                .ok()
                .map(UnifiedPoolEventResponse::V3Swap),
            UnifiedPoolEvent::V3Collect => log
                .log_decode::<V3Pool::Collect>()
                .ok()
                .map(UnifiedPoolEventResponse::V3Collect),
            UnifiedPoolEvent::V3Burn => log
                .log_decode::<V3Pool::Burn>()
                .ok()
                .map(UnifiedPoolEventResponse::V3Burn),
            UnifiedPoolEvent::V3Flash => log
                .log_decode::<V3Pool::Flash>()
                .ok()
                .map(UnifiedPoolEventResponse::V3Flash),
            UnifiedPoolEvent::V4Donate => log
                .log_decode::<IPoolManager::Donate>()
                .ok()
                .map(UnifiedPoolEventResponse::V4Donate),
            UnifiedPoolEvent::V4Initialize => log
                .log_decode::<IPoolManager::Initialize>()
                .ok()
                .map(UnifiedPoolEventResponse::V4Initialize),
            UnifiedPoolEvent::V4Modify => log
                .log_decode::<IPoolManager::ModifyLiquidity>()
                .ok()
                .map(UnifiedPoolEventResponse::V4Modify),
            UnifiedPoolEvent::V4Swap => log
                .log_decode::<IPoolManager::Swap>()
                .ok()
                .map(UnifiedPoolEventResponse::V4Swap),
            UnifiedPoolEvent::ERC20Transfer => log
                .log_decode::<IERC20::Transfer>()
                .ok()
                .map(UnifiedPoolEventResponse::ERC20Transfer),
            UnifiedPoolEvent::ERC20Approval => log
                .log_decode::<IERC20::Approval>()
                .ok()
                .map(UnifiedPoolEventResponse::ERC20Approval),
        }
    }
}

#[derive(Debug, Clone)]
pub enum UnifiedPoolEventResponse {
    // UNISWAP V2
    V2Mint(Log<sol::IUniswapV2Pair::Mint>),
    V2Burn(Log<sol::IUniswapV2Pair::Burn>),
    V2Swap(Log<sol::IUniswapV2Pair::Swap>),
    V2Sync(Log<sol::IUniswapV2Pair::Sync>),
    V2Approval(Log<sol::IUniswapV2Pair::Approval>),
    V2Transfer(Log<sol::IUniswapV2Pair::Transfer>),

    // UNISWAP V3
    V3Mint(Log<sol::V3Pool::Mint>),
    V3Swap(Log<sol::V3Pool::Swap>),
    V3Collect(Log<sol::V3Pool::Collect>),
    V3Burn(Log<sol::V3Pool::Burn>),
    V3Flash(Log<sol::V3Pool::Flash>),

    // UNISWAP V4
    V4Donate(Log<IPoolManager::Donate>),
    V4Initialize(Log<IPoolManager::Initialize>),
    V4Modify(Log<IPoolManager::ModifyLiquidity>),
    V4Swap(Log<IPoolManager::Swap>),

    ERC20Transfer(Log<IERC20::Transfer>),
    ERC20Approval(Log<IERC20::Approval>),
}
