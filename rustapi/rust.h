#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

void rust_draw(uint8_t *pixels, uint32_t width, uint32_t height);

bool rust_mouse_down(uint32_t x, uint32_t y);

bool rust_mouse_dragged(uint32_t x, uint32_t y);

bool rust_mouse_up(uint32_t x, uint32_t y);
