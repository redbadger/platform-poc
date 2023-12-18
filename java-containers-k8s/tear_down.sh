set -e

pushd infrastructure

echo "tearing down infrastructure..."

pushd kubernetes
terraform init
terraform destroy -auto-approve
popd

pushd cluster
terraform init
terraform destroy -auto-approve
popd

pushd storage
terraform init
terraform destroy -auto-approve
popd

echo "infrastructure tear down finished!"

popd