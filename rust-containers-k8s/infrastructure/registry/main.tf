provider "google" {
  project = "platform-poc-rust"
}

module "shared_vars" {
  source = "../shared"
}

resource "google_artifact_registry_repository" "inventory-service" {
  location      = module.shared_vars.region
  repository_id = "inventory-service"
  description   = "rust inventory service"
  format        = "DOCKER"

  docker_config {
    immutable_tags = true
  }
}

resource "google_artifact_registry_repository" "notification-service" {
  location      = module.shared_vars.region
  repository_id = "notification-service"
  description   = "rust notification service"
  format        = "DOCKER"

  docker_config {
    immutable_tags = true
  }
}

resource "google_artifact_registry_repository" "order-service" {
  location      = module.shared_vars.region
  repository_id = "order-service"
  description   = "rust order service"
  format        = "DOCKER"

  docker_config {
    immutable_tags = true
  }
}

resource "google_artifact_registry_repository" "product-service" {
  location      = module.shared_vars.region
  repository_id = "product-service"
  description   = "rust product service"
  format        = "DOCKER"

  docker_config {
    immutable_tags = true
  }
}
