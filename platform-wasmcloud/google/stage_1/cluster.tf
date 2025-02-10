resource "google_container_cluster" "primary" {
  name                = "${var.project_id}-cluster"
  location            = var.region
  deletion_protection = false

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
    machine_type    = "e2-standard-2"
    service_account = google_service_account.workload-identity-user-sa.email
  }
}

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

resource "google_project_iam_member" "artifact-registry-reader-role" {
  project = var.project_id
  role    = "roles/artifactregistry.reader"
  member  = "serviceAccount:${google_service_account.workload-identity-user-sa.email}"
}

output "node_pool_service_account" {
  value = google_service_account.workload-identity-user-sa.email
}
