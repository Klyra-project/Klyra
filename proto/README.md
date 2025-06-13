# klyra-proto

This crate contains the protofiles used to generate code for the gRPC APIs.
Having the generated files commited means users don't need protoc installed to install `cargo-klyra`.

If you make changes to the protofiles, run `cargo make proto` to generate new files.
