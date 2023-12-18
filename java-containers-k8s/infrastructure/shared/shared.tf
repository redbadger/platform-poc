locals {
  project_id         = "platform-poc-407113"
  region             = "europe-west2"
  kubernetes_context = "gke_platform-poc-407113_europe-west2_platform-poc-407113-cluster"
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