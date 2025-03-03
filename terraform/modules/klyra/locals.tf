locals {
  data_dir     = "/opt/klyra"
  docker_image = "public.ecr.aws/klyra/backend"
}

resource "random_string" "initial_key" {
  length  = 16
  special = false
  lower   = true
  number  = true
  upper   = true
}
