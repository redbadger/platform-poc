output "kubernetes_context" {
  value = "gke_${var.project_id}_${var.region}_${google_container_cluster.primary.name}"
}

output "workload-identity-user-sa" {
  value = google_service_account.workload-identity-user-sa.email
}
