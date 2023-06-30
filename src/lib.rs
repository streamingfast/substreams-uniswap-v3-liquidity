mod pb;

use crate::pb::uniswap::types::v1::events::PoolSqrtPrice;
use crate::pb::uniswap::types::v1::{Events, Pools};
use pb::uniswap::types::v1::events::{DecreaseLiquidityPosition, IncreaseLiquidityPosition};
use std::collections::{HashMap, HashSet};
use substreams::errors::Error;
use substreams::key;
use substreams::pb::substreams::Clock;
use substreams::scalar::BigInt;
use substreams::store::{
    DeltaBigInt, DeltaExt, DeltaProto, Deltas, StoreAdd, StoreAddBigInt, StoreNew,
};
use substreams_database_change::pb::database::table_change::PrimaryKey;
use substreams_database_change::pb::database::{CompositePrimaryKey, DatabaseChanges, TableChange};
use substreams_database_change::tables::Tables;

struct PositionChange {
    log_ordinal: u64,
    token_id: String,
    liquidity: BigInt,
}

impl From<IncreaseLiquidityPosition> for PositionChange {
    fn from(event: IncreaseLiquidityPosition) -> Self {
        Self {
            log_ordinal: event.log_ordinal,
            token_id: event.token_id,
            liquidity: BigInt::try_from(event.liquidity).unwrap(),
        }
    }
}

impl From<DecreaseLiquidityPosition> for PositionChange {
    fn from(event: DecreaseLiquidityPosition) -> Self {
        Self {
            log_ordinal: event.log_ordinal,
            token_id: event.token_id,
            // We must negate the liquidity because it is a decrease
            liquidity: BigInt::try_from(event.liquidity).unwrap().neg(),
        }
    }
}

#[substreams::handlers::store]
pub fn store_position_liquidities(
    events: Events, /* map_extract_data_types */
    store: StoreAddBigInt,
) {
    let mut changes = Vec::<PositionChange>::with_capacity(
        events.increase_liquidity_positions.len() + events.decrease_liquidity_positions.len(),
    );
    changes.extend(
        events
            .increase_liquidity_positions
            .into_iter()
            .map(Into::into),
    );
    changes.extend(
        events
            .decrease_liquidity_positions
            .into_iter()
            .map(Into::into),
    );

    changes.sort_by(|a, b| a.log_ordinal.cmp(&b.log_ordinal));

    for change in changes {
        store.add(
            change.log_ordinal,
            format!("position:{}", change.token_id),
            change.liquidity,
        )
    }
}

#[substreams::handlers::map]
pub fn db_out(
    clock: Clock,
    events: Events,       /* map_extract_data_types */
    pools_created: Pools, /* map_pools_created */
    pool_sqrt_price_deltas: Deltas<DeltaProto<PoolSqrtPrice>>, /* store_pool_sqrt_price */
    pool_liquidities_store_deltas: Deltas<DeltaBigInt>, /* store_pool_liquidities */
    position_liquidities_store_deltas: Deltas<DeltaBigInt>, /* store_position_liquidities */
) -> Result<DatabaseChanges, Error> {
    let mut pool_keys: HashSet<String> = HashSet::<String>::new();
    let mut position_keys: HashSet<String> = HashSet::<String>::new();
    let mut tick_keys: HashSet<String> = HashSet::<String>::new();

    let mut tables = Tables::new();

    // Pool
    for pool in pools_created.pools {
        let token0 = pool.token0.unwrap();
        let token1 = pool.token1.unwrap();

        tables
            .create_row("pool", &pool.address)
            .set("token0_address", token0.address)
            .set("token0_symbol", token0.symbol)
            .set("token0_decimals", token0.decimals)
            .set("token1_address", token1.address)
            .set("token1_symbol", token1.symbol)
            .set("token1_decimals", token1.decimals);

        let liquidity_key = format!("{}:{}", &pool.address, clock.number);
        pool_keys.insert(liquidity_key.clone());
        tables
            .create_row("pool_liquidity", liquidity_key)
            .set("liquidity", "0");
    }

    for delta in pool_liquidities_store_deltas.key_first_segment_eq("pool") {
        let liquidity_key = format!("{}:{}", key::segment_at(&delta.key, 1), clock.number);
        let row = if pool_keys.contains(&liquidity_key) {
            tables.update_row("pool_liquidity", liquidity_key)
        } else {
            pool_keys.insert(liquidity_key.clone());
            tables.create_row("pool_liquidity", liquidity_key)
        };

        row.set("liquidity", &delta.new_value);
    }

    for delta in pool_sqrt_price_deltas.key_first_segment_eq("pool") {
        let tick_key = format!("{}:{}", key::segment_at(&delta.key, 1), clock.number);
        let row = if tick_keys.contains(&tick_key) {
            tables.update_row("pool_tick", tick_key)
        } else {
            tick_keys.insert(tick_key.clone());
            tables.create_row("pool_tick", tick_key)
        };

        row.set("tick", &delta.new_value.tick);
    }

    for position in events.created_positions {
        tables
            .create_row("position", &position.token_id)
            .set("pool_address", position.pool)
            .set("tick_lower_idx", position.tick_lower)
            .set("tick_upper_idx", position.tick_upper);

        let liquidity_key: String = format!("{}:{}", &position.token_id, clock.number);
        position_keys.insert(liquidity_key.clone());
        tables
            .create_row("position_liquidity", liquidity_key)
            .set("liquidity", "0");
    }

    for delta in position_liquidities_store_deltas {
        let liquidity_key = format!("{}:{}", key::segment_at_owned(delta.key, 1), clock.number);
        let row = if position_keys.contains(&liquidity_key) {
            tables.update_row("position_liquidity", liquidity_key)
        } else {
            position_keys.insert(liquidity_key.clone());
            tables.create_row("position_liquidity", liquidity_key)
        };

        row.set("liquidity", delta.new_value);
    }

    Ok(fix_composite_keys(tables.to_database_changes()))
}

fn fix_composite_keys(mut changes: DatabaseChanges) -> DatabaseChanges {
    changes.table_changes.iter_mut().for_each(|mut change| {
        let pk = get_pk(change);

        match change.table.as_str() {
            "pool_liquidity" => {
                change.primary_key = Some(PrimaryKey::CompositePk(CompositePrimaryKey {
                    keys: HashMap::from_iter(vec![
                        (
                            "pool_address".to_string(),
                            key::segment_at(pk, 0).to_string(),
                        ),
                        (
                            "block_number".to_string(),
                            key::segment_at(pk, 1).to_string(),
                        ),
                    ]),
                }))
            }
            "pool_tick" => {
                change.primary_key = Some(PrimaryKey::CompositePk(CompositePrimaryKey {
                    keys: HashMap::from_iter(vec![
                        (
                            "pool_address".to_string(),
                            key::segment_at(pk, 0).to_string(),
                        ),
                        (
                            "block_number".to_string(),
                            key::segment_at(pk, 1).to_string(),
                        ),
                    ]),
                }))
            }
            "position_liquidity" => {
                change.primary_key = Some(PrimaryKey::CompositePk(CompositePrimaryKey {
                    keys: HashMap::from_iter(vec![
                        (
                            "position_id".to_string(),
                            key::segment_at(pk, 0).to_string(),
                        ),
                        (
                            "block_number".to_string(),
                            key::segment_at(pk, 1).to_string(),
                        ),
                    ]),
                }))
            }
            _ => (),
        }
    });

    changes
}

fn get_pk(change: &TableChange) -> &String {
    match change.primary_key.as_ref().unwrap() {
        PrimaryKey::Pk(pk) => pk,
        _ => panic!("Expected plain primary key"),
    }
}
