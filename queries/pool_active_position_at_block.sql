with positions as (
    select * from position_liquidity
    left join position on position_id = position.id
    where block_number <= ${BLOCK_NUMBER}
    and position.pool_address = '${POOL_ADDRESS}'
), cte as (
    select
        position_id, block_number, liquidity,
        rank() over (
            partition by position_id
            order by block_number desc
        ) as rank
    from positions
)
select * from cte where rank = 1 and liquidity > 0;