create table pool (
    address text not null constraint pool_pk primary key,
    token0_address text,
    token0_symbol text,
    token0_decimals integer,
    token1_address text,
    token1_symbol text,
    token1_decimals integer
);

create table liquidity (
    pool_address text,
    block_number integer,
    liquidity   decimal,
    current_tick integer,
    tick_lower_tick_idx integer,
    tick_upper_tick_idx integer,
    tick_liquidity decimal,
    PRIMARY KEY(pool_address, block_number)
);

create table cursors (
    id         text not null constraint cursor_pk primary key,
    cursor     text,
    block_num  bigint,
    block_id   text
);
