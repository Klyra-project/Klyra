# Would actually like to error on all errors, but `Enable-ExperimentalFeature`
# does not work for this version of Windows
# https://github.com/PowerShell/PowerShell/issues/3415#issuecomment-1354457563
Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

# Add cargo to PATH
$env:Path += [IO.Path]::PathSeparator + "$env:USERPROFILE/.cargo/bin"

# Install the WASM target
rustup target add wasm32-wasi

# Install wasm runtime from checked out code
cargo install klyra-runtime --path runtime --bin klyra-next --features next

# Start locally
$job = Start-Job -Name "local-run" -ScriptBlock { cd examples/next/hello-world; cargo klyra run }
Start-Sleep -Seconds 70

echo "Testing local wasm endpoint"
$output=curl http://localhost:8000 | Select-Object -ExpandProperty Content
if ( $output -ne "Hello, world!")
{
    echo "Did not expect output: $output"
    exit 1
}

Stop-Job $job

exit 0
