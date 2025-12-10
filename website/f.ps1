# Friendev Installation Script for Windows
$ErrorActionPreference = 'Stop'

# Colors
$Green = [ConsoleColor]::Green
$Yellow = [ConsoleColor]::Yellow
$Red = [ConsoleColor]::Red
$Reset = [ConsoleColor]::White

function Log-Info($Message) {
    Write-Host "[INFO] $Message" -ForegroundColor $Green
}

function Log-Warn($Message) {
    Write-Host "[WARN] $Message" -ForegroundColor $Yellow
}

function Log-Error($Message) {
    Write-Host "[ERROR] $Message" -ForegroundColor $Red
}

function Get-Architecture {
    $arch = $env:PROCESSOR_ARCHITECTURE
    if ($arch -eq 'AMD64') {
        return 'amd64'
    } elseif ($arch -eq 'ARM64') {
        return 'arm64'
    } elseif ($arch -eq 'x86') {
        return 'i686'
    } else {
        Log-Error "Unsupported architecture: $arch"
        exit 1
    }
}

function Get-LatestVersion {
    $apiUrl = "https://api.github.com/repos/HelloAIXIAOJI/Friendev/releases/latest"
    try {
        $response = Invoke-RestMethod -Uri $apiUrl -Method Get -TimeoutSec 10
        $tag = $response.tag_name
        if ([string]::IsNullOrWhiteSpace($tag)) {
            throw "Tag name is empty"
        }
        return $tag
    } catch {
        Log-Error "Unable to fetch latest version from GitHub: $_"
        exit 1
    }
}

function Install-Friendev {
    Log-Info "Starting Friendev installation..."

    # 1. Get Architecture
    $arch = Get-Architecture
    Log-Info "Detected Architecture: $arch"

    # 2. Get Latest Version
    Log-Info "Fetching latest version info..."
    $version = Get-LatestVersion
    Log-Info "Latest version: $version"

    # 3. Construct Download URL
    $assetName = "friendev-windows-${arch}.zip"
    $downloadUrl = "https://github.com/HelloAIXIAOJI/Friendev/releases/download/${version}/${assetName}"
    Log-Info "Target file: $assetName"

    # 4. Prepare Temp Directory
    $tempDir = Join-Path $env:TEMP "friendev_install_$(Get-Random)"
    New-Item -ItemType Directory -Path $tempDir -Force | Out-Null
    $zipPath = Join-Path $tempDir $assetName

    try {
        # 5. Download
        Log-Info "Downloading..."
        Invoke-WebRequest -Uri $downloadUrl -OutFile $zipPath -UseBasicParsing
        
        # 6. Extract
        Log-Info "Extracting..."
        Expand-Archive -Path $zipPath -DestinationPath $tempDir -Force

        # 7. Find Binary
        $binaryName = "friendev.exe"
        $binaryPath = Get-ChildItem -Path $tempDir -Filter $binaryName -Recurse | Select-Object -First 1 -ExpandProperty FullName

        if (-not $binaryPath) {
            Log-Error "'$binaryName' not found in the archive"
            exit 1
        }

        # 8. Install Directory
        $installDir = Join-Path $env:LOCALAPPDATA "Friendev\bin"
        if (-not (Test-Path $installDir)) {
            New-Item -ItemType Directory -Path $installDir -Force | Out-Null
        }

        # 9. Move Binary
        Log-Info "Installing to $installDir..."
        Copy-Item -Path $binaryPath -Destination (Join-Path $installDir $binaryName) -Force

        # 10. Update PATH
        $userPath = [Environment]::GetEnvironmentVariable("Path", "User")
        if ($userPath -notlike "*$installDir*") {
            Log-Info "Adding install directory to User PATH..."
            $newPath = "$userPath;$installDir"
            [Environment]::SetEnvironmentVariable("Path", $newPath, "User")
            $env:Path = "$env:Path;$installDir" # Update current session
            Log-Info "PATH updated."
        } else {
            Log-Info "Install directory already in PATH."
        }

        Log-Info "Installation complete!"
        Log-Info "Friendev installed successfully. You may need to restart your terminal."
        Log-Info "Run 'friendev' to start."

    } catch {
        Log-Error "Installation failed: $_"
        exit 1
    } finally {
        # Cleanup
        if (Test-Path $tempDir) {
            Remove-Item -Path $tempDir -Recurse -Force
        }
    }
}

Install-Friendev