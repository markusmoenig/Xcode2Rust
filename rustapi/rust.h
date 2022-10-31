#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

void rust_draw(uint8_t *pixels, uint32_t width, uint32_t height);

uint32_t rust_target_fps(void);

bool rust_touch_down(float x, float y);

bool rust_touch_dragged(float x, float y);

bool rust_touch_up(float x, float y);
