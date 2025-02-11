.PHONY = default release

default:
	cargo sqlx prepare --workspace
	cargo build

release:
	cargo build --release
