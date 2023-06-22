create table pool_liquidity
(
    address     text not null constraint address_pk primary key,
    liquidity   decimal,
    current_tick integer,
    token0_address text,
    token0_symbol text,
    token0_decimals integer,
    token1_address text,
    token1_symbol text,
    token1_decimals integer,
    tick_lower_tick_idx integer,
    tick_upper_tick_idx integer,
    tick_liquidity decimal,
);

create table cursors
(
    id         text not null constraint cursor_pk primary key,
    cursor     text,
    block_num  bigint,
    block_id   text
);
