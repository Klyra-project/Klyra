export klyra_API="https://api.klyra.dev"
unset klyra_API_KEY
export PS1="(klyra: Production) $(echo $PS1 | sed -e "s/(klyra: .*) //")"
