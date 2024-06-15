import pygame
import random

pygame.init()

texture_size = (16, 16)
SIZE_FACTOR = 10

texture = []

def r():
    return round(random.random()*100)/100

for i in range(texture_size[1]):
    texture.append([])
    for j in range(texture_size[0]):
        texture[i].append([r(), r(), r(), 1])

window_size = (texture_size[0] * SIZE_FACTOR, texture_size[1] * SIZE_FACTOR)
window = pygame.display.set_mode(window_size)

print(texture)

running = True
while running:
    for event in pygame.event.get():
        if event.type == pygame.QUIT:
            running = False

    window.fill((0, 0, 0))

    for ir, r in enumerate(texture):
        for ic, c in enumerate(r):
            pygame.draw.rect(window, tuple([x*255 for x in c[:3]]), ((ic * SIZE_FACTOR, ir * SIZE_FACTOR), (SIZE_FACTOR, SIZE_FACTOR)))

    pygame.display.flip()

pygame.quit()
