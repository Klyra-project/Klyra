# Klyra Service Info

This plugin allows applications to obtain certain information about their runtime environment.

## Usage

Add `klyra-service-info` to the dependencies for your service.

You can get this resource using the `klyra_service_info::KlyraServiceInfo` attribute to get a `ServiceInfo`. This struct will contain information such as the Klyra service name.

```rust
#[klyra_runtime::main]
async fn app(
    #[klyra_service_info::KlyraServiceInfo] service_info: klyra_service_info::ServiceInfo,
) -> __ { ... }
```

#### Example projects that use `klyra-service-info`

| Framework | Link                                                                                       |
| --------- | ------------------------------------------------------------------------------------------ |
| Axum      | [axum example](https://github.com/klyra-hq/klyra-examples/tree/main/axum/service-info) |
