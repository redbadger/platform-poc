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
