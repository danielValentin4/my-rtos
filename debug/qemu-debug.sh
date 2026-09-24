#!/usr/bin/env bash
qemu-system-arm -cpu cortex-m3 -machine lm3s6965evb -nographic \
  -semihosting-config enable=on \
  -kernel "$1" \
  -s -S
