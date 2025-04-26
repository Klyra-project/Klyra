# Klyra Static Folder

This plugin allows services to get the path to a static folder at runtime.

## Usage

Add `klyra-static-folder` to the dependencies for your service. 
This resource will be provided by adding the `klyra_static_folder::StaticFolder` attribute to `main`.  

It returns  a `PathBuf` which holds the location of the static folder.

The folder obtained will be consistent between deployments, but will not be in the same folder as the executable.  This has implications when using some frameworks such as [Rocket](https://github.com/SergioBenitez/rocket) because it becomes necessary to override the default location when using Rocket's dynamic templates or static file serving features.

#### Example projects that use `klyra-static-folder`

| Framework | Link                                                                                                        |
|-----------|-------------------------------------------------------------------------------------------------------------|
| Axum      | [axum websocket example](https://github.com/klyra-hq/examples/tree/main/axum/websocket)                   |
| Rocket    | [rocket dynamic template example](https://github.com/klyra-hq/examples/tree/main/rocket/dyn_template_hbs) |


``` rust
#[klyra_runtime::main]
async fn app(
    #[klyra_static_folder::StaticFolder] static_folder: PathBuf,
) -> __ { ... }
```

### Parameters

| Parameter | Type | Default  | Description                                                        |
|-----------|------|----------|--------------------------------------------------------------------|
| folder    | str  | `static` | The relative path, from the crate root, to the directory containing static files to deploy |

### Example: Using the public folder instead

Since this plugin defaults to the `static` folder, the arguments can be used to use the `public` folder instead.

``` rust
#[klyra_runtime::main]
async fn app(
    #[klyra_static_folder::StaticFolder(folder = "public")] public_folder: PathBuf,
) -> __ { ... }
```
