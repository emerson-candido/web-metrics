export COMMIT_HASH := $(shell git rev-parse --short=8 HEAD)
export PROJECT_NAME := "ecandido"
export APP_NAME := "web-metrics"
export APP_VERSION := $(shell awk -F' = ' '/^version =/{gsub(/"/, "", $$2); print $$2}' Cargo.toml)

app-version:
	@echo ${APP_VERSION}

build:
	clear && cargo build

check:
	clear && cargo clippy --tests -- -Dwarnings

docker-image: release
	docker image build -t ${PROJECT_NAME}/${APP_NAME}:${COMMIT_HASH} .

docker-image-push: docker-image docker-login
	docker image push ${PROJECT_NAME}/${APP_NAME}:${COMMIT_HASH}

docker-login:
	@echo ${DOCKER_HUB_PAT} | docker login --username ${PROJECT_NAME} --password-stdin

docker-release: docker-image
	docker image tag ${PROJECT_NAME}/${APP_NAME}:${COMMIT_HASH} ${PROJECT_NAME}/${APP_NAME}:${APP_VERSION}
	docker image tag ${PROJECT_NAME}/${APP_NAME}:${APP_VERSION} ${PROJECT_NAME}/${APP_NAME}:latest

docker-release-push: docker-release docker-login
	docker image push ${PROJECT_NAME}/${APP_NAME}:${APP_VERSION}
	docker image push ${PROJECT_NAME}/${APP_NAME}:latest

release:
	clear && cargo build --release --target=x86_64-unknown-linux-musl

run:
	clear && cargo run

test:
	clear && cargo test
