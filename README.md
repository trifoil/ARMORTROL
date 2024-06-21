# ARMORTROL

Acronym for ARMOR-CONTROL

## Design brief

Operating modes :

* Client-Server-Vehicle
* Client-Vehicle (where vehicle has the Server role)

Security : 

* TLS over TCP 
* SELinux

SBC used :

* rock64 or better, supporting Fedora Linux
* other SBC, with Armbian

Informations transferred

* Multiple video flux
* Multiple audio flux
* Multiple command flux
* Multiple feedback flux
 
## Detailed operation modes description

### Client-Server-Vehicle

### Client-Vehicle (where vehicle has the Server role)

## Software structure

Using as much functions as possible, in a well organized tree.
The main function has to be the shortest possible for every software, for clarity reasons.

## Flashing and installing on the rock64

```
sudo arm-image-installer  --image=/home/x/Downloads/Fedora-Server-40-1.14.aarch64.raw.xz --target=rock64-rk3328 --media=/dev/sdd --norootpass
```

## Dependancies for compiling on Fedora Linux

pour la dépendance avec joystick :
```
sudo dnf install -y systemd-devel
```

## Get sh installation script

```
wget https://raw.githubusercontent.com/trifoil/python-tank/main/install.sh
```