provider "google" {
  project = "platform-poc-rust"
}

module "shared_vars" {
  source = "../shared"
}

resource "google_container_cluster" "primary" {
  name     = "${module.shared_vars.project_id}-cluster"
  location = module.shared_vars.region
  deletion_protection = false

  remove_default_node_pool = true
  initial_node_count       = 1

  workload_identity_config {
    workload_pool = "${module.shared_vars.project_id}.svc.id.goog"
  }
}

resource "google_container_node_pool" "primary_nodes" {
  name       = "primary-node-pool"
  location   = module.shared_vars.region
  cluster    = google_container_cluster.primary.name
  node_count = 1

  node_config {
    machine_type = "e2-standard-2"
    service_account = google_service_account.workload-identity-user-sa.email
  }
}

resource "google_service_account" "workload-identity-user-sa" {
  account_id   = "cloud-sql-client-sa"
  display_name = "Cloud SQL Client Service Account"
  description  = "Service account used for Cloud SQL Auth PRoxy"
}

resource "google_project_iam_member" "sql-client-role" {
  project = module.shared_vars.project_id
  role    = "roles/cloudsql.client"
  member  = "serviceAccount:${google_service_account.workload-identity-user-sa.email}"
}

resource "google_project_iam_member" "datastore-user-role" {
  project = module.shared_vars.project_id
  role    = "roles/datastore.user"
  member  = "serviceAccount:${google_service_account.workload-identity-user-sa.email}"
}

resource "google_project_iam_member" "storage-role" {
  project = module.shared_vars.project_id
  role    = "roles/storage.admin"
  member  = "serviceAccount:${google_service_account.workload-identity-user-sa.email}"
}

output "node_pool_service_account" {
  value = google_service_account.workload-identity-user-sa.email
}
