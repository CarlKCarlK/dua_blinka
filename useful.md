# Useful Commands

## Simulating

Get the main branch of Renode 2040.

```cmd
cd tests
renode
include @run_blinky.resc
start
sysbus.gpio.button Press
sysbus.gpio.button Release
i @tap.resc
quit

```
