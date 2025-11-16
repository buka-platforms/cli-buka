# Build the workspace in debug mode
cargo build

# Check if build succeeded
if ($LASTEXITCODE -eq 0) {
    # Ensure dist_plugins directory exists
    if (!(Test-Path "dist_plugins")) {
        New-Item -ItemType Directory -Path "dist_plugins" | Out-Null
    }
    # Copy the hello.dll from target\debug to dist_plugins folder
    Copy-Item -Path "target\debug\hello.dll" -Destination "dist_plugins\hello.dll" -Force
    Write-Host "hello.dll copied to dist_plugins folder."
} else {
    Write-Host "Build failed. DLL not copied."
}