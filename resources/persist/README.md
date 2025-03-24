# Klyra Persist
This plugin allows persisting struct that implement `serde::Serialize` and loading them again using `serde::Deserialize`.

## Usage
Add `klyra-persist` to the dependencies for your service. You can get this resource using the `klyra-persist::Persist` attribute to get a `PersistInstance`. Object can now be saved using `PersistInstance.save()` and loaded again using `PersistInstance.load()`.

An example using the Rocket framework can be found on [GitHub](https://github.com/klyra-hq/klyra/tree/main/examples/rocket/persist)

