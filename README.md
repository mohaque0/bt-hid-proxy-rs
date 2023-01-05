This is mostly a rust implementation of the proxy service implemented here: https://github.com/Hacksore/bt-hid-proxy

The implementation is working but is not very well tested. This only implements the proxy service and not the usb gadget service. In the future it may include the usb gadget service and also n-key rollover. It is still a work in progress.

# References for N-Key Rollover

https://www.usb.org/sites/default/files/hid1_11.pdf

https://www.kernel.org/doc/html/latest/usb/gadget_hid.html

https://www.devever.net/~hl/usbnkro

https://emergent.unpythonic.net/01626210345

https://git.40percent.club/di0ib/tmk_keyboard_custom/commit/1ed336a06499c1baee2421141d59d115f0ee3c4b?lang=zh-HK

https://learn.adafruit.com/custom-hid-devices-in-circuitpython/n-key-rollover-nkro-hid-device
