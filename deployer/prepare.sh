#!/usr/bin/env sh

###############################################################################
# This file is used by our common Containerfile incase the container for this #
# service might need some extra preparation steps for its final image         #
###############################################################################

# Patch crates to be on same versions
mkdir -p $CARGO_HOME; \
echo '[patch.crates-io]
klyra-service = { path = "/usr/src/klyra/service" }
klyra-aws-rds = { path = "/usr/src/klyra/resources/aws-rds" }
klyra-persist = { path = "/usr/src/klyra/resources/persist" }
klyra-shared-db = { path = "/usr/src/klyra/resources/shared-db" }
klyra-secrets = { path = "/usr/src/klyra/resources/secrets" }
klyra-static-folder = { path = "/usr/src/klyra/resources/static-folder" }' > $CARGO_HOME/config.toml

# Prefetch crates.io index
cd /usr/src/klyra/service
cargo fetch

# Make future crates requests to our own mirror
echo '
[source.klyra-crates-io-mirror]
registry = "http://panamax:8080/git/crates.io-index"
[source.crates-io]
replace-with = "klyra-crates-io-mirror"' >> $CARGO_HOME/config.toml
