import pygame
import sys

def menu():
    """Funkcja powrotu do menu (opcjonalna)."""
    print("Powrót do menu...")
    # Tutaj możesz dodać logikę menu głównego
    sys.exit()

def main():
    # 1. Ustawienia początkowe
    pygame.init()
    
    background_colour = (10, 10, 10)
    (width, height) = (1000, 666)
    
    screen = pygame.display.set_mode((width, height))
    pygame.display.set_caption('CyberPong')

    clock = pygame.time.Clock()
    font = pygame.font.SysFont("Arial", 60)

    # 2. Definicja obiektów
    player1 = pygame.Rect(15, height // 2 - 45, 15, 90)
    player2 = pygame.Rect(970, height // 2 - 45, 15, 90)
    ball = pygame.Rect(width // 2 - 10, height // 2 - 10, 20, 20)

    # Prędkości
    ball_speed_x = 7
    ball_speed_y = 7
    player_speed = 7

    # Punktacja
    score1 = 0
    score2 = 0

    running = True
    while running:
        # --- OBSŁUGA ZDARZEŃ ---
        for event in pygame.event.get():
            if event.type == pygame.QUIT:
                running = False

        # --- STEROWANIE ---
        keys = pygame.key.get_pressed()

        # Quit system
        if keys[pygame.K_q]:
            print("Exiting Pong")
            print(f"Final Score: P1:{score1} P2:{score2}")
            pygame.quit()
            menu()
            return

        # Gracz 1 (W, S)
        if keys[pygame.K_w] and player1.top > 0:
            player1.y -= player_speed
        if keys[pygame.K_s] and player1.bottom < height:
            player1.y += player_speed

        # Gracz 2 (Strzałki)
        if keys[pygame.K_UP] and player2.top > 0:
            player2.y -= player_speed
        if keys[pygame.K_DOWN] and player2.bottom < height:
            player2.y += player_speed

        # --- LOGIKA PIŁKI ---
        ball.x += ball_speed_x
        ball.y += ball_speed_y

        # Odbicia od góry i dołu
        if ball.top <= 0 or ball.bottom >= height:
            ball_speed_y *= -1

        # Kolizje z paletkami
        if ball.colliderect(player1):
            ball_speed_x = abs(ball_speed_x)
        if ball.colliderect(player2):
            ball_speed_x = -abs(ball_speed_x)

        # --- PUNKTACJA I RESET ---
        if ball.left <= 0:
            score2 += 1
            ball.center = (width // 2, height // 2)
            ball_speed_x *= -1 

        if ball.right >= width:
            score1 += 1
            ball.center = (width // 2, height // 2)
            ball_speed_x *= -1

        # --- RYSOWANIE ---
        screen.fill(background_colour)

        # Linia środkowa
        pygame.draw.aaline(screen, (50, 50, 50), (width // 2, 0), (width // 2, height))

        # Obiekty
        pygame.draw.rect(screen, (255, 0, 0), player1)
        pygame.draw.rect(screen, (0, 0, 255), player2)
        pygame.draw.ellipse(screen, (255, 255, 255), ball)

        # Napisy (Wynik)
        score_surf = font.render(f"{score1} : {score2}", True, (255, 255, 255))
        screen.blit(score_surf, (width // 2 - score_surf.get_width() // 2, 20))

        # --- ODŚWIEŻANIE ---
        pygame.display.flip()
        clock.tick(60)

    pygame.quit()
    sys.exit()

if __name__ == "__main__":
    main()
