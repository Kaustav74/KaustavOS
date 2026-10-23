# Simple Start Guide

## Goal
Get something that boots and shows text as quickly as possible, then grow the features you asked for.

## Tools You Need
- Rust toolchain (rustup)
- QEMU
- (Optional but recommended) Limine bootloader

## Quick Path
1. Follow a public "Writing an OS in Rust" tutorial to get VGA text output working.
2. Change the printed text to "KaustavOS".
3. Add a short sequence that prints your threads then clears the screen.
4. Add a keyboard-driven menu for Windows / macOS / Linux style.
5. Later add USB persistence and real isolation.

## Why We Don't Copy Proprietary Software
Windows, macOS, and closed parts of other systems are copyrighted. Copying their code or interfaces verbatim is not allowed. We build original, simple versions inspired by open-source designs and public educational materials only.

This keeps the project legal, educational, and focused on the secure lightweight architecture we discussed.
