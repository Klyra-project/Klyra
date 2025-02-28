# AWS klyra module
This module contains all the resources needed to deploy klyra on AWS. The basic architecture is to create:
1. A single EC2 instance to run klyra and PostgresDB
1. Two Route53 zones - one for the klyra api; another to reach user services hosted by klyra (called the proxy)
1. Three Load Balancers - one for the api, proxy, and PostgresDB respectively

## Usage guide
The following terraform can be used as a starting point for using this module:

```tf
module "klyra" {
  source = "github.com/klyra-hq/klyra/terraform/modules/klyra"

  api_fqdn             = "api.test.klyra.rs"
  proxy_fqdn           = "test.klyraapp.rs"
  postgres_password    = "password"
  klyra_admin_secret = "12345"
}

output "api_name_servers" {
  value = module.klyra.api_name_servers
}

output "user_name_servers" {
  value = module.klyra.user_name_servers
}

output "initial_user_key" {
  value       = module.klyra.initial_user_key
  description = "Key given to the initial klyra user"
}
```

The klyra api will be reachable at `api_fqdn` while hosted services will be subdomains of `proxy_fqdn`. The `postgres_password` sets the root password for Postgres and `klyra_admin_secret` will be the secret needed to add more user keys to klyra by an admin user. Klyra does create the first user key though. This key is stored in the `initial_user_key` output variable.

Just running `terraform apply` for the first time will fail since SSl certificates will be created for the api and proxy domains which will be verified. This verification will fail since it uses DNS that will be missing on first setup. So for first setups rather run the following:

``` sh
terraform apply --target module.klyra.aws_route53_zone.user --target module.klyra.aws_route53_zone.api
```

This command will create just the DNS zones needed for the api and proxy. Now use the `api_name_servers` and `user_name_servers` outputs from this module to manually add NS records for `api_fqdn` and `proxy_fqdn` in your DNS provider respectively.

Once these records have propagated, a `terraform apply` command will succeed.

