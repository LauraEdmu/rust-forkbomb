# Forkbomb

## Description
This Rust program is designed to test system memory by continuously spawning new instances of itself, effectively creating a "forkbomb". **Warning: This software is experimental and intended for educational use only. Running it could cause your system to crash or require a hard reboot.**

## Usage
To run the program, use the following command:
```sh
cargo run
```
To bypass the warning message, use the `--bypass` argument:
```sh
cargo run -- --bypass
```

## Warning
This program will consume all available memory on your machine and may cause it to crash. Do not run this program on your machine unless you fully understand the implications. This program is only for educational purposes.

## License
This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Disclaimer
This software is provided for educational purposes only. The author is not responsible for any damage caused by the use of this software. Running this software may cause your system to become unresponsive and require a hard reboot. Use at your own risk. For more details, see the [DISCLAIMER](DISCLAIMER) file.