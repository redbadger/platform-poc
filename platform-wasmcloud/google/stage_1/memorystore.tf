resource "google_memorystore_instance" "redis" {
  instance_id = "${var.project_id}-redis"
  shard_count = 1
  desired_psc_auto_connections {
    network    = google_compute_network.producer_net.id
    project_id = data.google_project.project.project_id
  }
  location                    = var.region
  deletion_protection_enabled = false
  depends_on = [
    google_network_connectivity_service_connection_policy.default
  ]

  lifecycle {
    # we don't store any critical data
    prevent_destroy = false
  }
}

resource "google_network_connectivity_service_connection_policy" "default" {
  name          = "${var.project_id}-redis-policy"
  location      = var.region
  service_class = "gcp-memorystore"
  description   = "redis connection policy"
  network       = google_compute_network.producer_net.id
  psc_config {
    subnetworks = [google_compute_subnetwork.producer_subnet.id]
  }
}

resource "google_compute_subnetwork" "producer_subnet" {
  name          = "my-subnet"
  ip_cidr_range = "10.0.0.248/29"
  region        = var.region
  network       = google_compute_network.producer_net.id
}

resource "google_compute_network" "producer_net" {
  name                    = "my-network"
  auto_create_subnetworks = false
}

data "google_project" "project" {
}
