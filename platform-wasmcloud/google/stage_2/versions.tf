terraform {
  required_providers {
    google = {
      source  = "hashicorp/google"
      version = "6.19.0"
    }

    kubernetes = {
      source  = "hashicorp/kubernetes"
      version = "2.35.1"
    }
  }

  backend "gcs" {
    bucket = "platform-poc-wasmcloud-tofu-state"
    prefix = "dev/stage-2"
  }

  required_version = ">= 1.9.0"
}
