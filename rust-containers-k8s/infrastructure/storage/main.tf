provider "google" {
  project = "platform-poc-rust"
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
  name             = "${module.shared_vars.project_id}-pg"
  region           = module.shared_vars.region
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
  project                 = module.shared_vars.project_id
  name                    = "(default)"
  location_id             = module.shared_vars.region
  type                    = "DATASTORE_MODE"
  delete_protection_state = "DELETE_PROTECTION_DISABLED"
  deletion_policy         = "DELETE"
}
