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
        set -l COMPONENT (string replace -a '-' _ $component)
        mv build/{$COMPONENT}_s.wasm $OUTPUT_DIR
    popd
end

### optimize, requires `wasm-opt` from `binaryen`
# e.g.`cargo binstall wasm-opt`
pushd $OUTPUT_DIR
for component in data_init inventory_service orders_service products_service http_controller notification_service
    wasm-opt -Oz --enable-bulk-memory -o {$component}_s_opt.wasm {$component}_s.wasm
    mv {$component}_s_opt.wasm {$component}_s.wasm
end
popd

popd
