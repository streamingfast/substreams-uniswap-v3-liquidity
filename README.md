### Uniswap V3 Liquidity Extraction

This show cases composing on top of [Substreams Uniswap V3](https://github.com/streamingfast/substreams-uniswap-v3) to extract liquidity pool for the chain.

```
make stream START_BLOCK=17000000
```

Any gRPC aware language can stream using Substreams RPC v2 to any format, the emitted Protobuf type is [./proto/nascent/uniswap/v1/uniswap.proto](./proto/nascent/uniswap/v1/uniswap.proto).
