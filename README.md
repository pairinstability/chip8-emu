CHIP-8 emulator and disassembler in rust
========================================

CHIP-8 is an old programming language from the 1970s designed for some 8-bit
processors. it was made to be simple to allow video games to be easily
programmed, as such there are some neat classic games ported to CHIP-8, with the
target of this emulator to be able to play space invaders.

roms taken from [here](https://github.com/kurtjd/jaxe)

the emulator uses rust-sdl2 for display 


disassembler
-----------

afaik there is no formal set of mnemonics for the CHIP-8 instruction set so it
ended up being custom but intuitive (and documented). see `instruction-set.txt`

to build/run:

```sh
cd disassembler
cargo build
# for example, fishie chip-8 rom
cargo run ../roms/fishie.ch8
```


emulator
--------

CHIP-8 has the following specifications:
- memory: direct access to up to 4kB of RAM starting at 0x200
- display: 64x32 pixels monochrome. the display buffer is in RAM at 0xF00
the display is redrawn when the emulator executes an instruction that modifies
the display dataO
- stack: for 16-bit addresses at 0xEA0
- 8-bit delay timer which is decremented at a rate of 60Hz until 0
- 8-bit sound timer which functions similar to the delay timer but beeps until 0
- one 16-bit index register "I" to point to locations in memory
- 16 8-bit general purpose registers V0-VF (VF is the flag reg)
- the index register and PC can only address 12-bits (4096 addresses)
- each font character is 4x5 pixels with sprite data representing numbers 0-F.
font is commonly placed at 0x050-0x09F but its fine anywhere 0x000-0x1FF
- keypad: 16 keys 0-F historically, but for QWERTY keyboards:
1234,QWER,ASDF,ZXCV
- timing: 700 instructions/second should be a good speed. this is obviously
different to the original hardware execution speed since the original CHIP-8
computer processors were clocked to around 1MHz

fundamentally, an emulator runs in an infinite loop: fetching an
instruction from memory at the current PC, decoding the instruction, and
executing the instruction.



to build/run:

```sh
cd emulator
cargo build
# to test
cargo run ../roms/chip8-logo.ch8
```


to do
-----

still buggy. it displays but the games dont really work
