#!/bin/bash
REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

echo "=== Starting Automated Interactive Session in QEMU ==="

(
  # Wait for kernel load, splash screen, and thread sequence animation
  sleep 24
  # Select option 1 (Windows style)
  printf "1\n"
  sleep 3
  # Add a persistent note
  printf "note First journal entry on KaustavOS USB persistence\n"
  sleep 3
  # View persistent notes
  printf "notes\n"
  sleep 3
  # View system info
  printf "sys\n"
  sleep 3
  # Hot-switch to macOS style
  printf "theme 2\n"
  sleep 3
  # Show personal threads
  printf "threads\n"
  sleep 3
  # Clean poweroff
  printf "poweroff\n"
) | timeout 55s qemu-system-x86_64 \
    -cdrom "$REPO_ROOT/build/kaustavos.iso" \
    -m 1024M \
    -nographic \
    -serial mon:stdio || true
