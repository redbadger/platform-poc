TARGET_DIRS = common/ data-init/ http-controller/ inventory-service/ orders-service/ products-service/ notification-service/

build-all:
	@for dir in $(TARGET_DIRS); do \
		echo "Building $$dir"; \
		if [ $$dir = "common/" ]; then \
			pushd $$dir && cargo build && popd; \
		else \
			pushd $$dir && wash build && popd; \
		fi \
	done