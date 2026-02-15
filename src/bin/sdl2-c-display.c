#include <unistd.h>
#include <stdio.h>

#include <SDL2/SDL.h>

#include "led-strip-display.h"

#define ARRAY_LENGTH(x) (sizeof(x) / sizeof((x)[0]))

int main()
{
    if (SDL_Init(SDL_INIT_VIDEO) < 0)
    {
        SDL_Log("SDL init error: %s\n", SDL_GetError());
        return 1;
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

    LED leds[30];
    for (int i = 0; i < ARRAY_LENGTH(leds); i++)
    {
        uint8_t v = 25.5 * (i % 10);
        if (i < 10)
        {
            leds[i].r = v;
            leds[i].g = 0;
            leds[i].b = 0;
        }
        else if (i < 20)
        {
            leds[i].r = 0;
            leds[i].g = v;
            leds[i].b = 0;
        }
        else
        {
            leds[i].r = 0;
            leds[i].g = 0;
            leds[i].b = v;
        }
    }

    led_strip_update(display, leds, ARRAY_LENGTH(leds));

    sleep(5);

    led_strip_display_free(display);

    SDL_Quit();

    return 0;
}
