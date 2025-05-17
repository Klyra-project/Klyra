# Klyra Metadata

This plugin allows applications to obtain certain information about their runtime environment.

## Usage

Add `klyra-metadata` to the dependencies for your service.

You can get this resource using the `klyra_metadata::KlyraMetadata` attribute to get a `Metadata`. This struct will contain information such as the Klyra service name.

```rust
#[klyra_runtime::main]
async fn app(
    #[klyra_metadata::KlyraMetadata] metadata: klyra_metadata::Metadata,
) -> __ { ... }
```

#### Example projects that use `klyra-metadata`

| Framework | Link                                                                                   |
| --------- | -------------------------------------------------------------------------------------- |
| Axum      | [axum example](https://github.com/klyra-hq/klyra-examples/tree/main/axum/metadata) |
