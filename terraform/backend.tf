terraform {
  backend "s3" {
    bucket = "unveil-terraform-state"
    key    = "unveil.tfstate"
    region = "eu-west-2"
  }

  required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = "~> 4.0"
    }
    cloudinit = {
      source  = "hashicorp/cloudinit"
      version = "~> 2.0"
    }
    random = {
      source  = "hashicorp/random"
      version = "~> 3.0"
    }
  }

  required_version = ">= 0.14.9"
}

provider "aws" {
  region = "eu-west-2"
}

module "klyra" {
  source = "./modules/klyra"

  api_fqdn             = "api.klyra.rs"
  db_fqdn              = "db.klyra.rs"
  proxy_fqdn           = "klyraapp.rs"
  postgres_password    = var.postgres_password
  mongodb_password     = var.mongodb_password
  klyra_admin_secret = var.klyra_admin_secret
  instance_type        = "c6g.4xlarge"
}
