# Use this to insert an admin API key in your local stack and
# set that key to be used with cargo-klyra requests
#
# Usage:
#     source scripts/local-admin.sh

key="dh9z58jttoes3qvt" # arbitrary test key
export klyra_API_KEY=$key
export klyra_API="http://localhost:8001"
export PS1="(klyra: local admin key) $(echo $PS1 | sed -e "s/(klyra: .*) //")"

docker compose --file docker-compose.rendered.yml --project-name klyra-dev exec auth /usr/local/bin/klyra-auth --db-connection-uri=postgres://postgres:postgres@control-db init-admin --user-id admin --key $key
