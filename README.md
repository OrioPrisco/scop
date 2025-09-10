# Scop

Basic .onj viewer written in rust, a 42 project.

## Compiling

just `cargo build`  
A Makefile is included to comply with the 42 subject, this makefile uses a docker container to compile the project using a more up to date version of the rust toolchain than what is installed by default on the 42 computers. Please do not use it.

## Running

`cargo run <obj_file>`  
ex : `cargo run objs/42.obj`  
  
You can add `--no-ignore-unimplemented` as a second argument (yes it is positionally dependant, I didn't bother with actual argument parsing)  
It will abort the program if the obj contains an entry type not supported by my simple parser

## Controls

Movement is basically Minecraft creative mode
* W/S : Forward/Backward
* A/D : Left/Right
* Space/Shift : Up/Down
* Numpad+/Numpad- : Scale model
* C: Toggle between gray faces and colorful texture + light
* Esc: Quit

## Licensing

The `objs/` folder is a collection of random models my program can load that i found on the internet, they're not mine  
Everyhting else was written by me, while loosely following <https://learnopengl.com>
