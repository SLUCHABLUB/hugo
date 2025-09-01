# Hugo

Hugo is a blinkmojt (blinking gizmo) located above the door to Schäraton in IDét.

Hugo is prunounced /y.go/ (y’gå).

## libhugo

A wrapper library for [WiringPi Library](https://github.com/WiringPi/WiringPi)
that gives a reasonable API for working with this specific screen.

## life

Runs game of life. Once the state stabalizes it will re-populate the screen
with cells and continue running.

## radar

Draws a sweeping wave that bounces on the left and right side of the screen.
The wave randomly spawns dots in it's wake that slowly fade away.

## udp

Listens on a given port (1337 currently) for images and displays them on the screen.
