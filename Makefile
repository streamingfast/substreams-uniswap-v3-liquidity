ENDPOINT ?= mainnet.eth.streamingfast.io:443
START_BLOCK ?= 12369621
STOP_BLOCK ?= +500
ROOT_DIR ?= $(shell dirname $(realpath $(firstword $(MAKEFILE_LIST))))
KV_DSN ?= badger3://$(ROOT_DIR)/kv.db

.PHONY: build
build:
	cargo build --target wasm32-unknown-unknown --release

.PHONY: protogen
protogen:
	substreams protogen ./substreams.yaml --exclude-paths="sf/substreams,google,uniswap/v1/"

.PHONY: pack
pack:
	substreams pack

.PHONY: stream
stream: build
	substreams run -e $(ENDPOINT) substreams.yaml map_liquidity -s $(START_BLOCK) -t $(STOP_BLOCK)
