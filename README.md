# note
A Lightweight and simple CLI notepad written in Rust.

With this project I'm learning Rust.
The project builds and installs using make and works well on Windows.

## Build/Install

Use standard cargo commands for build in case of just play with it.

Use `make install` to install app globally - then app will be available through `note [PARAMS]` cmd in your win terminal/ps. (you need install `make` if you don't have yet)

App installs in: `C:\Users\[USERNAME]\.cargo\bin`
At the moment installed app saves records within `C:\Users\[USERNAME]\.cargo\notes` - I will change it in the future.

## Commands

- `note` - interactive ^^. You can pass your note or 
- `note ls` - List of your "today" notes 
- `note "Your sample thoughts"` - Takes your note to the black hole of notes.

## TODO

- learn testing within Rust
- rebuild architecture
- focus on better hermetization