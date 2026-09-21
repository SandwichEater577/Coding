import pygame

def main():
    pygame.init()
    userInput = input("Enter the screen size (width, height): ")
    width, height = map(int, userInput.split(','))

    screen = pygame.display.set_mode((width, height))

    pygame.display.set_caption("TheSpace")

    running = True

    while running:
        for event in pygame.event.get():
            if event.type == pygame.QUIT:
                running = False

        screen.fill((255,255,255))