# Useful Commands

## Simulating (new)

```cmd
pip3 install -r C:\deldir\1124\Renode_RP2040\visualization\requirements.txt
renode --console run_dua_blinka.rescs
```

## Simulating (old)

Get the main branch of Renode 2040.

```cmd
cd tests
renode
include @run_dua_blinka.resc
start
sysbus.gpio.button Press
sysbus.gpio.button Release
i @tap.resc
quit

```
