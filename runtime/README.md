## How to run

```bash
$ make wasm
$ DISCORD_TOKEN=xxx cargo run
```

In another terminal:

``` bash
grpcurl -plaintext -import-path ../proto -proto runtime.proto -d '{"service_name": "Tonic", "path": "runtime/bot.wasm"}' localhost:8000 runtime.Runtime/load
grpcurl -plaintext -import-path ../proto -proto runtime.proto -d '{"service_name": "Tonic"}' localhost:8000 runtime.Runtime/start
```
## klyra-legacy

Load and run an .so library that implements `klyra_service::Service`. 

To test, first start a provisioner from the root directory using:

```bash
docker-compose -f docker-compose.rendered.yml up provisioner
```

Then in another shell, start the runtime using the clap CLI:

```bash
cargo run -- --legacy --provisioner-address http://localhost:8000
```

Or directly (this is the path hardcoded in `deployer::start`):
```bash
# first, make sure the klyra-runtime binary is built
cargo build
# then
/home/<path to klyra repo>/target/debug/klyra-runtime --legacy --provisioner-address http://localhost:8000
```

Pass the path to `deployer::start`
Then in another shell, load a `.so` file and start it up:

``` bash
grpcurl -plaintext -import-path ../proto -proto runtime.proto -d '{"service_name": "Tonic", "path": "examples/rocket/hello-world/target/debug/libhello_world.so"}' localhost:8000 runtime.Runtime/load
grpcurl -plaintext -import-path ../proto -proto runtime.proto -d '{"service_name": "Tonic"}' localhost:8000 runtime.Runtime/start
```

## Running the tests
```bash
$ cd ..; make test
```
