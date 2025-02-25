#!/usr/bin/env bash

if [ -z $PROXY_FQDN ]
then
    echo "The variable 'PROXY_FQDN' is missing"
    exit 1
fi

export PG_VERSION=${PG_VERSION:-11}

export PG_CLUSTER_NAME=${PG_CLUSTER_NAME:-klyra}

export PG_DATA=${PG_DATA:-/var/lib/postgresql/$PG_VERSION/$PG_CLUSTER_NAME}

export PG_PORT=${PG_PORT:-5432}

export PG_PASSWORD=${PG_PASSWORD:-postgres}

if [[ "$(pg_lsclusters -h | wc -l)" -ne "1" ]]; then
    set -e
    pg_createcluster -d $PG_DATA $PG_VERSION $PG_CLUSTER_NAME

    conftool() {
        pg_conftool $PG_VERSION $PG_CLUSTER_NAME set $1 $2
    }
    conftool listen_addresses \"*\"
    conftool port $PG_PORT
    conftool log_statement all

    pg_ctlcluster $PG_VERSION $PG_CLUSTER_NAME start
    su postgres -c "psql -c \"ALTER USER postgres PASSWORD '${PG_PASSWORD}'\""
    pg_ctlcluster $PG_VERSION $PG_CLUSTER_NAME stop
    set +e
fi

export PG_LOG=$(pg_lsclusters -h | cut -d' ' -f7)

export PG_HOST=localhost

export PG_URI=postgres://postgres:${PG_PASSWORD}@localhost:${PG_PORT}/postgres

export CRATES_PATH=${CRATES_PATH:-/var/lib/klyra/crates}

mkdir -p $CRATES_PATH

export PROXY_PORT=${PROXY_PORT:-8000}

export API_PORT=${API_PORT:-8001}

if [[ ! -z "${klyra_USERS_TOML}" && ! -s "${klyra_USERS_TOML}" ]]
then
    if [[ -z "${klyra_INITIAL_KEY}" ]]
    then
        echo "\$klyra_INITIAL_KEY is not set to create initial user's key"
        exit 1
    fi

    echo "Creating a first user with key '${klyra_INITIAL_KEY}' at '${klyra_USERS_TOML}'"
    mkdir -p $(dirname "${klyra_USERS_TOML}")
    echo -e "[$klyra_INITIAL_KEY]\nname = 'first-user'\nprojects = []" > "${klyra_USERS_TOML}"
fi

exec supervisord -n -c /usr/share/supervisord/supervisord.conf
