#!/usr/bin/env bash
# build.sh — Build JewOS from source into a bootable ISO
# Usage: ./build.sh [--kernel-dir /path/to/linux]
#
# Prerequisites:
#   - Rust toolchain (rustup + cargo)
#   - musl target: rustup target add x86_64-unknown-linux-musl
#   - grub-mkrescue, xorriso (for ISO creation)
#   - cpio, gzip (for initramfs)
#   - Optionally: compiled Linux kernel bzImage

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
BUILD_DIR="${SCRIPT_DIR}/build"
ROOTFS_SRC="${SCRIPT_DIR}/rootfs"
INITRAMFS_DIR="${BUILD_DIR}/initramfs"
ISO_DIR="${BUILD_DIR}/iso"
TARGET="x86_64-unknown-linux-musl"
KERNEL_DIR=""

# Parse args
while [[ $# -gt 0 ]]; do
    case "$1" in
        --kernel-dir) KERNEL_DIR="$2"; shift 2 ;;
        *) echo "Unknown option: $1"; exit 1 ;;
    esac
done

echo "=== JewOS Build System ==="

# Step 1: Build all Rust workspace members (statically linked with musl)
echo "[1/5] Building Rust workspace (target: ${TARGET})..."
cargo build --release --target "${TARGET}" --manifest-path "${SCRIPT_DIR}/Cargo.toml"

RELEASE_DIR="${SCRIPT_DIR}/target/${TARGET}/release"

# Step 2: Assemble the initramfs root filesystem
echo "[2/5] Assembling initramfs..."
rm -rf "${INITRAMFS_DIR}"
mkdir -p "${INITRAMFS_DIR}"

# Copy rootfs skeleton
cp -a "${ROOTFS_SRC}/." "${INITRAMFS_DIR}/"

# Copy binaries
mkdir -p "${INITRAMFS_DIR}/bin" "${INITRAMFS_DIR}/sbin"
cp "${RELEASE_DIR}/init"    "${INITRAMFS_DIR}/sbin/init"
cp "${RELEASE_DIR}/shell"   "${INITRAMFS_DIR}/bin/shell"
cp "${RELEASE_DIR}/desktop" "${INITRAMFS_DIR}/bin/desktop"

# Copy all utils binaries
for util in ls cat echo mkdir rm pwd clear uname; do
    if [[ -f "${RELEASE_DIR}/${util}" ]]; then
        cp "${RELEASE_DIR}/${util}" "${INITRAMFS_DIR}/bin/${util}"
    else
        echo "  WARNING: ${util} binary not found, skipping"
    fi
done

# Ensure everything is executable
chmod +x "${INITRAMFS_DIR}/sbin/init"
chmod +x "${INITRAMFS_DIR}/bin/"*

echo "  Rootfs contents:"
find "${INITRAMFS_DIR}" -type f | sort | sed 's|^|    |'

# Step 3: Create initramfs cpio archive
echo "[3/5] Creating initramfs.cpio.gz..."
(cd "${INITRAMFS_DIR}" && find . | cpio -o -H newc --quiet | gzip -9) \
    > "${BUILD_DIR}/initramfs.cpio.gz"

echo "  Size: $(du -h "${BUILD_DIR}/initramfs.cpio.gz" | cut -f1)"

# Step 4: Locate or build the kernel
BZIMAGE=""
if [[ -n "${KERNEL_DIR}" && -f "${KERNEL_DIR}/arch/x86/boot/bzImage" ]]; then
    BZIMAGE="${KERNEL_DIR}/arch/x86/boot/bzImage"
    echo "[4/5] Using kernel: ${BZIMAGE}"
elif [[ -f "${SCRIPT_DIR}/bzImage" ]]; then
    BZIMAGE="${SCRIPT_DIR}/bzImage"
    echo "[4/5] Using pre-built kernel: ${BZIMAGE}"
else
    echo "[4/5] WARNING: No kernel bzImage found."
    echo "  Place a bzImage in the project root, or pass --kernel-dir /path/to/linux"
    echo "  To compile a kernel:"
    echo "    cd /path/to/linux-source"
    echo "    cp ${SCRIPT_DIR}/kernel/.config .config"
    echo "    make olddefconfig"
    echo "    make -j\$(nproc) bzImage"
    echo ""
    echo "  Skipping ISO creation — you can still boot with QEMU directly:"
    echo "    qemu-system-x86_64 -kernel /path/to/bzImage \\"
    echo "      -initrd ${BUILD_DIR}/initramfs.cpio.gz \\"
    echo "      -append 'console=ttyS0 init=/sbin/init' -nographic"
    exit 0
fi

# Step 5: Package into bootable ISO with GRUB
echo "[5/5] Creating bootable ISO..."
rm -rf "${ISO_DIR}"
mkdir -p "${ISO_DIR}/boot/grub"

cp "${BZIMAGE}" "${ISO_DIR}/boot/bzImage"
cp "${BUILD_DIR}/initramfs.cpio.gz" "${ISO_DIR}/boot/initramfs.cpio.gz"
cp "${SCRIPT_DIR}/bootloader/grub.cfg" "${ISO_DIR}/boot/grub/grub.cfg"

grub-mkrescue -o "${BUILD_DIR}/jewos.iso" "${ISO_DIR}" 2>/dev/null

echo ""
echo "=== Build complete ==="
echo "  ISO:       ${BUILD_DIR}/jewos.iso"
echo "  Initramfs: ${BUILD_DIR}/initramfs.cpio.gz"
echo ""
echo "Boot with QEMU:"
echo "  qemu-system-x86_64 -cdrom ${BUILD_DIR}/jewos.iso -m 256M"
echo ""
echo "Or without ISO (direct kernel boot):"
echo "  qemu-system-x86_64 -kernel ${BZIMAGE} \\"
echo "    -initrd ${BUILD_DIR}/initramfs.cpio.gz \\"
echo "    -append 'console=ttyS0 init=/sbin/init' -nographic"
