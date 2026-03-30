// kernel.c

// Definicje kolorów
#define COLOR_GREEN 0x0A
#define COLOR_RED 0x0C
#define COLOR_WHITE 0x0F

// Funkcja czyszcząca ekran
void cls()
{
    char *vga = (char *)0xb8000;
    for (int i = 0; i < 80 * 25; i++)
    {
        vga[i * 2] = ' ';      // Spacja
        vga[i * 2 + 1] = 0x07; // Standardowy szary kolor
    }
}

// Funkcja wypisująca tekst w konkretnej linii
void print_at(char *message, int line, char color)
{
    char *vga = (char *)0xb8000;
    int offset = line * 80 * 2; // Każda linia to 80 znaków, każdy znak to 2 bajty

    for (int i = 0; message[i] != '\0'; i++)
    {
        vga[offset + i * 2] = message[i];
        vga[offset + i * 2 + 1] = color;
    }
}

void _start()
{
    cls(); // Czyścimy śmieci po BIOSie

    print_at("--- Russet OS Kernel v0.1 ---", 0, COLOR_WHITE);
    print_at("Status: System zaladowany pomyslnie.", 1, COLOR_GREEN);
    print_at("Blad testowy: Brak klawiatury (jeszcze).", 3, COLOR_RED);

    while (1)
        ;
}