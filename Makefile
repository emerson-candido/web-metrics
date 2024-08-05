export COMMIT_HASH := $(shell git rev-parse --short=8 HEAD)
export PROJECT_NAME := "monitoring"
export APP_NAME := "web_metrics"

build:
	clear && cargo build

check:
	clear && cargo clippy --tests -- -Dwarnings

docker-image:
	docker image build -t ${PROJECT_NAME}/${APP_NAME}:${COMMIT_HASH} .

release:
	clear && cargo build --release --target=x86_64-unknown-linux-musl

run:
	clear && RUST_LOG=info cargo run

test:
	clear && cargo test
