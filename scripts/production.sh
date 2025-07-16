# Use this to make cargo-klyra target the production env.
# Useful when running cargo-klyra in debug mode, since that targets the local stack by default.
#
# Usage:
#     source scripts/production.sh

export klyra_API="https://api.klyra.rs"
unset klyra_API_KEY
unset klyra_BETA
export PS1="(klyra: production) $(echo $PS1 | sed -e "s/(klyra: .*) //")"
