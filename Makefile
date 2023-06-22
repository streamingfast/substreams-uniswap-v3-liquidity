ENDPOINT ?= mainnet.eth.streamingfast.io:443
START_BLOCK ?= 12369621
STOP_BLOCK ?= +500

# substreams-sink-postgres
POSTGRESQL_DSN ?= psql://dev-node:insecure-change-me-in-prod@localhost:5432/dev-node?sslmode=disable

.PHONY: build
build:
	cargo build --target wasm32-unknown-unknown --release

.PHONY: protogen
protogen:
	substreams protogen ./substreams.yaml --exclude-paths="sf/substreams,google,uniswap/v1/"

.PHONY: package
package:
	substreams pack

.PHONY: stream
stream: build
	substreams run -e $(ENDPOINT) substreams.yaml map_liquidity -s $(START_BLOCK) -t $(STOP_BLOCK)

.PHONE: sink_postgres
sink_postgres: package
	substreams-sink-postgres setup --ignore-duplicate-table-errors "$(POSTGRESQL_DSN)" schema.sql
