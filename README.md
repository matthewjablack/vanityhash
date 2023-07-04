# Vanity Hash Miner

A simple tool to mine vanity hashes. Quick and dirty code.

## Compilation

```
git clone https://github.com/matthewjablack/vanityhash.git
cd vanityhash
cargo build --release
```

## Usage

Generating vanity hashes with the most leading 0x69 bytes

Note: converts decimal to hex (hex value 69 = decimal value 105)

```
./vanityhash 105 1
```

## Inspiration

https://twitter.com/SachinMeier/status/1674250697687150592
