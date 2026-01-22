#include <unistd.h>
#include <stdio.h>

#include <SDL3/SDL.h>

#include "led-strip-display.h"

int main()
{
    if (!SDL_Init(SDL_INIT_VIDEO))
    {
        SDL_Log("SDL init error: %s\n", SDL_GetError());
        return false;
    }

    LEDStripDisplay *display = led_strip_display_new(200, 40);
    if (display)
    {
        printf("Display successfully created!\n");
    }
    else
    {
        printf("Error while creating display!\n");
    }

    sleep(5);

    SDL_Quit();
}
