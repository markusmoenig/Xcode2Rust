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

int shipping_rust_addition(int a, int b);

char *rust_greeting(const char *to);

void rust_greeting_free(char *s);

void foo_new(uint8_t *pstext, uint32_t itextlen);

#endif /* Header_h */
