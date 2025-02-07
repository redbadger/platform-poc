# Platform PoC wasmCloud on Google Cloud

## Prerequisites

- [Google Cloud SDK](https://cloud.google.com/sdk/docs/install)
- [OpenTofu](https://opentofu.org/)
- [wash](https://wasmcloud.com/docs/installation/)

## Setup

Login to Google Cloud:

```fish
# for interactive use of gcloud CLI ...
gcloud auth login

# for OpenTofu/Terraform ...
gcloud auth application-default login
```

Install the GKE gcloud auth plugin (so that the provision script can get the cluster credentials):

```fish
gcloud components install gke-gcloud-auth-plugin
```

## Provisioning

```fish
./scripts/provision.fish
```

## Destroying

```fish
./scripts/destroy.fish
```
