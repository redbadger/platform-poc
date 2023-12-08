# obtain an .env file and export env variables before running this
provider "google" {
  project = "platform-poc-407113"
}

provider "kubernetes" {
  config_path    = "~/.kube/config"
  config_context = var.kubernetes_context
}

variable "project_id" {
  description = "The Google Cloud project ID."
}

variable "kubernetes_context" {
  description = "K8S context name for the GKE cluster"
}

variable "region" {
  description = "The GCP region for resources."
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


######### Cluster

resource "google_container_cluster" "primary" {
  name     = "${var.project_id}-cluster"
  location = var.region

  remove_default_node_pool = true
  initial_node_count       = 1

  workload_identity_config {
    workload_pool = "${var.project_id}.svc.id.goog"
  }
}

resource "google_container_node_pool" "primary_nodes" {
  name       = "primary-node-pool"
  location   = var.region
  cluster    = google_container_cluster.primary.name
  node_count = 1

  node_config {
    machine_type = "e2-standard-2"
    service_account = google_service_account.workload-identity-user-sa.email
  }
}

#### IAM

resource "google_service_account" "workload-identity-user-sa" {
  account_id   = "cloud-sql-client-sa"
  display_name = "Cloud SQL Client Service Account"
  description  = "Service account used for Cloud SQL Auth PRoxy"
}

resource "google_project_iam_member" "sql-client-role" {
  project = var.project_id
  role    = "roles/cloudsql.client"
  member  = "serviceAccount:${google_service_account.workload-identity-user-sa.email}"
}

resource "google_project_iam_member" "datastore-user-role" {
  project = var.project_id
  role    = "roles/datastore.user"
  member  = "serviceAccount:${google_service_account.workload-identity-user-sa.email}"
}

resource "google_project_iam_member" "workload_identity-role" {
  project = var.project_id
  role    = "roles/iam.workloadIdentityUser"
  member  = "serviceAccount:${var.project_id}.svc.id.goog[default/${kubernetes_service_account.ksa.metadata[0].name}]"
}

resource "kubernetes_service_account" "ksa" {
  metadata {
    name        = "kubernetes-service-account"
    annotations = {
      "iam.gke.io/gcp-service-account" = google_service_account.workload-identity-user-sa.email
    }
  }
}

#### K8s

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

##### Data stores

resource "google_sql_database" "database_orders" {
  name     = "order-service"
  instance = google_sql_database_instance.instance.name
}

resource "google_sql_database" "database_inventory" {
  name     = "inventory-service"
  instance = google_sql_database_instance.instance.name
}

resource "google_sql_user" "user" {
  name     = var.pg_user
  instance = google_sql_database_instance.instance.name
  password = var.pg_password
}

resource "google_sql_database_instance" "instance" {
  name             = "${var.project_id}-pg"
  region           = var.region
  database_version = "POSTGRES_15"
  settings {
    tier = "db-f1-micro"
    database_flags {
      name  = "max_connections"
      value = "50"
    }
  }

  deletion_protection = "false"
}

resource "google_firestore_database" "datastore_database" {
  project                 = var.project_id
  name                    = "(default)"
  location_id             = var.region
  type                    = "DATASTORE_MODE"
  delete_protection_state = "DELETE_PROTECTION_DISABLED"
  deletion_policy         = "DELETE"
}
