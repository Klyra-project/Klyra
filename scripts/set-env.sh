# Usage: source scripts/set-env.sh [env]

if [ -z "$1" ]; then
    echo "provide an env name";
else
    export klyra_API_ENV="$1"
    unset klyra_API
    unset klyra_API_KEY
    export PS1="(klyra: $klyra_API_ENV) $(echo $PS1 | sed -e "s/(klyra: .*) //")"
fi
