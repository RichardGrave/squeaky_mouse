# Squeaky Mouse
## Auto mouse clicker

My kids play a lot of computer games, including Roblox.\
In some Roblox games you have to click a lot with the mouse buttons to do something.\
E.g. The Roblox game 'BIG Paintball!' where you have to click a lot with certain weapons.\
Since my youngest son cannot click that quickly and my oldest son has a muscle disease,\
I thought it would be fun to make an auto mouse clicker.\
Of course there are more auto clickers available on the internet,\
but I thought it would be fun to see if I could make one myself.

## Terminal app
This is a terminal app and I use MacOS myself\
So, I don't know if this works on other operating systems.

This application needs 1 or 2 arguments.\
If one number (in milliseconds) is specified then a mouse click is fired every X milliseconds.\
If two numbers (both in milliseconds) are used, a randomly selected number is used that lies between the specified values.

I've added randomly selected milliseconds so it's not too obvious that you're using an auto clicker.\
So if you want to use that, don't use numbers that are too close together.\
E.g. 150 and 175 is too close to each other and does not make much difference to the human eye.

## How to start
Examples:
* Auto click every 100 milliseconds:\
squeaky_mouse 100

* Auto click every randomly selected number between 150 and 500 milliseconds:\
squeaky_mouse 150 500

First number cannot be lower than 50 milliseconds.\
Second number? Well that depends on how long you'll want to wait for a next click.

## Default
Default the auto clicking is ENABLED
* LEFT mouse button auto click is ON
* RIGHT mouse button auto click is OFF
* MIDDLE mouse button auto click is OFF

## Usable keys
* END key quits the program
* ALT + (Num-5 or Numpad-5) turns ALL auto click ON/OFF
* ALT + (Num-1 or Numpad-1) toggle ON/OFF
* ALT + (Num-2 or Numpad-2) toggle ON/OFF
* ALT + (Num-3 or Numpad-3) toggle ON/OFF

## Dependencies
rand = "0.8.3"\
device_query = "0.2.8"\
enigo = "0.0.14"\
crossterm = "0.19.0"

## Issues
When you close the program the used keys are promted on the command line.
Not sure if I can find a way to fix this.