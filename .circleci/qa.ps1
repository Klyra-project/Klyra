# Would actually like to error on all errors, but `Enable-ExperimentalFeature`
# does not work for this version of Windows
# https://github.com/PowerShell/PowerShell/issues/3415#issuecomment-1354457563
Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

# Prepare directory
mkdir -p /tmp/qa-windows
cd /tmp/qa-windows

# Add cargo to PATH
$env:Path += [IO.Path]::PathSeparator + "$env:USERPROFILE/.cargo/bin"

# Init app
cargo klyra init --name qa-windows --template axum

# # Start locally
$job = Start-Job -Name "local-run" -ScriptBlock { cd /tmp/qa-windows; cargo klyra run }
Start-Sleep -Seconds 270

echo "Testing local hello endpoint"
$output=curl http://localhost:8000 | Select-Object -ExpandProperty Content
if ( $output -ne "Hello, world!")
{
    echo "Did not expect output: $output"
    exit 1
}

Stop-Job $job

cargo klyra project start

cargo klyra deploy --allow-dirty

echo "Testing remote hello endpoint"
$output=curl https://qa-windows.unstable.klyraapp.rs | Select-Object -ExpandProperty Content
if ( $output -ne "Hello, world!")
{
    echo "Did not expect output: $output"
    exit 1
}

cargo klyra project stop

exit 0
