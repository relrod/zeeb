#!/usr/bin/env bash

# Must be <= 12 characters
# Filter out words with characters not in the alphabet
# Must be >= 3 characters
grep -v '.\{13,\}' american-english-huge.orig \
    | grep -v '[^dofmjavlzbntseypicxguwhrk]' \
    | grep '.\{3,\}' \
