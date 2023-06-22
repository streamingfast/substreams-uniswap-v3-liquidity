mod pb;

use std::str::FromStr;
use pb::nascent::uniswap::v1::{PoolLiquidities, PoolLiquidity};
use substreams::errors::Error;
use substreams::{Hex, key};
use substreams::key::{key_first_segment_in, key_last_segment_in, key_last_segments_in, key_segment_in, operations_ne, segment};
use substreams::pb::substreams::Clock;
use substreams::pb::substreams::store_delta::Operation;
use substreams::scalar::BigInt;
use substreams::store::{DeltaBigInt, Deltas, StoreGet, StoreGetBigInt, StoreGetProto, StoreNew, StoreSet, StoreSetProto};
use crate::pb::nascent::uniswap::v1::{PoolTickState, Token};
use crate::pb::uniswap::types::v1::events::PoolSqrtPrice;
use crate::pb::uniswap::types::v1::{Events, Pool, Pools};
use substreams_database_change::pb::database::DatabaseChanges;
use substreams_database_change::tables::Tables;


#[substreams::handlers::store]
pub fn store_pool_tick_state(
    events: Events,
    output: StoreSetProto<PoolTickState>
) {
    let mut pool_tick_state = PoolTickState::default();

    for event in events.created_positions.into_iter() {
        pool_tick_state.tick_upper_tick_idx = event.tick_upper
    }

    output.set()
    // for pool_addr in interesting_pools {
    //     let mut tick_candidates = vec![];
    //     let tick_lower: i32;
    //     let tick_upper: i32;
    //
    //     for delta in ticks_liquidities_deltas.deltas.iter()
    //         .filter(key_first_segment_in("tick"))
    //         .filter(key_last_segment_in("liquidityGross")) {
    //
    //         if delta.key.contains(&pool_addr) {
    //             tick_candidates.push((segment(&delta.key, 2), &delta.new_value));
    //         }
    //     }
    //
    //     if tick_candidates.len() != 2 {
    //         panic!("there has to be only 2 candidates")
    //     }
    //
    //     if tick_candidates[0] >= tick_candidates[1] {
    //         tick_upper = tick_candidates[0].0.parse::<i32>().unwrap();
    //         tick_lower = tick_candidates[1].0.parse::<i32>().unwrap();
    //     } else {
    //         tick_upper = tick_candidates[1].0.parse::<i32>().unwrap();
    //         tick_lower = tick_candidates[0].0.parse::<i32>().unwrap();
    //     }
    //
    //     pool_liquidity.tick_lower_tick_idx = tick_lower;
    //     pool_liquidity.tick_upper_tick_idx = tick_upper;
    //     pool_liquidity.liquidity = String::new();
    // }

}


#[substreams::handlers::map]
pub fn db_out(clock: Clock, pools: Pools, events: Events) -> Result<DatabaseChanges, Error> {
    let mut tables = Tables::default;

    // Pool
    for pool in pools.pools {
        tables
            .create_row("Pool", &pool.address)
            .set("address", &pool.address)
            .set("token0_address", &pool.token0.as_ref().unwrap().address)
            .set("token0_symbol", &pool.token0.as_ref().unwrap().symbol)
            .set("token0_decimals", &pool.token0.as_ref().unwrap().decimals)
            .set("token1_address", &pool.token1.as_ref().unwrap().address)
            .set("token1_symbol", &pool.token1.as_ref().unwrap().symbol)
            .set("token1_decimals", &pool.token1.as_ref().unwrap().decimals)
    }

    // Created liquidity
    for liquidity in events.created_positions.into_iter() {
        tables
            .create_row("Liquidity", &liquidity.pool)
            .set("pool_address", liquidity.pool)
            .set("block_number", clock.number)
            .set("liquidity", BigInt::zero())
            .set("current_tick", 0)
            .set("tick_lower_tick_idx", liquidity.tick_lower)
            .set("tick_upper_tick_idx", liquidity.tick_upper)
            .set("tick_liquidity", 0)
    }

    // IncreaseLiquidity
    for liquidity in events.increase_liquidity_positions.into_iter() {
        // tables
        //     .update_row("Liquidity", &liquidity.)
    }

    // DecreaseLiquidity


    let mut pool_liquidities = PoolLiquidities::default();
    let mut pool_liquidity = PoolLiquidity::default();

    for pool_liquidity in events.pool_liquidities {
        database_changes.push_change("transfer", "primary-key", 0, Operation::Create)
            .change("key1", ("previous1", "value1"))
            .change("key2", ("previous2", "value2"))
    }

    for delta in liquidity_deltas.deltas.iter()
        .filter(key_first_segment_in("pool"))
        .filter(operations_ne(Operation::Delete)) {
        let pool: Pool = pool_store.must_get_last(&delta.key);
        let token0 = pool.token0.unwrap();
        let token1 = pool.token1.unwrap();
        let tick = sqrt_price_store.must_get_last(&delta.key).tick;
        let pool_tick_state = pool_tick_state_store.must_get_last(pool.address);

        pool_liquidity = PoolLiquidity {
            address: segment(&delta.key, 1).to_string(),
            liquidity: delta.new_value.to_string(),
            current_tick: tick,
            token0: Some(Token {
                address: token0.address,
                symbol: token0.symbol,
                decimals: token0.decimals,
            }),
            token1: Some(Token {
                address: token1.address,
                symbol: token1.symbol,
                decimals: token1.decimals,
            }),
            tick_state: Some(pool_tick_state),
        };

    }

    pool_liquidities.items.push(pool_liquidity);

    Ok(tables.to_database_changes())
}
