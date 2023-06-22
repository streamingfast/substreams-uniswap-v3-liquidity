mod pb;

use pb::nascent::uniswap::v1::{PoolLiquidities, PoolLiquidity};
use substreams::errors::Error;
use substreams::key::{key_first_segment_in, operations_ne, segment};
use substreams::pb::substreams::store_delta::Operation;
use substreams::store::{DeltaBigInt, Deltas};

#[substreams::handlers::map]
pub fn map_liquidity(
    liquidity_deltas: Deltas<DeltaBigInt>, /* store_pool_liquitidities deltas */
) -> Result<PoolLiquidities, Error> {
    Ok(PoolLiquidities {
        items: liquidity_deltas
            .deltas
            .iter()
            .filter(key_first_segment_in("pool"))
            .filter(operations_ne(Operation::Delete))
            .map(|delta| PoolLiquidity {
                address: segment(&delta.key, 1).to_string(),
                liquidity: delta.new_value.to_string(),
            })
            .collect(),
    })
}
