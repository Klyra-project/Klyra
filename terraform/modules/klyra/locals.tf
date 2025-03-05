locals {
  data_dir                 = "/opt/klyra"
  docker_backend_image     = "public.ecr.aws/klyra/backend"
  docker_provisioner_image = "public.ecr.aws/klyra/provisioner"
}

resource "random_string" "initial_key" {
  length  = 16
  special = false
  lower   = true
  number  = true
  upper   = true
}
