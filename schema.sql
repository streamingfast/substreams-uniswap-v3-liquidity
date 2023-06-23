create table pool (
    address text not null constraint pool_pk primary key,
    token0_address text,
    token0_symbol text,
    token0_decimals integer,
    token1_address text,
    token1_symbol text,
    token1_decimals integer
);

create table position (
    id text not null constraint position_pk primary key,
    pool_address text,
    tick_lower_idx integer,
    tick_upper_idx integer
);

create table pool_tick (
    pool_address text,
    block_number integer,
    tick decimal,
    PRIMARY KEY(pool_address, block_number)
);

create table pool_liquidity (
    pool_address text,
    block_number integer,
    liquidity decimal,
    PRIMARY KEY(pool_address, block_number)
);

create table position_liquidity (
    position_id text,
    block_number integer,
    liquidity decimal,
    PRIMARY KEY(position_id, block_number)
);
