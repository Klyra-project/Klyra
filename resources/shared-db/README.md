# Klyra Shared Databases
This plugin manages databases that are shared with other services on [klyra](https://www.klyra.rs).

## Usage
Add `klyra-shared-db` to the dependencies for your service. Every type of shareable database is behind the following feature flag and attribute path

| Engine   | Feature flag | Attribute path              |
|----------|--------------|-----------------------------|
| Postgres | postgres     | klyra_shared_db::Postgres |
| MongoDB  | mongodb      | klyra_shared_db::MongoDb  |

An example using the Rocket framework can be found on [GitHub](https://github.com/klyra-hq/examples/tree/main/rocket/postgres)

