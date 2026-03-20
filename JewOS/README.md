JewOS/
├── .cargo/
│ └── config.toml ← custom target + linker config
├── src/
│ ├── main.rs ← kernel entry (#![no_std] #![no_main])
│ ├── vga.rs ← VGA text mode driver
│ ├── gdt.rs ← Global Descriptor Table
│ ├── idt.rs ← Interrupt Descriptor Table
│ ├── interrupts.rs ← interrupt handlers (PIC, IRQs)
│ ├── keyboard.rs ← PS/2 keyboard driver
│ └── shell.rs ← prosty shell
├── boot/
│ └── boot.asm ← Multiboot2 header + \_start (NASM)
├── x86_64-jewos.json ← custom target triple
├── Cargo.toml
├── linker.ld
├── Makefile
└── README.md
