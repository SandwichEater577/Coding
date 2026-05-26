import pygame
import sys

def main():
    pygame.init()
    
    rozmiarx = 676
    rozmiary = 767

    background_colour = (10, 10, 10)
    (width, height) = (rozmiarx, rozmiary)
    
    screen = pygame.display.set_mode((width, height))
    pygame.display.set_caption('CyberPong')

    clock = pygame.time.Clock()
    font = pygame.font.SysFont("Arial", 60)

    player1 = pygame.Rect(rozmiarx - rozmiarx + 15, height // 2 - 45, 15, 90)
    player2 = pygame.Rect(rozmiarx - 30, height // 2 - 45, 15, 90)
    ball = pygame.Rect(width // 2 - 10, height // 2 - 10, 20, 20)

    ball_speed_x = 1
    ball_speed_y = 1
    player_speed = 7

    score1 = 0
    score2 = 0

    running = True
    while running:
        for event in pygame.event.get():
            if event.type == pygame.QUIT:
                running = False
        keys = pygame.key.get_pressed()

        if keys[pygame.K_q]:
            print("Exiting Pong")
            print(f"Final Score: P1:{score1} P2:{score2}")
            pygame.quit()
            return

        if keys[pygame.K_w] and player1.top > 0:
            player1.y -= player_speed
        if keys[pygame.K_s] and player1.bottom < height:
            player1.y += player_speed

        if keys[pygame.K_UP] and player2.top > 0:
            player2.y -= player_speed
        if keys[pygame.K_DOWN] and player2.bottom < height:
            player2.y += player_speed

        ball.x += ball_speed_x
        ball.y += ball_speed_y

        if ball.top <= 0 or ball.bottom >= height:
            ball_speed_y *= -1

        if ball.colliderect(player1):
            ball_speed_x = abs(ball_speed_x)
        if ball.colliderect(player2):
            ball_speed_x = -abs(ball_speed_x)

        has_collision = False

        if ball.left <= 0:
            score2 += 1
            has_collision = True
        if ball.right >= width:
            score1 += 1
            has_collision = True

        if has_collision:
            ball.center = (width // 2, height // 2)
            if ball_speed_x <= 0:
                ball_speed_x -= 1
            else:
                ball_speed_x += 1

            if ball_speed_y <= 0:
                ball_speed_y -= 1
            else:
                ball_speed_y += 1

            ball_speed_x *= -1
            ball_speed_y *= -1

        screen.fill(background_colour)

        pygame.draw.aaline(screen, (50, 50, 50), (width // 2, 0), (width // 2, height))

        pygame.draw.rect(screen, (255, 0, 0), player1)
        pygame.draw.rect(screen, (0, 0, 255), player2)
        pygame.draw.ellipse(screen, (255, 255, 255), ball)

        score_surf = font.render(f"{score1} : {score2}", True, (255, 255, 255))

        screen.blit(score_surf, (width // 2 - score_surf.get_width() // 2, 20))

        pygame.display.flip()
        clock.tick(60)

    pygame.quit()
    sys.exit()

if __name__ == "__main__":
    main()
