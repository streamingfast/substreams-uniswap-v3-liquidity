ENDPOINT ?= mainnet.eth.streamingfast.io:443
START_BLOCK ?= 12370000
STOP_BLOCK ?= +500

# substreams-sink-postgres
POSTGRESQL_DSN ?= psql://dev-node:insecure-change-me-in-prod@localhost:5432/dev-node?sslmode=disable

.PHONY: build
build:
	cargo build --target wasm32-unknown-unknown --release

.PHONY: protogen
protogen:
	substreams protogen ./substreams.yaml --exclude-paths="sf/substreams,google,database.proto"

.PHONY: package
package: build
	substreams pack

.PHONY: gui
gui: build
	substreams gui -e $(ENDPOINT) substreams.yaml db_out -s $(START_BLOCK) -t $(STOP_BLOCK)

.PHONY: stream
stream: build
	substreams run -e $(ENDPOINT) substreams.yaml db_out -s $(START_BLOCK) -t $(STOP_BLOCK)

.PHONE: sink_postgres
sink_postgres: build
	substreams-sink-postgres setup --ignore-duplicate-table-errors "$(POSTGRESQL_DSN)" schema.sql
	substreams-sink-postgres run $(POSTGRESQL_DSN) $(ENDPOINT) "substreams.yaml" db_out