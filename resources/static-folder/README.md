# Klyra Static Folder
This plugin allows services to get the path to a static folder at runtime

## Usage
Add `klyra-static-folder` to the dependencies for your service. This resource can be using by the `klyra_static_folder::StaticFolder` attribute to get a `PathBuf` with the location of the static folder.

An example using the Axum framework can be found on [GitHub](https://github.com/klyra-hq/examples/tree/main/axum/websocket)
