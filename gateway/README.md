# klyra-gateway

## Tests

To run the tests for gateway, follow the steps in [contributing](../CONTRIBUTING.md) to set up your local environment. Then, from the root of the repository, run:

```bash
klyra_TESTS_RUNTIME_IMAGE=public.ecr.aws/klyra-dev/deployer:latest klyra_TESTS_NETWORK=klyra-dev_user-net cargo test --package klyra-gateway --all-features -- --nocapture
```
