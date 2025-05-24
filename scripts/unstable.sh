# Use this to make cargo-klyra target the unstable (staging) env.
#
# Usage:
#     source scripts/unstable.sh

export klyra_API="https://api.unstable.klyra.rs"
unset klyra_API_KEY
export PS1="(klyra: unstable) $PS1"
