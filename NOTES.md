# To start Rust embedded development with embassy


1. Clean up you rust installation (optional)
rustup self uninstall

2. Install Rust again (not really needed)
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

3. Install Embedded Tools (flash, debug, ect)
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/probe-rs/probe-rs/releases/latest/download/probe-rs-tools-installer.sh | sh

which provides `cargo-embed`, `cargo-flash` and `probe-rs`

4. Install cargo-embassy to create new project templates based on embassy-rs

```bash
cargo install cargo-embassy
```

5. We also need some udev rules:

File 99-microbit.rules:
```
SUBSYSTEM=="usb", ATTR{idVendor}=="0d28", ATTR{idProduct}=="0204", MODE:="666"
SUBSYSTEM=="hidraw", MODE:="666"
```
and reload the rules
```bash
sudo udevadm control --reload-rules && sudo udevadm trigger
```

# Creating the project

```bash
cargo embassy init my_project --chip nRF52833_xxAA
```