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

Informations transferred :

* Multiple video flux
* Multiple audio flux
* Multiple command flux
* Multiple feedback flux

User interface :

* ICED

Imputs library :

* 

Other libraries :

*

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

## Subroutines

input 
* accelerator 0 -> +1
* steering wheel -1 -> +1
* brake 0/1
* direction 0/1

output
* tank config
    * throttle left -1 -> +1
    * throttle right -1 -> +1
* car config
    * throttle -1 -> +1
    * direction -1 -> +1

input 
* tank config
    * throttle left -1 -> +1
    * throttle right -1 -> +1
* car config
    * throttle -1 -> +1
    * direction -1 -> +1

output 
* tank config
    * throttle left 0 -> +255
    * throttle right 0 -> +255
* car config
    * throttle 0 -> +255
    * direction 0 -> +255


### functions 
* input_2_tank()
* input_2car() no need
* unit_2_arduino()


## Modules 

### config
### inputs
### networking
### subroutines 
### 

## Sources
https://github.com/dionysus-oss/netrusting
https://docs.rs/clap/latest/clap/_derive/_tutorial/index.html#modules
https://www.youtube.com/playlist?list=PL5E0b3rgRMdpDBdw56w7tNSZgVt2dysNF
https://leafheap.com/articles/iced-tutorial-version-0-12

Rocket M5 M2
https://store.ui.com/us/en/pro/category/all-wireless/products/rocketm2

