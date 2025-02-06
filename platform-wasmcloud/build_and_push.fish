#!/usr/bin/env fish

set SCRIPT_DIR (dirname (realpath (status -f)))
set COMPONENT_DIR (realpath $SCRIPT_DIR/../wasm-components)

cd $COMPONENT_DIR

for component in data-init inventory-service orders-service products-service http-controller notification-service
    pushd $component
        wash build
        set --local COMPONENT (string replace -a '-' _ $component)
        # TODO: make this better...
        wash push --insecure \
            localhost:5001/v2/platform-poc/{$COMPONENT}:0.1.0 \
            build/{$COMPONENT}_s.wasm
    popd
end
