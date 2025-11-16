param(
    [ValidateSet("debug", "release")]
    [string]$BuildType = "debug"
)

# Set build flags and paths based on build type
if ($BuildType -eq "release") {
    $cargoArgs = "build --release"
    $buildDir = "target\release"
} else {
    $cargoArgs = "build"
    $buildDir = "target\debug"
}

# Build the workspace
Invoke-Expression "cargo $cargoArgs"

# Check if build succeeded
if ($LASTEXITCODE -eq 0) {
    # Ensure dist_plugins directory exists
    $distDir = "$buildDir\dist_plugins"
    if (!(Test-Path $distDir)) {
        New-Item -ItemType Directory -Path $distDir | Out-Null
    }

    # Remove all files in dist_plugins directory
    Get-ChildItem -Path $distDir -File | Remove-Item -Force

    # Detect all plugin folders under 'plugins'
    $pluginRoot = "plugins"
    $pluginDirs = Get-ChildItem -Path $pluginRoot -Directory | Select-Object -ExpandProperty Name

    foreach ($plugin in $pluginDirs) {
        $dllPath = "$buildDir\$plugin.dll"
        if (Test-Path $dllPath) {
            Copy-Item -Path $dllPath -Destination "$distDir\$plugin.dll" -Force
            Write-Host "$plugin.dll copied to '$distDir' successfully."
        } else {
            Write-Host "DLL not found for plugin '$plugin': $dllPath"
        }
    }
} else {
    Write-Host "Build failed. DLLs not copied."
}