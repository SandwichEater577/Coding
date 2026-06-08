#include <windows.h> // Wymagane na Windows do zarządzania rozdzielczością ekranu
#include <stdio.h>
#define GLFW_INCLUDE_NONE
#include <GLFW/glfw3.h>

// Deklaracja rozszerzenia wgl do wyłączenia V-Sync na Windowsie
typedef int (WINAPI *PFNWGLSWAPINTERVALEXTPROC)(int interval);

int main(void) {
    // 1. Inicjalizacja biblioteki okien GLFW
    if (!glfwInit()) {
        printf("Blad inicjalizacji GLFW!\n");
        return -1;
    }

    // 2. Konfiguracja nowoczesnego profilu OpenGL (Core Profile 4.6 lub 3.3)
    glfwWindowHint(GLFW_CONTEXT_VERSION_MAJOR, 4);
    glfwWindowHint(GLFW_CONTEXT_VERSION_MINOR, 6);
    glfwWindowHint(GLFW_OPENGL_PROFILE, GLFW_OPENGL_CORE_PROFILE);
    
    // Konfiguracja okna: brak obramowania dla idealnego Borderless Fullscreen
    glfwWindowHint(GLFW_DECORATED, GLFW_FALSE);

    // 3. Pobranie natywnej rozdzielczości Twojego monitora Windows
    int screenWidth = GetSystemMetrics(SM_CXSCREEN);
    int screenHeight = GetSystemMetrics(SM_CYSCREEN);

    // 4. Stworzenie okna gry na pełny ekran
    GLFWwindow* window = glfwCreateWindow(screenWidth, screenHeight, "Raw C 3D Engine", NULL, NULL);
    if (!window) {
        printf("Blad podczas tworzenia okna GLFW!\n");
        glfwTerminate();
        return -1;
    }

    // Ustawienie okna na pozycji (0,0) - pokrywa cały ekran
    glfwSetWindowPos(window, 0, 0);
    glfwMakeContextCurrent(window);

    // 5. EKSTREMALNA OPTYMALIZACJA: Wyłączenie V-Sync na poziomie sterownika GPU
    PFNWGLSWAPINTERVALEXTPROC wglSwapIntervalEXT = 
        (PFNWGLSWAPINTERVALEXTPROC)wglGetProcAddress("wglSwapIntervalEXT");
    if (wglSwapIntervalEXT) {
        wglSwapIntervalEXT(0); // 0 oznacza całkowite odblokowanie klatek (brak limitu)
    }

    // Ukrycie i zablokowanie myszy wewnątrz okna gry
    glfwSetInputMode(window, GLFW_CURSOR, GLFW_CURSOR_DISABLED);

    double lastTime = glfwGetTime();
    int frameCount = 0;
    int currentFPS = 0;

    // 6. Główna pętla gry (Działa dopóki nie zamkniesz gry lub nie wciśniesz ESC)
    while (!glfwWindowShouldClose(window)) {
        
        // --- PROSTY LICZNIK FPS W TERMINALU (Aby nie marnować zasobów na UI) ---
        double currentTime = glfwGetTime();
        frameCount++;
        if (currentTime - lastTime >= 1.0) {
            currentFPS = frameCount;
            // Wyświetla FPS w tytule okna / terminalu
            char title[50];
            sprintf(title, "Raw C Engine | FPS: %d", currentFPS);
            // Uwaga: Zmiana tytułu ukrytego okna borderless nie zwalnia pętli renderu
            printf("FPS: %d\n", currentFPS); 
            frameCount = 0;
            lastTime = currentTime;
        }

        // --- OBSŁUGA KLAWIATURY ---
        if (glfwGetKey(window, GLFW_KEY_ESCAPE) == GLFW_PRESS) {
            glfwSetWindowShouldClose(window, GLFW_TRUE);
        }

        // --- RENDEROWANIE (Czyste GPU) ---
        // Czyścimy ekran do koloru ciemnoszarego
        // glClearColor(R, G, B, A)
        // glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);

        // Zamiana buforów (Wyświetlenie wyrenderowanej klatki na ekran)
        glfwSwapBuffers(window);
        
        // Sprawdzenie zdarzeń systemowych (mysz, klawiatura)
        glfwPollEvents();
    }

    // 7. Czyszczenie pamięci po wyjściu z pętli
    glfwDestroyWindow(window);
    glfwTerminate();
    return 0;
}
