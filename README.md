# Pi-Power Button

A small utility to create a power button for your raspberry pi, written with Rust.

## Installation
Rust must be installed on your raspberry pi, then you can install this the power-button utility from this repo by cloning and running the following command.

```bash
cargo install --path .
```

At that point, you can then run the `install.sh` script.

```bash
sh install.sh
```

You also need a button connect to GPIO 3, and a ground. GPIO 3 is also set to automatically power on the raspberry pi by default.