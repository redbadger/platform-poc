resource "google_project_iam_member" "workload_identity-role" {
  project = var.project_id
  role    = "roles/iam.workloadIdentityUser"
  member  = "serviceAccount:${var.project_id}.svc.id.goog[default/${kubernetes_service_account.ksa.metadata[0].name}]"
}

resource "kubernetes_service_account" "ksa" {
  metadata {
    name = "kubernetes-service-account"
    annotations = {
      "iam.gke.io/gcp-service-account" = data.terraform_remote_state.stage_1.outputs.workload-identity-user-sa
    }
  }
}

resource "kubernetes_secret" "db_secrets" {
  metadata {
    name = "postgres-db-secrets"
  }

  data = {
    username = var.pg_user
    password = var.pg_password
    database = var.pg_database
  }
}
