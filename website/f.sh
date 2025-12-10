#!/bin/bash

# Friendev Installation Script
set -e

# Color definitions
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Log functions
log_info() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

log_warn() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Check dependencies
check_dependencies() {
    local deps=("curl" "tar" "uname")
    for dep in "${deps[@]}"; do
        if ! command -v "$dep" &> /dev/null; then
            log_error "Missing necessary dependency: $dep"
            exit 1
        fi
    done
}

# Get OS
get_os() {
    local os
    os=$(uname -s)
    case "$os" in
        Linux)
            # Check for Android (Termux or other environments)
            if [ -n "$ANDROID_ROOT" ] || [ -n "$ANDROID_DATA" ] || (uname -o 2>/dev/null | grep -q Android); then
                echo "android"
            else
                echo "linux"
            fi
            ;;
        Darwin)
            echo "macos"
            ;;
        FreeBSD)
            echo "freebsd"
            ;;
        *)
            log_error "Unsupported OS: $os"
            exit 1
            ;;
    esac
}

# Get Architecture
get_architecture() {
    local arch
    arch=$(uname -m)
    
    case "$arch" in
        x86_64)
            echo "amd64"
            ;;
        aarch64|arm64)
            echo "arm64"
            ;;
        armv7l)
            echo "armv7"
            ;;
        armv6l)
            echo "arm"
            ;;
        i686)
            echo "i686"
            ;;
        ppc64le)
            echo "powerpc64le"
            ;;
        riscv64)
            echo "riscv64"
            ;;
        s390x)
            echo "s390x"
            ;;
        *)
            log_error "Unsupported architecture: $arch"
            exit 1
            ;;
    esac
}

# Check for musl libc (Linux only)
is_musl() {
    if ldd /bin/sh 2>/dev/null | grep -q musl; then
        return 0
    fi
    return 1
}

# Get latest non-pre-release version
get_latest_release() {
    local api_url="https://api.github.com/repos/HelloAIXIAOJI/Friendev/releases/latest"
    local response
    
    # Use curl to get latest version info
    if ! response=$(curl -s --connect-timeout 10 "$api_url"); then
        log_error "Unable to connect to GitHub API, please check your network connection"
        return 1
    fi
    
    if [ -z "$response" ]; then
        log_error "GitHub API returned empty data"
        return 1
    fi
    
    # Parse tag_name
    local latest_tag
    latest_tag=$(echo "$response" | grep -o '"tag_name": *"[^"]*"' | head -1 | sed 's/.*"tag_name": *"\([^"]*\)".*/\1/')
    
    if [ -z "$latest_tag" ]; then
        # Fallback parsing method
        latest_tag=$(echo "$response" | sed -n 's/.*"tag_name": "\([^"]*\)".*/\1/p' | head -1)
    fi
    
    if [ -z "$latest_tag" ]; then
        log_error "Unable to parse latest version tag"
        return 1
    fi
    
    echo "$latest_tag"
}

# Download and Install
install_friendev() {
    local version
    local os
    local arch
    local asset_name
    local download_url
    local install_dir="/usr/local/bin"
    
    # Adjust install dir for Android/Termux
    if [ -n "$ANDROID_ROOT" ] || [ -n "$ANDROID_DATA" ]; then
        install_dir="$PREFIX/bin"
    fi

    log_info "Starting Friendev installation..."
    
    check_dependencies
    
    log_info "Fetching latest version info..."
    if ! version=$(get_latest_release); then
        exit 1
    fi
    log_info "Latest version: $version"
    
    os=$(get_os)
    arch=$(get_architecture)
    log_info "Detected System: $os, Architecture: $arch"
    
    # Build filename
    # Normalization for specific platforms
    local file_arch="$arch"
    
    if [ "$os" = "android" ] && [ "$arch" = "amd64" ]; then
        file_arch="x86_64"
    fi

    if [ "$os" = "linux" ]; then
        if [ "$arch" = "amd64" ] || [ "$arch" = "arm64" ]; then
             if is_musl; then
                asset_name="friendev-${os}-${file_arch}-musl.tar.gz"
                log_info "Detected musl libc, using musl version"
            else
                asset_name="friendev-${os}-${file_arch}.tar.gz"
            fi
        else
             asset_name="friendev-${os}-${file_arch}.tar.gz"
        fi
    else
        asset_name="friendev-${os}-${file_arch}.tar.gz"
    fi
    
    log_info "Target file: $asset_name"
    download_url="https://github.com/HelloAIXIAOJI/Friendev/releases/download/${version}/${asset_name}"
    
    # Create temp dir
    local temp_dir
    temp_dir=$(mktemp -d)
    # Ensure cleanup
    trap 'rm -rf "$temp_dir"' EXIT
    
    cd "$temp_dir"
    
    log_info "Downloading..."
    # Use --progress-bar
    if ! curl -L -f --progress-bar --connect-timeout 30 -o "$asset_name" "$download_url"; then
        log_error "Download failed: $download_url"
        # Fallback logic for Linux musl
        if [[ "$os" == "linux" ]] && [[ "$asset_name" == *-musl.tar.gz ]]; then
            log_info "Trying non-musl version..."
            asset_name="friendev-${os}-${file_arch}.tar.gz"
            download_url="https://github.com/HelloAIXIAOJI/Friendev/releases/download/${version}/${asset_name}"
            if ! curl -L -f --progress-bar --connect-timeout 30 -o "$asset_name" "$download_url"; then
                log_error "Download failed: $download_url"
                exit 1
            fi
        else
            exit 1
        fi
    fi
    
    log_info "Extracting..."
    if ! tar -xzf "$asset_name"; then
        log_error "Extraction failed"
        exit 1
    fi
    
    # Find binary
    local binary_path
    binary_path=$(find . -type f -name "friendev" | head -n 1)
    
    if [ -z "$binary_path" ]; then
        log_error "'friendev' binary not found in the archive"
        exit 1
    fi
    
    log_info "Installing to $install_dir..."
    
    # Check sudo
    local sudo_cmd=""
    if [ ! -w "$install_dir" ]; then
        if command -v sudo &> /dev/null; then
            sudo_cmd="sudo"
            log_warn "Sudo permission required to write to $install_dir"
        else
            log_error "Current user has no write permission for $install_dir and sudo not found"
            exit 1
        fi
    fi
    
    $sudo_cmd cp -f "$binary_path" "$install_dir/friendev"
    $sudo_cmd chmod +x "$install_dir/friendev"
    
    log_info "Installation complete!"
    
    if command -v friendev &> /dev/null; then
        log_info "Friendev installed successfully. Run 'friendev' to start."
    else
        log_warn "Installation complete, but '$install_dir' might not be in your PATH."
        log_warn "Try running: export PATH=\$PATH:$install_dir"
    fi
}

main() {
    install_friendev
}

main "$@"
