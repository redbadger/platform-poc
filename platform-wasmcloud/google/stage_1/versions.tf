terraform {
  required_providers {
    google = {
      source  = "hashicorp/google"
      version = "6.19.0"
    }
  }

  backend "gcs" {
    bucket = "platform-poc-wasmcloud-tofu-state"
    prefix = "dev"
  }

  required_version = ">= 1.9.0"
}
