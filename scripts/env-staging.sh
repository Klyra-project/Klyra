export klyra_API="https://api.staging.klyra.dev"
unset klyra_API_KEY
export PS1="(klyra: Staging) $(echo $PS1 | sed -e "s/(klyra: .*) //")"
