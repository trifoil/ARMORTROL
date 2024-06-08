# python-tank
repo for python tank remote testing

## Flashing and installing on the rock64


```
sudo arm-image-installer  --image=/home/x/Downloads/Fedora-Server-40-1.14.aarch64.raw.xz --target=rock64-rk3328 --media=/dev/sdd --norootpass
```

## Get sh installation script

```
wget https://raw.githubusercontent.com/trifoil/python-tank/main/install.sh
```

## Python venv

python3.12 -m venv pute
source pute/bin/activate

## Software segmentation

### Function 1 
Written test
### Function 2
Wheel input correction
Uncorrected wheel input (0 -> 255) -> corrected wheel input (-255 -> 255) 
### Function 3
Zero turn 
corrected wheel input, accelerator input, reverse input -> left_track_motor, right_track_motor
