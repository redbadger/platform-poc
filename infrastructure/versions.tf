terraform {
  required_providers {
    google = {
      source  = "hashicorp/google"
      version = "= 5.8.0"
    }
    kubernetes = {
      source  = "hashicorp/kubernetes"
      version = ">= 2.0.0"
    }
  }

  required_version = ">= 1.6.5"
}