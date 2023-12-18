set -e

pushd infrastructure

echo "spinning up infrastructure..."

pushd cluster
terraform init
terraform apply -auto-approve
popd

pushd storage
terraform init
terraform apply -auto-approve
popd

echo "Authenticating with the cluster..."
gcloud container clusters get-credentials platform-poc-rust-cluster --project platform-poc-rust --location europe-west2
echo "done!"

pushd kubernetes
terraform init
terraform apply -auto-approve
popd

echo "infrastructure ready!"

popd

echo "deploying services..."
cd deployment
helm install kafka oci://registry-1.docker.io/bitnamicharts/kafka --set listeners.client.protocol=PLAINTEXT

for service in *; do
  pushd "$service"
  helm install "$service" -f values.yaml .
  popd
done
echo "deployment done!"
