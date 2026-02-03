#!/bin/bash
#
# ServerMark Installation Script
# https://github.com/Grazulex/servermark
#
# Usage:
#   curl -fsSL https://raw.githubusercontent.com/Grazulex/servermark/main/install.sh | bash
#
# Or with wget:
#   wget -qO- https://raw.githubusercontent.com/Grazulex/servermark/main/install.sh | bash
#

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# GitHub repository
REPO="Grazulex/servermark"
APP_NAME="servermark"
INSTALL_DIR="/opt/servermark"
BIN_LINK="/usr/local/bin/servermark"

# Print functions
info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

success() {
    echo -e "${GREEN}[OK]${NC} $1"
}

warn() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

error() {
    echo -e "${RED}[ERROR]${NC} $1"
    exit 1
}

# Banner
print_banner() {
    echo -e "${BLUE}"
    echo "  ____                           __  __            _    "
    echo " / ___|  ___ _ ____   _____ _ __|  \/  | __ _ _ __| | __"
    echo " \___ \ / _ \ '__\ \ / / _ \ '__| |\/| |/ _\` | '__| |/ /"
    echo "  ___) |  __/ |   \ V /  __/ |  | |  | | (_| | |  |   < "
    echo " |____/ \___|_|    \_/ \___|_|  |_|  |_|\__,_|_|  |_|\_\\"
    echo -e "${NC}"
    echo "  Local Development Environment Manager for Linux"
    echo ""
}

# Check if running as root
check_root() {
    if [ "$EUID" -ne 0 ]; then
        error "This script must be run as root. Use: sudo bash install.sh"
    fi
}

# Detect architecture
detect_arch() {
    local arch
    arch=$(uname -m)

    case "$arch" in
        x86_64|amd64)
            echo "x86_64"
            ;;
        aarch64|arm64)
            echo "aarch64"
            ;;
        *)
            error "Unsupported architecture: $arch. ServerMark supports x86_64 and aarch64."
            ;;
    esac
}

# Detect Linux distribution
detect_distro() {
    if [ -f /etc/os-release ]; then
        . /etc/os-release
        echo "$ID"
    elif [ -f /etc/debian_version ]; then
        echo "debian"
    elif [ -f /etc/fedora-release ]; then
        echo "fedora"
    elif [ -f /etc/arch-release ]; then
        echo "arch"
    else
        echo "unknown"
    fi
}

# Detect package type based on distribution
detect_package_type() {
    local distro=$1

    case "$distro" in
        ubuntu|debian|linuxmint|pop|elementary|zorin)
            echo "deb"
            ;;
        fedora|rhel|centos|rocky|alma|nobara)
            echo "rpm"
            ;;
        arch|manjaro|endeavouros|garuda)
            echo "appimage"
            ;;
        opensuse*|suse*)
            echo "rpm"
            ;;
        *)
            echo "appimage"
            ;;
    esac
}

# Check for required tools
check_dependencies() {
    local missing=()

    # Check for curl or wget
    if ! command -v curl &> /dev/null && ! command -v wget &> /dev/null; then
        missing+=("curl or wget")
    fi

    if [ ${#missing[@]} -ne 0 ]; then
        error "Missing required tools: ${missing[*]}. Please install them first."
    fi
}

# Download function (uses curl or wget)
download() {
    local url=$1
    local output=$2

    if command -v curl &> /dev/null; then
        curl -fsSL "$url" -o "$output"
    elif command -v wget &> /dev/null; then
        wget -q "$url" -O "$output"
    fi
}

# Download to stdout (for API calls)
download_stdout() {
    local url=$1

    if command -v curl &> /dev/null; then
        curl -fsSL "$url"
    elif command -v wget &> /dev/null; then
        wget -qO- "$url"
    fi
}

# Get latest release version from GitHub
get_latest_version() {
    local api_url="https://api.github.com/repos/${REPO}/releases/latest"
    local version

    version=$(download_stdout "$api_url" | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/')

    if [ -z "$version" ]; then
        error "Failed to fetch latest version from GitHub. Check your internet connection."
    fi

    echo "$version"
}

# Build download URL based on package type and architecture
get_download_url() {
    local version=$1
    local pkg_type=$2
    local arch=$3

    # Remove 'v' prefix if present for filename
    local ver_num=${version#v}

    case "$pkg_type" in
        deb)
            echo "https://github.com/${REPO}/releases/download/${version}/ServerMark_${ver_num}_amd64.deb"
            ;;
        rpm)
            echo "https://github.com/${REPO}/releases/download/${version}/ServerMark-${ver_num}-1.${arch}.rpm"
            ;;
        appimage)
            echo "https://github.com/${REPO}/releases/download/${version}/ServerMark_${ver_num}_amd64.AppImage"
            ;;
    esac
}

# Install .deb package
install_deb() {
    local pkg_path=$1

    info "Installing .deb package..."

    if command -v apt &> /dev/null; then
        apt install -y "$pkg_path"
    elif command -v dpkg &> /dev/null; then
        dpkg -i "$pkg_path"
        apt-get install -f -y 2>/dev/null || true
    else
        error "No suitable package manager found for .deb installation"
    fi
}

# Install .rpm package
install_rpm() {
    local pkg_path=$1

    info "Installing .rpm package..."

    if command -v dnf &> /dev/null; then
        dnf install -y "$pkg_path"
    elif command -v yum &> /dev/null; then
        yum install -y "$pkg_path"
    elif command -v zypper &> /dev/null; then
        zypper install -y "$pkg_path"
    else
        error "No suitable package manager found for .rpm installation"
    fi
}

# Install AppImage
install_appimage() {
    local pkg_path=$1

    info "Installing AppImage..."

    # Create installation directory
    mkdir -p "$INSTALL_DIR"

    # Move AppImage to installation directory
    mv "$pkg_path" "$INSTALL_DIR/ServerMark.AppImage"
    chmod +x "$INSTALL_DIR/ServerMark.AppImage"

    # Create symlink
    ln -sf "$INSTALL_DIR/ServerMark.AppImage" "$BIN_LINK"

    # Create desktop entry
    cat > /usr/share/applications/servermark.desktop << EOF
[Desktop Entry]
Name=ServerMark
Comment=Local Development Environment Manager
Exec=$INSTALL_DIR/ServerMark.AppImage
Icon=servermark
Type=Application
Categories=Development;
Terminal=false
EOF

    success "AppImage installed to $INSTALL_DIR"
}

# Verify installation
verify_installation() {
    info "Verifying installation..."

    if command -v servermark &> /dev/null || [ -f "$BIN_LINK" ] || [ -f "$INSTALL_DIR/ServerMark.AppImage" ]; then
        return 0
    else
        return 1
    fi
}

# Cleanup temporary files
cleanup() {
    local tmp_dir=$1
    if [ -d "$tmp_dir" ]; then
        rm -rf "$tmp_dir"
    fi
}

# Main installation function
main() {
    print_banner

    # Pre-flight checks
    check_root
    check_dependencies

    # Detect system
    local arch=$(detect_arch)
    local distro=$(detect_distro)
    local pkg_type=$(detect_package_type "$distro")

    info "Detected: $distro ($arch)"
    info "Package type: $pkg_type"

    # Get latest version
    info "Fetching latest version..."
    local version=$(get_latest_version)
    success "Latest version: $version"

    # Build download URL
    local download_url=$(get_download_url "$version" "$pkg_type" "$arch")
    info "Download URL: $download_url"

    # Create temporary directory
    local tmp_dir=$(mktemp -d)
    trap "cleanup $tmp_dir" EXIT

    # Download package
    local pkg_file="$tmp_dir/servermark.$pkg_type"
    info "Downloading ServerMark $version..."
    download "$download_url" "$pkg_file"

    if [ ! -f "$pkg_file" ]; then
        error "Download failed. Please check your internet connection."
    fi
    success "Download complete"

    # Install based on package type
    case "$pkg_type" in
        deb)
            install_deb "$pkg_file"
            ;;
        rpm)
            install_rpm "$pkg_file"
            ;;
        appimage)
            install_appimage "$pkg_file"
            ;;
    esac

    # Verify installation
    if verify_installation; then
        echo ""
        success "ServerMark $version installed successfully!"
        echo ""
        echo -e "${GREEN}Next steps:${NC}"
        echo "  1. Launch ServerMark from your application menu"
        echo "  2. Or run: servermark"
        echo ""
        echo -e "${BLUE}Documentation:${NC} https://github.com/${REPO}"
        echo ""
    else
        error "Installation verification failed. Please check the logs above."
    fi
}

# Run main function
main "$@"
