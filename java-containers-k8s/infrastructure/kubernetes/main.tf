provider "kubernetes" {
  config_path    = "~/.kube/config"
  config_context = module.shared_vars.kubernetes_context
}

module "shared_vars" {
  source = "../shared"
}

variable "pg_user" {
  description = "Username for Postgres Cloud SQL database"
}

variable "pg_password" {
  description = "password for Postgres Cloud SQL database"
}

variable "pg_database" {
  description = "Postgres Cloud SQL database name"
}

data "terraform_remote_state" "workload-identity-user-sa" {
  backend = "local"

  config = {
    path = "../cluster/terraform.tfstate"
  }
}

resource "google_project_iam_member" "workload_identity-role" {
  project = module.shared_vars.project_id
  role    = "roles/iam.workloadIdentityUser"
  member  = "serviceAccount:${module.shared_vars.project_id}.svc.id.goog[default/${kubernetes_service_account.ksa.metadata[0].name}]"
}

resource "kubernetes_service_account" "ksa" {
  metadata {
    name        = "kubernetes-service-account"
    annotations = {
      "iam.gke.io/gcp-service-account" = data.terraform_remote_state.workload-identity-user-sa.outputs.node_pool_service_account
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