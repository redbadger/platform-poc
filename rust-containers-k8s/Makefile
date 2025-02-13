.PHONY = default release

default:
	cargo sqlx prepare --workspace
	cargo build

release:
	cargo build --release

docker:
	docker build -t rust-containers-k8s .
