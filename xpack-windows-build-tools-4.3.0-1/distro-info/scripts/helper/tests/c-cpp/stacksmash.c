/*
 * Copyright (c) 2018 Martin Storsjo
 *
 * This file is part of llvm-mingw.
 *
 * Permission to use, copy, modify, and/or distribute this software for any
 * purpose with or without fee is hereby granted, provided that the above
 * copyright notice and this permission notice appear in all copies.
 *
 * THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
 * WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
 * MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
 * ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
 * WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
 * ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
 * OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
 */

#include <stdio.h>

__attribute__((noinline))
void func(char *ptr, int idx) {
    ptr[idx] = 0;
}

int main(int argc, char *argv[]) {
    char buf[10];
    if (argc > 1) {
        fprintf(stderr, "smashing stack\n");
        fflush(stderr);
        func(buf, 10);
    } else {
        func(buf, 9);
        fprintf(stderr, "%s: A test tool for detecting stack smashing.\n"
                        "Run this with a command line argument to trigger an "
                        "error.\n", argv[0]);
    }
    return 0;
}
