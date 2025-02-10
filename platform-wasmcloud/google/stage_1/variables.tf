variable "project_id" {
  type        = string
  description = "The project ID"
}

variable "region" {
  type        = string
  description = "The region"
}

variable "pg_user" {
  type        = string
  description = "Username for Postgres Cloud SQL database"
}

variable "pg_password" {
  type        = string
  description = "password for Postgres Cloud SQL database"
}

variable "pg_database" {
  type        = string
  description = "Postgres Cloud SQL database name"
}
