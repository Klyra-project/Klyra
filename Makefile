COMMIT_SHA?=$(shell git rev-parse --short HEAD)

ifeq ($(PUSH),true)
BUILDX_OP=--push
else
BUILDX_OP=--load
endif

ifdef PLATFORMS
PLATFORM_FLAGS=--platform $(PLATFORMS)
endif

BUILDX_FLAGS=$(BUILDX_OP) $(PLATFORM_FLAGS)

# The Rust version used by our containers
# Can be updated to the latest stable
RUSTUP_TOOLCHAIN=1.76.0

TAG?=$(shell git describe --tags --abbrev=0)
AUTH_TAG?=$(TAG)
DEPLOYER_TAG?=$(TAG)
GATEWAY_TAG?=$(TAG)
LOGGER_TAG?=$(TAG)
PROVISIONER_TAG?=$(TAG)
RESOURCE_RECORDER_TAG?=$(TAG)

DOCKER_BUILD?=docker buildx build
ifeq ($(CI),true)
DOCKER_BUILD+= --progress plain
endif

DOCKER_COMPOSE=$(shell which docker-compose)
ifeq ($(DOCKER_COMPOSE),)
DOCKER_COMPOSE=docker compose
endif

DOCKER_SOCK?=/var/run/docker.sock

POSTGRES_PASSWORD?=postgres
MONGO_INITDB_ROOT_USERNAME?=mongodb
MONGO_INITDB_ROOT_PASSWORD?=password
STRIPE_SECRET_KEY?=""
AUTH_JWTSIGNING_PRIVATE_KEY?=""

DD_ENV=$(klyra_ENV)
ifeq ($(klyra_ENV),production)
DOCKER_COMPOSE_FILES=docker-compose.yml
STACK=klyra-prod
APPS_FQDN=klyraapp.rs
DB_FQDN=db.klyra.rs
CONTAINER_REGISTRY=public.ecr.aws/klyra
# make sure we only ever go to production with `--tls=enable`
USE_TLS=enable
CARGO_PROFILE=release
RUST_LOG?=klyra=debug,info
else
DOCKER_COMPOSE_FILES=docker-compose.yml docker-compose.dev.yml
STACK?=klyra-dev
APPS_FQDN=unstable.klyraapp.rs
DB_FQDN=db.unstable.klyra.rs
CONTAINER_REGISTRY=public.ecr.aws/klyra-dev
USE_TLS?=disable
# default for local run
CARGO_PROFILE?=debug
RUST_LOG?=klyra=debug,info
DEV_SUFFIX=-dev
DEPLOYS_API_KEY?=gateway4deployes
GATEWAY_ADMIN_KEY?=dh9z58jttoes3qvt

# this should use the same version as our prod RDS database
CONTROL_DB_POSTGRES_TAG?=15
CONTROL_DB_POSTGRES_PASSWORD?=postgres
CONTROL_DB_POSTGRES_URI?=postgres://postgres:${CONTROL_DB_POSTGRES_PASSWORD}@control-db:5432/postgres

# this should use the same version as our prod RDS logger database
LOGGER_POSTGRES_TAG?=15
LOGGER_POSTGRES_PASSWORD?=postgres
LOGGER_POSTGRES_URI?=postgres://postgres:${LOGGER_POSTGRES_PASSWORD}@logger-postgres:5432/postgres
endif

ifeq ($(CI),true)
# default for staging
CARGO_PROFILE=release
endif

POSTGRES_EXTRA_PATH?=./extras/postgres
POSTGRES_TAG?=14

OTEL_EXTRA_PATH?=./extras/otel
OTEL_TAG?=0.90.1

ifeq ($(klyra_DETACH), disable)
klyra_DETACH=
else
klyra_DETACH=--detach
endif

DOCKER_COMPOSE_ENV=\
	STACK=$(STACK)\
	AUTH_TAG=$(AUTH_TAG)\
	DEPLOYER_TAG=$(DEPLOYER_TAG)\
	GATEWAY_TAG=$(GATEWAY_TAG)\
	LOGGER_TAG=$(LOGGER_TAG)\
	PROVISIONER_TAG=$(PROVISIONER_TAG)\
	RESOURCE_RECORDER_TAG=$(RESOURCE_RECORDER_TAG)\
	POSTGRES_TAG=${POSTGRES_TAG}\
	CONTROL_DB_POSTGRES_TAG=${CONTROL_DB_POSTGRES_TAG}\
	CONTROL_DB_POSTGRES_PASSWORD=${CONTROL_DB_POSTGRES_PASSWORD}\
	CONTROL_DB_POSTGRES_URI=${CONTROL_DB_POSTGRES_URI}\
	LOGGER_POSTGRES_TAG=${LOGGER_POSTGRES_TAG}\
	LOGGER_POSTGRES_PASSWORD=${LOGGER_POSTGRES_PASSWORD}\
	LOGGER_POSTGRES_URI=${LOGGER_POSTGRES_URI}\
	OTEL_TAG=${OTEL_TAG}\
	APPS_FQDN=$(APPS_FQDN)\
	DB_FQDN=$(DB_FQDN)\
	POSTGRES_PASSWORD=$(POSTGRES_PASSWORD)\
	RUST_LOG=$(RUST_LOG)\
	DEPLOYS_API_KEY=$(DEPLOYS_API_KEY)\
	CONTAINER_REGISTRY=$(CONTAINER_REGISTRY)\
	MONGO_INITDB_ROOT_USERNAME=$(MONGO_INITDB_ROOT_USERNAME)\
	MONGO_INITDB_ROOT_PASSWORD=$(MONGO_INITDB_ROOT_PASSWORD)\
	STRIPE_SECRET_KEY=$(STRIPE_SECRET_KEY)\
	AUTH_JWTSIGNING_PRIVATE_KEY=$(AUTH_JWTSIGNING_PRIVATE_KEY)\
	GATEWAY_ADMIN_KEY=$(GATEWAY_ADMIN_KEY)\
	DD_ENV=$(DD_ENV)\
	USE_TLS=$(USE_TLS)\
	COMPOSE_PROFILES=$(COMPOSE_PROFILES)\
	DOCKER_SOCK=$(DOCKER_SOCK)\
	klyra_ENV=$(klyra_ENV)\
	klyra_SERVICE_VERSION=$(klyra_SERVICE_VERSION)

.PHONY: clean deep-clean images the-klyra-images klyra-% postgres otel deploy test docker-compose.rendered.yml up down

clean:
	rm .klyra-*
	rm docker-compose.rendered.yml

deep-clean:
	find . -type d \( -name target -or -name .klyra-executables -or -name node_modules \) | xargs rm -rf

images: the-klyra-images postgres otel

the-klyra-images: klyra-auth klyra-deployer klyra-gateway klyra-logger klyra-provisioner klyra-resource-recorder

klyra-%:
	$(DOCKER_BUILD) \
		--target $(@)$(DEV_SUFFIX) \
		--build-arg folder=$(*) \
		--build-arg crate=$(@) \
		--build-arg prepare_args=$(PREPARE_ARGS) \
		--build-arg klyra_ENV=$(klyra_ENV) \
		--build-arg klyra_SERVICE_VERSION=$(klyra_SERVICE_VERSION) \
		--build-arg RUSTUP_TOOLCHAIN=$(RUSTUP_TOOLCHAIN) \
		--build-arg CARGO_PROFILE=$(CARGO_PROFILE) \
		--tag $(CONTAINER_REGISTRY)/$(*):$(COMMIT_SHA) \
		--tag $(CONTAINER_REGISTRY)/$(*):$(TAG) \
		--tag $(CONTAINER_REGISTRY)/$(*):latest \
		$(BUILDX_FLAGS) \
		-f Containerfile \
		.

postgres:
	$(DOCKER_BUILD) \
		--build-arg POSTGRES_TAG=$(POSTGRES_TAG) \
		--tag $(CONTAINER_REGISTRY)/postgres:$(POSTGRES_TAG) \
		$(BUILDX_FLAGS) \
		-f $(POSTGRES_EXTRA_PATH)/Containerfile \
		$(POSTGRES_EXTRA_PATH)

otel:
	$(DOCKER_BUILD) \
		--build-arg OTEL_TAG=$(OTEL_TAG) \
		--tag $(CONTAINER_REGISTRY)/otel:$(OTEL_TAG) \
		$(BUILDX_FLAGS) \
		-f $(OTEL_EXTRA_PATH)/Containerfile \
		$(OTEL_EXTRA_PATH)

deploy: docker-compose.yml
	$(DOCKER_COMPOSE_ENV) docker stack deploy -c $< $(STACK)

test:
	POSTGRES_PASSWORD=$(POSTGRES_PASSWORD) \
	APPS_FQDN=$(APPS_FQDN) \
	cargo test --manifest-path=e2e/Cargo.toml $(CARGO_TEST_FLAGS) -- --nocapture

docker-compose.rendered.yml: docker-compose.yml docker-compose.dev.yml
	$(DOCKER_COMPOSE_ENV) $(DOCKER_COMPOSE) -f docker-compose.yml -f docker-compose.dev.yml $(DOCKER_COMPOSE_CONFIG_FLAGS) -p $(STACK) config > $@

# Start the containers locally.
up: $(DOCKER_COMPOSE_FILES)
	$(DOCKER_COMPOSE_ENV) \
	$(DOCKER_COMPOSE) \
	$(addprefix -f ,$(DOCKER_COMPOSE_FILES)) \
	-p $(STACK) \
	up \
	$(klyra_DETACH)

down: $(DOCKER_COMPOSE_FILES)
	$(DOCKER_COMPOSE_ENV) \
	$(DOCKER_COMPOSE) \
	$(addprefix -f ,$(DOCKER_COMPOSE_FILES)) \
	-p $(STACK) \
	down
