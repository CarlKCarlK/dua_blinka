# Useful Commands

## Emulation (new) cmk update

```cmd
python3 -m pip install -r C:\deldir\1124\Renode_RP2040\visualization\requirements.txt
cd tests
renode --console run_dua_blinka.resc
s
```

## Emulation (old)

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
