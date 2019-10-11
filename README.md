
[![](https://github.com/AlexEne/rust-chip8/workflows/Build/badge.svg)](https://github.com/AlexEne/rust-chip8/actions)

# rust-chip8
Chip8 emulator in rust

This is written entirely on stream, recordings are available here:

https://www.youtube.com/playlist?list=PLPv6awLpSB_diKBY_95ip0jL1uT7i_rJP

## How to play the games:

The entire chip8 keyboard is mapped like this:

Chip8 keyboard:

| | | | |
|-|-|-|-|
|1|2|3|C|
|4|5|6|D|
|7|8|9|E|
|A|0|B|F|

Is mapped to: 

| | | | |
|-|-|-|-|
|1|2|3|4|
|Q|W|E|R|
|A|S|D|F|
|Z|X|C|V|


### Tetris controls:
* Q - Rotate piece
* W - Move left
* E - Move right
* A - Fast drop

### Invaders controls:
* Q - Move left
* E - Move right
* W - Shoot weapon
