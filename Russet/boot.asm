[BITS 16]
[ORG 0x7C00]

start:
    xor ax, ax
    mov ds, ax
    mov es, ax
    mov ss, ax
    mov sp, 0x7C00
    mov [BOOT_DRIVE], dl  ; Zapisujemy to, co dał BIOS (0x80 dla HDD)

    ; Reset dysku (ważne!)
    xor ax, ax
    mov dl, [BOOT_DRIVE]
    int 0x13

    ; Wczytywanie Kernela
    mov ah, 0x02
    mov al, 15          ; Zmniejszmy na razie do 15 sektorów (bezpieczniej)
    mov ch, 0
    mov cl, 2           ; Sektor 2
    mov dh, 0
    mov dl, [BOOT_DRIVE]
    mov bx, 0x8000      ; Adres docelowy
    int 0x13
    jc disk_error       ; Jeśli błąd, skocz do wypisania 'E'

    ; 2. Przejście do Protected Mode
    cli
    lgdt [gdt_descriptor]
    mov eax, cr0
    or eax, 0x1
    mov cr0, eax
    jmp CODE_SEG:init_pm

[BITS 32]
init_pm:
    mov ax, DATA_SEG
    mov ds, ax
    mov ss, ax
    mov es, ax
    mov fs, ax
    mov gs, ax

    mov ebp, 0x90000
    mov esp, ebp

    ; Skok do Rusta (funkcja _start musi być pod 0x8000)
    call 0x8000
    jmp $

disk_error:
    mov ah, 0x0E
    mov al, 'E'
    int 0x10
    hlt

gdt_start:
    dq 0x0
gdt_code:
    dw 0xffff, 0x0
    db 0x0, 10011010b, 11001111b, 0x0
gdt_data:
    dw 0xffff, 0x0
    db 0x0, 10010010b, 11001111b, 0x0
gdt_end:
gdt_descriptor:
    dw gdt_end - gdt_start - 1
    dd gdt_start

CODE_SEG equ gdt_code - gdt_start
DATA_SEG equ gdt_data - gdt_start
BOOT_DRIVE db 0

times 510-($-$$) db 0
dw 0xAA55