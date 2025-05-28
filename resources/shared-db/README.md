# Klyra Shared Databases

This plugin manages databases that are shared with other services on [Klyra](https://www.klyra.rs).
Your database will be in a cluster shared with other users, but it will not be accessible by other users.

## Usage

Add `klyra-shared-db` to the dependencies for your service. Every type of shareable database is behind the following feature flag and attribute path (`*-rustls` uses rustls for TLS, the default uses native-tls).

| Engine   | Feature flags                  | Attribute path              |
|----------|--------------------------------|-----------------------------|
| Postgres | `postgres` / `postgres-rustls` | klyra_shared_db::Postgres |
| MongoDB  | `mongodb`                      | klyra_shared_db::MongoDb  |

An example using the Rocket framework can be found on [GitHub](https://github.com/klyra-hq/klyra-examples/tree/main/rocket/postgres)

### Postgres

This resource has the following options

| Option    | Type | Description                                                                                                    |
|-----------|------|----------------------------------------------------------------------------------------------------------------|
| local_uri | &str | Don't spin a local docker instance of Postgres, but rather connect to this URI instead for `cargo klyra run` |

### MongoDB

This resource has the following options

| Option    | Type | Description                                                                                                   |
|-----------|------|---------------------------------------------------------------------------------------------------------------|
| local_uri | &str | Don't spin a local docker instance of MongoDB, but rather connect to this URI instead for `cargo klyra run` |
