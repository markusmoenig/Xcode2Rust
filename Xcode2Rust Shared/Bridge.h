//
//  Header.h
//  Xcode2Rust
//
//  Created by Markus Moenig on 30/10/22.
//

#ifndef Bridge_h
#define Bridge_h

#import "Metal.h"

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

void rust_draw(uint8_t *pixels, uint32_t width, uint32_t height);

bool rust_touch_down(float x, float y);

bool rust_touch_dragged(float x, float y);

bool rust_touch_up(float x, float y);

#endif /* Header_h */
