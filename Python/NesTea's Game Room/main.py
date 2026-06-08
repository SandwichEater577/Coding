import pygame
import sys
import platform
import subprocess
import time

def CyberPong():
    pygame.init()

    user_input = input("Enter The width of the game window (default 1024): ")
    if user_input:
        rozmiarx = int(user_input)
    else:
        rozmiarx = 1024

    rozmiary = rozmiarx * 0.5625

    for frame in ["|", "/", "-", "\\"] * 4:
        subprocess.call('cls' if platform.system() == "Windows" else 'clear', shell=True)
        print(f"Starting CyberPong: {frame}")
        time.sleep(0.05)

    background_colour = (10, 10, 10)
    (width, height) = (rozmiarx, rozmiary)
    
    screen = pygame.display.set_mode((width, height))
    pygame.display.set_caption('CyberPong')

    clock = pygame.time.Clock()
    # clock is a pygame.time.Clock object used to control the game's frame rate
    # calling clock.tick(fps) later limits the main loop to the specified FPS
    font = pygame.font.SysFont("Arial", 60)

    player1 = pygame.Rect(rozmiarx - rozmiarx + 15, height // 2 - 45, 15, 90)
    player2 = pygame.Rect(rozmiarx - 30, height // 2 - 45, 15, 90)
    ball = pygame.Rect(width // 2 - 10, height // 2 - 10, 20, 20)

    ball_speed_x = 5
    ball_speed_y = 5
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
            ball_speed_x = (abs(ball_speed_x) + 0.67) * (-1 if ball_speed_x > 0 else 1) * -1
            ball_speed_y = (abs(ball_speed_y) + 0.67) * (-1 if ball_speed_y > 0 else 1) * -1

        screen.fill(background_colour)

        pygame.draw.aaline(screen, (50, 50, 50), (width // 2, 0), (width // 2, height))

        pygame.draw.rect(screen, (255, 0, 0), player1)
        pygame.draw.rect(screen, (0, 0, 255), player2)
        pygame.draw.ellipse(screen, (255, 255, 255), ball)

        score_surf = font.render(f"{score1} : {score2}", True, (255, 255, 255))

        screen.blit(score_surf, (width // 2 - score_surf.get_width() // 2, 20))

        pygame.display.flip()

        clock.tick(60)
        
        font = pygame.font.SysFont("Arial", 60)

        running = True
        while running:
            for event in pygame.event.get():
                if event.type == pygame.QUIT:
                    running = False
        keys = pygame.key.get_pressed()

        player = pygame.Rect(rozmiarx - rozmiarx + 15, height // 2 - 45, 15, 90)

    pygame.quit()
    ...

def SpaceInvaders():
    pygame.init()

    user_input = input("Enter The width of the game window (default 1024): ")
    if user_input:
        rozmiarx = int(user_input)
    else:
        rozmiarx = 1024

    rozmiary = rozmiarx * 0.5625

    print("1 -> Easy")
    print("2 -> Medium")
    print("3 -> Hard")
    print("4 -> Infinite")
    
    user_input = input("Choose the difficluty (Default: 1): ")

    match user_input:
        case "1":
            difficulty = 1
        case "2":
            difficulty = 2
        case "3":
            difficulty = 3
        case "4":
            difficulty = 4
        case _:
            print("Invalid input. Defaulting to Easy.")
            difficulty = 1

    for frame in ["|", "/", "-", "\\"] * 4:
        subprocess.call('cls' if platform.system() == "Windows" else 'clear', shell=True)
        print(f"Starting Space Invaders: {frame}")
        time.sleep(0.05)

    background_colour = (10, 10, 10)
    (width, height) = (rozmiarx, rozmiary)
    screen = pygame.display.set_mode((width, height))
    pygame.display.set_caption('Space Invaders')

    clock = pygame.time.Clock()

    player = pygame.Rect(width // 2 - 25, height - 60, 50, 30)
    ...

def main():
    # clear terminal in a cross-platform way using subprocess
    clear_cmd = 'cls' if platform.system() == "Windows" else 'clear'

    def clear_screen():
        subprocess.call(clear_cmd, shell=True)

    clear_screen()

    print("welcome to NesTea's Game Room!")
    print("Choose a game to play!:")
    print("0 -> Exit")
    print("1 -> CyberPong")
    print("2 -> Space Invaders")

    while True:
        user_input = input("Enter the number of the game you want to play: ")

        if user_input.isdigit() != True:
            print("Invalid input. Please enter a number.")
            continue

        if user_input == "0":
            for frame in ["|", "/", "-", "\\"] * 4:
                clear_screen()
                print(f"Exiting Game Room: {frame}")
                time.sleep(0.05)
            clear_screen()
            print("GoodBye!")
            time.sleep(0.67)
            clear_screen()
            sys.exit()
        elif user_input == "1":
            CyberPong()
        elif user_input == "2":
            SpaceInvaders()
        else:
            print("Invalid input. Please enter 0, 1, or 2.")

if __name__ == "__main__":
    main()