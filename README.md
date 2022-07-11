# ds4linux
A Sony Dualshock 4 driver for linux written in rust

## Why is this?
There are common drivers that work with the dualshock out of the box for linux, 
but they lacked the ability to finetune the sensitivity of the analog sticks. 
Also, I couldn't map the touchpad buttons in retroarch, which I usually map to start/select for most games because I really dislike the share/options buttons.

This was also a good opportunity to try my hands on some reverse engineering.

## Does it work?
Yes.

## Is it any good?
It depends on your definition of *good*. but no.

Longer answer:
* There is no slick gui like with the wonderful [ds4windows](https://github.com/Ryochan7/DS4Windows)
* All of the mappings/deadzones/curves etc. are hardcoded, unlike the easily customizable ds4windows
* There's no bluetooth support at the moment, unlike... yeah, you get the point
* I'm a complete beginner to making drivers
* I'm a complete beginner to interfacing with usb devices

Having said all that - and praised ds4windows for being everything this isn't - 
for my purposes it works as intended, and has surprisingly never actually crashed.


## On what distributions does this work on?
It's only been tested on Ubuntu >= 21.10.

## How do I use it?
At the moment, the only option is to build from source.

### Building from source

First, you need to [install rust](https://www.rust-lang.org/tools/install).

When that's done, download or clone this repo.

Navigate to the directory you cloned/unzipped to and run:

`cargo build --release`

When done, plug in your Dualshock, and then run:

`./target/release/ds4linux` (You can move this file wherever you want)

Your controller should now be connected, and its inputs being parsed by the driver. Hurray!

## Are there alternatives?
* [ds4windows](https://github.com/Ryochan7/DS4Windows) is great if you're a windows user.
* [ds4drv](https://github.com/chrippa/ds4drv) exists for linux, and has way more features. Although I've never tried it, it was a good reference while working on this project.
