## klyra-next runtime-wrapper

Load and run an .so library that implements `klyra_service::Service`. 

To load and run, pass the path to the .so file to load as an argument to the klyra-next binary:

```bash
cargo run -- -f "src/libhello_world.so"
```
