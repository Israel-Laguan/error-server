NAME=error-server
VERSION=$(shell git rev-parse HEAD)
RUSTV=stable
SEMVER_VERSION=$(shell grep version Cargo.toml | awk -F"\"" '{print $$2}' | head -n 1)
REPO=error-server
SHELL := /bin/bash

has_secrets:
	@[[ $$POSTGRES_DB ]] || (echo "source env.sh first"; exit 2)

no_postgres:
	@[ -z "$$(docker ps -q -f ancestor="postgres:9.6")" ] || (echo "db running"; exit 2)
has_postgres:
	@[ -n "$$(docker ps -q -f ancestor="postgres:9.6")" ] || (echo "db not running"; exit 2)

db: has_secrets	no_postgres
	@echo "Starting postgres container"
	docker run --rm -d \
		-p "5432:5432" \
		--expose 5432 \
		-e POSTGRES_DB="$$POSTGRES_DB" \
		-e POSTGRES_PASSWORD="$$POSTGRES_PASSWORD" \
		-e POSTGRES_USER="$$POSTGRES_USER" \
		-it postgres:9.6

stop:
	@docker ps -aq | xargs -r docker rm -f
	@pkill error-server || true

setup:
	cargo install thruster-cli
	rustup override set $$(cat .rustup)

compose: has_secrets
	docker-compose up -d db
	@echo "Waiting for postgres"
	if hash psql 2> /dev/null; then \
		until [[ $$RETRIES -ge 10 ]] || PGPASSWORD=$${POSTGRES_PASSWORD} psql -h $${POSTGRES_DB_URL} -U $$POSTGRES_USER -d $${POSTGRES_DB} -c "select 1" > /dev/null ; do \
			echo "$$((RETRIES++))"; \
			sleep 1; \
		done \
	else \
	  sleep 10 ;\
	fi
	docker-compose up -d web
	docker-compose logs web

run: has_secrets has_postgres
	cargo run

compile:
	docker run --rm \
		-v cargo-cache:/root/.cargo \
		-v $$PWD:/volume \
		-w /volume \
		-it clux/muslrust:nightly \
		cargo build --release
	sudo chown $$USER:$$USER -R target
	strip target/x86_64-unknown-linux-musl/release/error-server
	mv target/x86_64-unknown-linux-musl/release/error-server .

build:
	@echo "Reusing built binary in current directory from make compile"
	@ls -lah ./error-server
	docker build -t $(REPO)/$(NAME):$(VERSION) .

tag-latest: build
	docker tag $(REPO)/$(NAME):$(VERSION) $(REPO)/$(NAME):latest
	docker push $(REPO)/$(NAME):latest

tag-semver: build
	if curl -sSL https://registry.hub.docker.com/v1/repositories/$(REPO)/$(NAME)/tags | jq -r ".[].name" | grep -q $(SEMVER_VERSION); then \
		echo "Tag $(SEMVER_VERSION) already exists - not publishing" ; \
	else \
		docker tag $(REPO)/$(NAME):$(VERSION) $(REPO)/$(NAME):$(SEMVER_VERSION) ; \
		docker push $(REPO)/$(NAME):$(SEMVER_VERSION) ; \
	fi

#
#  Various Lint tools
#

install-fmt:
	rustup component add rustfmt --toolchain $(RUSTV)

check-fmt:	install-fmt
	cargo +$(RUSTV) fmt -- --check

install-clippy:
	rustup component add clippy --toolchain $(RUSTV)

check-clippy:	install-clippy
	cargo +$(RUSTV) clippy --all --all-targets --all-features --tests -- -D warnings -A clippy::upper_case_acronyms

build-all-test:
	cargo build --tests --all-features

run-all-unit-test:
	cargo test --all-features
