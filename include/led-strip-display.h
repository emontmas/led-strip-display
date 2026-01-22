/**
 * led-strip-display C FFI bindings
 */

#include <stddef.h>
#include <stdint.h>

typedef struct LEDStripDisplay LEDStripDisplay;

/**
 * @brief Initialse a LED strip display.
 * @return The initialised LEDStripDisplay, or NULL if an error occured.
 */
LEDStripDisplay *led_strip_display_new(size_t length, uint32_t led_per_row);
