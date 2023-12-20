provider "google" {
  project = "platform-poc-rust"
}

module "shared_vars" {
  source = "../shared"
}

resource "google_artifact_registry_repository" "registry" {
  location      = module.shared_vars.region
  repository_id = "registry"
  description   = "rust service"
  format        = "DOCKER"

  docker_config {
    immutable_tags = true 
  }
}

