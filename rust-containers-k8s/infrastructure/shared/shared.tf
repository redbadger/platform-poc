locals {
  project_id         = "platform-poc-rust"
  region             = "europe-west2"
  kubernetes_context = "gke_platform-poc-rust_europe-west2_platform-poc-rust-cluster"
}

output "project_id" {
  value = local.project_id
}

output "region" {
  value = local.region
}

output "kubernetes_context" {
  value = local.kubernetes_context
}
