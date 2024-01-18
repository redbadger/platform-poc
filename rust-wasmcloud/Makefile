.PHONY: clean clean-wit-deps

default: products-http

WIT_FILES = $(shell find wit -name '*.wit')
DEPS_TOML = $(shell find . -name 'deps.toml')

wit-deps: wit/sentinel

wit/sentinel: $(WIT_FILES) $(DEPS_TOML)
	for svc in products products-http platform-wasmcloud; do \
		(cd $$svc && wit-deps) \
	done
	echo "keep track of wit changes for make" > wit/sentinel

products: products/target/wasm32-wasi/release/products.wasm
platform-wasmcloud: platform-wasmcloud/target/wasm32-wasi/release/platform_wasmcloud.wasm

products-http: products-http/build/products_http_s.wasm

products-http/build/products_http_s.wasm: products-http/build/products_http.wasm
	cd products-http && wash build --sign-only

# COMPOSE
products-http/build/products_http.wasm: \
products-http/target/wasm32-wasi/release/products_http.wasm \
products/target/wasm32-wasi/release/products.wasm products-http/wasm-compose.yml
	cd products-http \
	&& mkdir -p build \
	&& wasm-tools compose -c wasm-compose.yml -o build/products_http.wasm target/wasm32-wasi/release/products_http.wasm

PRODUCTS_HTTP_SOURCES = $(shell find products-http -name '*.wit' -o -name '*.rs')
products-http/target/wasm32-wasi/release/products_http.wasm: \
wit/sentinel \
products-http/Cargo.toml \
$(PRODUCTS_HTTP_SOURCES)
	cd products-http && cargo component build --release

PRODUCTS_SOURCES = $(shell find products -name '*.wit' -o -name '*.rs')
products/target/wasm32-wasi/release/products.wasm: \
wit/sentinel \
products/Cargo.toml \
$(PRODUCTS_SOURCES)
	cd products && cargo component build --release

PLATFORM_WC_SOURCES = $(shell find platform-wasmcloud -name '*.wit' -o -name '*.rs')
platform-wasmcloud/target/wasm32-wasi/release/platform_wasmcloud.wasm: \
wit/sentinel \
platform-wasmcloud/Cargo.toml \
$(PLATFORM_WC_SOURCES)
	cd platform-wasmcloud && cargo component build --release

clean: clean-wit-deps
	cd products && cargo clean
	cd products-http && cargo clean && rm -rf build
	cd platform-wasmcloud && cargo clean

clean-wit-deps:
	rm -rf products/wit/deps
	rm -rf products-http/wit/deps
	rm -rf platform-wasmcloud/wit/deps
	rm -f wit/sentinel
