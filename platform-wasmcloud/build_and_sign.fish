#!/usr/bin/env fish

set SCRIPT_DIR (dirname (realpath (status -f)))
set OUTPUT_DIR $SCRIPT_DIR/signed/
set COMPONENT_DIR (realpath $SCRIPT_DIR/../wasm-components/rust)
set INPUT_DIR $COMPONENT_DIR/target/wasm32-wasip2/release/

mkdir -p $OUTPUT_DIR

pushd $COMPONENT_DIR

### cargo
# cargo build --release
# pushd $INPUT_DIR
# for component in data_init inventory_service orders_service products_service http_controller notification_service
#     wash claims sign {$component}.wasm
#     mv {$component}_s.wasm $OUTPUT_DIR
# end
# popd

### wash
for component in data-init inventory-service orders-service products-service http-controller notification-service
    pushd $component
    RUSTUP_TOOLCHAIN=stable wash build
    set COMPONENT (string replace -a '-' _ $component)
    cp build/{$COMPONENT}_s.wasm $OUTPUT_DIR
    popd
end

popd
