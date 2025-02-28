variable "postgres_password" {
  type        = string
  description = "Root password for postgres instance"
}

variable "klyra_admin_secret" {
  type        = string
  description = "Secret for the klyra admin user"
}
