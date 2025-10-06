# Hugo

Hugo is a blinkmojt (blinking gizmo) located above the door to Schäraton in IDét.

"Hugo" is pronounced [ʏˈgoː] (y'gå).

## Programmes

## `lib.rs`

A library for writing to Hugo's screen.
Currently, it uses [`rppal`](https://crates.io/crates/rppal) to write to the GPIO pins.

## Conway's game of life - `life.rs`

Runs Conway's game of life.
Once the state stabilises it will re-populate the screen with cells and continue running.

## Radar - `radar.rs`

Draws a sweeping wave that bounces on the left and right side of the screen.
The wave randomly spawns dots in it's wake that slowly fade away.

## Message Service - `udp.rs`

Listens on a port (default is 1337) for images and displays them on the screen.
