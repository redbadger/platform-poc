resource "google_artifact_registry_repository" "registry" {
  location      = var.region
  repository_id = "registry"
  description   = "OCI registry for wasmcloud services"
  format        = "DOCKER"

  docker_config {
    immutable_tags = false
  }
}
