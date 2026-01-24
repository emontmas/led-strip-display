/**
 * led-strip-display C FFI bindings
 */

#ifndef LED_STRIP_DISPLAY_H
#define LED_STRIP_DISPLAY_H

#include <stddef.h>
#include <stdint.h>

typedef struct LEDStripDisplay LEDStripDisplay;

typedef struct
{
    uint8_t r;
    uint8_t g;
    uint8_t b;
} LED;

/**
 * @brief Initialse a LED strip display.
 * @return The initialised LEDStripDisplay, or NULL if an error occured.
 */
LEDStripDisplay *led_strip_display_new(size_t length, uint32_t led_per_row);

/**
 * @brief Free the LED strip display previously initialised by led_strip_display_new().
 */
void led_strip_display_free(LEDStripDisplay *display);

/**
 * @brief Update the LED strip display with the given LED array.
 * @result 0 if the display was updated without error, -1 otherwise.
 */
int led_strip_update(LEDStripDisplay *display, const LED *leds, size_t length);

#endif // LED_STRIP_DISPLAY_H
