// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PoolLiquidities {
    #[prost(message, repeated, tag="1")]
    pub items: ::prost::alloc::vec::Vec<PoolLiquidity>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PoolLiquidity {
    #[prost(string, tag="1")]
    pub address: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub liquidity: ::prost::alloc::string::String,
}
// @@protoc_insertion_point(module)
