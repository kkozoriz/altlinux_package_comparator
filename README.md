# ALT Linux Package Comparator

This CLI tool compares binary packages between `sisyphus` and `p10` branches of the ALT Linux distribution.

## Installation

to install ALT Linux Package Comparator, ensure you have Rust and Cargo installed, then run:
```bash
git clone https://github.com/kkozoriz/altlinux_package_comparator.git
cd altlinux_package_comparator
```

```bash
cargo build --release
```

## Usage

To find out more information about usage, enter:
```bash
./target/release/pkg-cmp --help
```

# Examples

```bash
./target/release/pkg-cmp -a x86_64 -o output.json first-branch-only

./target/release/pkg-cmp -o output.json first-branch-only


```

## Output
