provider "kubernetes" {
  config_path    = "~/.kube/config"
  config_context = module.shared_vars.kubernetes_context
}

module "shared_vars" {
  source = "../shared"
}

variable "pg_user" {
  description = "Username for Postgres Cloud SQL database"
  default     = "commerce"
}

variable "pg_password" {
  description = "password for Postgres Cloud SQL database"
  default     = "commerce"
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
    name = "kubernetes-service-account"
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
    inventory_service_database_url = "postgresql://${var.pg_user}:${var.pg_password}@127.0.0.1/inventory-service"
    order_service_database_url     = "postgresql://${var.pg_user}:${var.pg_password}@127.0.0.1/order-service"
  }
}
