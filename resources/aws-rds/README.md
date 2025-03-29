# Klyra AWS RDS
This plugin provisions databases on AWS RDS using [klyra](https://www.klyra.rs). The following three engines are supported:
- Postgres
- MySql
- MariaDB

## Usage
Add `klyra-aws-rds` to the dependencies for your service. Every engine is behind the following feature flags and attribute paths:

| Engine   | Feature flag | Attribute path            |
|----------|--------------|---------------------------|
| Postgres | postgres     | klyra_aws_rds::Postgres |
| MySql    | mysql        | klyra_aws_rds::MySql    |
| MariaDB  | mariadb      | klyra_aws_rds::MariaDB  |

An example using the Tide framework can be found on [GitHub](https://github.com/klyra-hq/examples/tree/main/tide/postgres)

