variable "registry" {
  default = "public.ecr.aws/klyra"
}

variable "context" {
  default = "."
}

target "api" {
  dockerfile = "Containerfile"
  context = "${context}"
  tags = ["${registry}/api"]
  args = {
    crate = "klyra-api"
  }
}

target "provisioner" {
  dockerfile = "Containerfile"
  context = "${context}"
  tags = ["${registry}/provisioner"]
  args = {
    crate = "klyra-provisioner"
  }
}
