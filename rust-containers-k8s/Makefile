.PHONY = default prepare release docker

default: prepare
	cargo build

prepare:
	cargo sqlx prepare --workspace

release:
	cargo build --release

docker:
	docker build -t rust-containers-k8s .
