#!/usr/bin/env fish

set SCRIPT_DIR (dirname (realpath (status -f)))
set COMPONENT_DIR (realpath $SCRIPT_DIR/../wasm-components)

cd $COMPONENT_DIR

for component in data-init inventory-service orders-service products-service http-controller notification-service
    pushd $component
    wash build
    popd
end
