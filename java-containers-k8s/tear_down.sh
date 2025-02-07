set -e

pushd infrastructure

echo "tearing down infrastructure..."

pushd kubernetes
tofu init
tofu destroy
popd

pushd cluster
tofu init
tofu destroy
popd

pushd storage
tofu init
tofu destroy
popd

echo "infrastructure tear down finished!"

popd
