#!/usr/bin/env zsh

# Remove files matching 'mouse-entropy-*' from current directory
rm -fv ./mouse-entropy-*

# Remove matching directories in src/
rm -rfv ./src/mouse-entropy-*

# Remove files matching 'mouse-entropy-*' from src/ directory
rm -fv ./src/mouse-entropy*

# Remove pkg directory recursively and forcefully
rm -rfv ./pkg

# Remove pkg directory recursively and forcefully
rm -rfv ./target
