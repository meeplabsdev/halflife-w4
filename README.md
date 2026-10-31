# Music Player
|<img width="1724" height="930" alt="image" src="https://github.com/user-attachments/assets/52996239-98eb-4d28-8e13-63ebaf2dfe8f" />|<img width="1724" height="930" alt="image" src="https://github.com/user-attachments/assets/66ebaf65-c032-4b3e-949f-9da60a13e8d1" />|
|---|---|

This is a embedded music player, which is designed to be on a desk and display the current track + lyrics over spotify or bluetooth (or others platforms).

# What it is
This project was created for week 4 of [halflife](https://halflife.hackclub.com/)! I designed the schematic and PCB using [KiCad](https://www.kicad.org/), as a hat for the [Orange Pi Zero 3](http://www.orangepi.org/orangepiwiki/index.php/Orange_Pi_Zero_3), which runs embedded linux so I was able to write the firmware in a form much closer to a traditional desktop application (and it can even be tested off the device with the gpio changes printed to the terminal). Originally, I was planning to incorporate the display driver onto the PCB, but I ended up just using a driver board from adafruit since it worked out to be cheaper.

# Hardware
Schematic:

<img width="1069" height="673" alt="image" src="https://github.com/user-attachments/assets/e376a1a4-bb97-40b7-83c9-6ecbdea6c528" />

PCB:

|<img width="944" height="848" alt="image" src="https://github.com/user-attachments/assets/c6cf7d37-b69f-4ebc-8f1e-d124e346881c" />|<img width="943" height="849" alt="image" src="https://github.com/user-attachments/assets/7b892e4e-1729-462c-9c17-6b50e817c36d" />|
|---|---|

# Software
[`firmware/`](firmware/) contains a rust project which can control the board (using wiringpi to set gpio states) and a GUI for the screen. Currently, it is just demo firmware to show how the device could be controlled but when I build this, I could update the firmware to something more useful.

# Case
I also designed a case for this to hold the screen at a reasonable angle, and to keep the boards in the back neat. (red is the screen)

|<img width="783" height="543" alt="image" src="https://github.com/user-attachments/assets/693dd479-c7b4-4a00-b54f-9df18afde459" />|<img width="317" height="469" alt="image" src="https://github.com/user-attachments/assets/9a6c26fc-05a7-44e0-9ebf-e905892df574" />|
|---|---|


# Cost
This is a rough outline of the cost, see [`BOM.csv`](bom.csv) for more details.

| Part | Cost |
| --- | --- |
| [Screen](https://www.aliexpress.com/i/32772985029.html) | $16.92 |
| [Controller board](https://www.aliexpress.com/i/1005007586011837.html) | $33.00 |
| [Screen controller](https://thepihut.com/products/adafruit-tfp401-hdmi-dvi-decoder-to-40-pin-ttl-breakout-with-touch) | $38.38 |
| [HDMI adapter](https://www.aliexpress.com/item/1005006253497000.html) | $3.19 |
| [Power adapter](https://www.aliexpress.com/item/1005012406854301.html) | $4.05 |
| **Total + Shipping** | **$99.74** |
