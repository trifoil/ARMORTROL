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