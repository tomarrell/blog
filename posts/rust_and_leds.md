# Raspberry Wasps

Back while I was living in New Zealand, I had a fair number of electronics projects on the go. This was one such project.

I'd been playing around with a couple of LED matrices bought from AliExpress. The LED's on one of the matrices were the very popular **WS2812b** variant. These panels are made up of addressable RGB LED's soldered to a flexible PCB, giving you three wires out the back -- `data`, `5V`, and `ground`.

After not being able to find a Rust library which supported these things, I decided I would instead learn the protocol and write one myself. This is a brief overview of that experience.

## The plan
The original plan was to have this thing running from an ARM microcontroller. Specifically a spin-off of the Sparkfun Pro Micro, running an ARM Cortex M processor. The methods below would certainly work for in that case, but for ease of the toolchain and build process I decided to prototype this with an old Raspberry Pi I had sitting around.

The wiring to the panel itself it rather simple. A single `5V` conection and `ground` to a power supply, and then the data in pin hooked up to a digital out, SPI capable pin from the Raspberry Pi.

The pinout of the Pi looks like so:

![Raspberry Pi Pinout](/public/images/pi_pinout.png)

## WS2812b Data Protocol
The WS2812b runs a protocol that can nicely be mapped to Pulse Width Modulated (PWM) peaks with a frequency of 800,000Hz or 800Khz. We're able to encode a 1 using a duty cycle of approximately 66%, and a 0 with a duty cycle of approximately 33%.

Using this, we need to send a buffer of 24 bits which is consumed by the next pixel in line. The 24 bits encodes the red, green and blue hues for the consuming pixel. Once a pixel has consumed a buffer, it will forward any further bits along the chain until it receives a reset signal, where it will display its resulting hue.

The Raspberry Pi does have built in hardware PWM. However I wanted to make it a little more difficult than simply relying on hardware implementation. So bit-banging it was.

## Bit Banging
Bit banging is the process of handling all aspects of serial communication in software rather than dedicated hardware. This is really only possible when you have complete control over what is running on the processor. Timing is incredibly important, and if you're managing clock signals in software and suddenly the OS jumps in to handle an interrupt, your program is probably going to miss its slot.

Experimenting with bit banging on the Raspberry Pi to the WS2812bs made it pretty evident. Trying to bit bang from user space on top of the Linux Kernel, even without a garbage collector in your binary, is just not feasible. The panel would spark to life at times, giving a pretty garbled display of colors from time to time, but its lack of precision timing meant that another solution had to be found for this experiment.

## SPI?
So with bit banging off the table, I had a look to another relatively simple serial protocol (...interface), [SPI](https://en.wikipedia.org/wiki/Serial_Peripheral_Interface). SPI is a chainable serial protocol, who's bus specifies 4 logic signals:

- SCLK: the clock signal
- MOSI: master ouput, slave input
- MISO: master input, slace ouput
- SS: slave select

Now this is relatively.

The LEDs run with a scan frequency of no less than `400Hz`, with a data transfer rate of `800kbps`.

The traditional method of communication with the WS2812b is usually PWM. The duty cycle to send a single bit is:

| Bit  | Time High | Time Low | Duty % |
| ---- | --------- | -------- | ------ |
| 0    | 0.4us     | 0.85us   | 32%    |
| 1    | 0.8us     | 0.45us   | 64%    |

A visual of the protocol is below.

```
PWM LED (0): where
    on time  = 0.4us +/-150ns
    off time = 0.85us +/-150ns
    
Volts
    |    START                      END
    |    v                          v
    |    ----------
    |    |        |
    |    |        |
    |    |        |
    | ----        -------------------
    *-------------------------------- Time
    
PWM LED (1): where
   on time  = 0.8us +/-150ns
   off time = 0.45us +/-150ns
   
Volts
    |    START                      END
    |    v                          v
    |    ------------------
    |    |                |
    |    |                |
    |    |                |
    | ----                -----------
    *-------------------------------- Time
```

Looking at the above duty cycle lengths, we can approximate each time slice into another 3 periods, each **33%** of the original wavelength in order to have constant high/low runs for the given period. This will allow us to continue using SPI, using a clock speed of approximately `3Mhz`.

To interact over SPI, each traditional LED sequence (3 bytes, 8 bits per colour channel) needs to be converted to a 9 byte sequence (72 bits per colour channel) before being shifted over SPI. Each traditional PWM 'bit' gets turned into an equivalent 3 SPI bits, and the clock speed of the SPI interface set to achieve within the `+/-150ns` error margins.

With the traditional period broken into 3 time sections, each can be pulled high or low individually. We can therefore represent what would have been a PWM (1) with an SPI `110`. And on the contrary, represent a PWM (0) with an SPI `100`.

## Example
An example of a Raspberry Pi controlling a flexible panel of WS2812b's:

![Image of Pixel Mario](https://imgur.com/14oSYYy.jpg)

Power WS2812b LEDs from your Raspberry Pi over SPI. This library leverages the Raspberry Pi's SPI interface through Linux's IOCTL sys calls in order to emulate the data transfer method required by the WS2812b.

**Note:** This is currently a heavy WIP. It is almost certainly unstable, and is subject to breaking changes.
