.PHONY: clean

default: products-http

products: products/target/wasm32-wasi/release/products.wasm

products-http: products-http/build/products_http_s.wasm

products-http/build/products_http_s.wasm: products-http/build/products_http.wasm
	cd products-http && wash build --sign-only

products-http/build/products_http.wasm: \
products-http/target/wasm32-wasi/release/products_http.wasm \
products/target/wasm32-wasi/release/products.wasm products-http/wasm-compose.yml
	cd products-http \
	&& mkdir -p build \
	&& wasm-tools compose -c wasm-compose.yml -o build/products_http.wasm target/wasm32-wasi/release/products_http.wasm

PRODUCTS_HTTP_RS_SOURCES := $(shell find products-http -name '*.rs' -print)
PRODUCTS_HTTP_WITS := $(shell find products-http -name '*.wit' -print)
products-http/target/wasm32-wasi/release/products_http.wasm: products-http/Cargo.toml $(PRODUCTS_HTTP_RS_SOURCES) $(PRODUCTS_HTTP_WITS)
	cd products-http && cargo component build --release

PRODUCTS_RS_SOURCES := $(shell find products -name '*.rs' -print)
PRODUCTS_WITS := $(shell find products -name '*.wit' -print)
products/target/wasm32-wasi/release/products.wasm: products/Cargo.toml $(PRODUCTS_RS_SOURCES) $(PRODUCTS_WITS)
	cd products && cargo component build --release

clean:
	cd products && cargo clean
	cd products-http && cargo clean && rm -rf build
