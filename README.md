# RGBE - Gameboy emulator written in Rust
Right now, emulator passes all of Blargg's cpu_instrs tests (except interrupts; interrupt handling is implemented, BUT triggering them is not). There is also a prototype of a clock (timing) and PPU (though PPU is right now disconnected from everything and needs a rewrite; it never worked). Finally, there is a simple debugger and logging.

For now, I don't work on this project, as my studies consume most of my life. However, I intent to return to it, and at least finish PPU, interrupts, and banking. Ideally, I will implemennt the whole system.