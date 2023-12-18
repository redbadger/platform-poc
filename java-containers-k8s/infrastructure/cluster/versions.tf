terraform {
  required_providers {
    google = {
      source  = "hashicorp/google"
      version = "= 5.8.0"
    }
  }

  required_version = ">= 1.6.5"
}