#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

void rust_draw(uint8_t *pixels, uint32_t width, uint32_t height);

bool rust_mouse_down(float x, float y);

bool rust_mouse_dragged(float x, float y);

bool rust_mouse_up(float x, float y);
