# JewOS

A custom Linux distro with a pure-Rust userspace. Boots on QEMU x86_64.

## Project Structure

```
JewOS/
├── Cargo.toml              # Workspace root
├── init/                   # PID 1 process (replaces systemd)
│   └── src/main.rs
├── shell/                  # Custom shell (replaces bash)
│   └── src/main.rs
├── utils/                  # Coreutils (ls, cat, echo, mkdir, rm, pwd, clear, uname)
│   └── src/bin/*.rs
├── tui/                    # Win95-style TUI desktop (ratatui + crossterm)
│   └── src/main.rs
├── rootfs/                 # Filesystem skeleton
│   ├── bin/                # User binaries (populated by build.sh)
│   ├── sbin/               # System binaries (init)
│   ├── etc/                # Config files (hostname, passwd, os-release)
│   ├── proc/ sys/ dev/     # Mount points for virtual filesystems
│   ├── tmp/                # Temporary files
│   └── home/user/          # User home directory
├── kernel/                 # Minimal Linux kernel .config for QEMU
├── bootloader/grub.cfg     # GRUB boot config
├── build.sh                # One-command build → bootable ISO
└── README.md
```

## Prerequisites

```bash
# Rust toolchain
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup target add x86_64-unknown-linux-musl

# Build tools
sudo apt install -y musl-tools grub-pc-bin grub-common xorriso mtools cpio qemu-system-x86
```

## Building the Kernel

JewOS uses a standard Linux kernel. Download and compile it:

```bash
# Download kernel source
wget https://cdn.kernel.org/pub/linux/kernel/v6.x/linux-6.6.tar.xz
tar xf linux-6.6.tar.xz
cd linux-6.6

# Use JewOS minimal config
cp /path/to/JewOS/kernel/.config .config
make olddefconfig
make -j$(nproc) bzImage

# Copy to JewOS root
cp arch/x86/boot/bzImage /path/to/JewOS/bzImage
```

## Building JewOS

```bash
# From the JewOS directory:
./build.sh

# Or specify kernel location:
./build.sh --kernel-dir /path/to/linux-6.6
```

## Running in QEMU

### With ISO (full GRUB boot):
```bash
qemu-system-x86_64 -cdrom build/jewos.iso -m 256M
```

### Direct kernel boot (faster, no ISO needed):
```bash
qemu-system-x86_64 \
  -kernel bzImage \
  -initrd build/initramfs.cpio.gz \
  -append "console=ttyS0 init=/sbin/init" \
  -nographic
```

### With graphics (for TUI desktop):
```bash
qemu-system-x86_64 \
  -kernel bzImage \
  -initrd build/initramfs.cpio.gz \
  -append "console=tty0 init=/sbin/init" \
  -m 256M
```

Press `Ctrl+A X` to exit QEMU in `-nographic` mode.

## What to Implement

All `.rs` files contain stubs with TODO comments. Work through them in this order:

1. **utils/** — Start here. Simple standalone binaries, good warm-up.
2. **shell/** — The interactive shell. Needs `nix` for fork/exec/pipe and `rustyline` for line editing.
3. **init/** — PID 1. Mounts /proc /sys /dev, sets hostname, spawns shell. Uses `nix` crate.
4. **tui/** — The Win95 desktop. Uses `ratatui` + `crossterm`.

## Tech Stack

| Component | Crate | Purpose |
|-----------|-------|---------|
| init | `nix` | Syscalls (mount, fork, sethostname, signals) |
| shell | `nix`, `rustyline` | fork/exec/pipe, line editing with history |
| utils | `clap` | CLI argument parsing |
| tui | `ratatui`, `crossterm` | Terminal UI framework |
