# To start Rust embedded development with embassy


1. Clean up you rust installation (optional)

```bash
rustup self uninstall
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

3. Install Embedded Tools (flash & debug, etc)
```bash
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/probe-rs/probe-rs/releases/latest/download/probe-rs-tools-installer.sh | sh
```

which provides `cargo-embed`, `cargo-flash` and `probe-rs`

4. Install `cargo-embassy` to create new project templates based on embassy-rs

```bash
cargo install cargo-embassy
```

5. We may also need some udev rules:

Example File: 99-microbit.rules:
```
SUBSYSTEM=="usb", ATTR{idVendor}=="0d28", ATTR{idProduct}=="0204", MODE:="666"
SUBSYSTEM=="hidraw", MODE:="666"
```
and reload the rules with `sudo udevadm control --reload-rules && sudo udevadm trigger`

# Creating the project

Identify the proper chip, must be one of the listed in `probe-rs chip list` 

```bash
cargo embassy init my_project --chip nRF52833_xxAA
```

# Editing the project

Adjust `memory.x` file to match the current microcontroller


# Build and run
``` 
cargo run
```