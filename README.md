# ALT Linux Package Comparator

This CLI tool compares binary packages between branches of the ALT Linux distribution.

## Installation

To install ALT Linux Package Comparator, ensure you have Rust and Cargo installed, then run:

```bash
git clone https://github.com/kkozoriz/altlinux_package_comparator.git
```

```bash
cd altlinux_package_comparator
```

### Required files for Alt Linux 10

Testing of the application was done using `docker`, the application uses the following dependencies:
```bash
ldd /target/release/pkg-cmp
```

Output of `ldd /target/release/pkg-cmp`:
```
linux-vdso.so.1 (0x0000ffff977a7000)
libssl.so.1.1 => /lib64/libssl.so.1.1 (0x0000ffff96d63000)
libcrypto.so.1.1 => /lib64/libcrypto.so.1.1 (0x0000ffff96a00000)
libgcc_s.so.1 => /lib64/libgcc_s.so.1 (0x0000ffff97750000)
libpthread.so.0 => /lib64/libpthread.so.0 (0x0000ffff97720000)
libm.so.6 => /lib64/libm.so.6 (0x0000ffff96cb8000)
libdl.so.2 => /lib64/libdl.so.2 (0x0000ffff9770c000)
libc.so.6 => /lib64/libc.so.6 (0x0000ffff96889000)
/lib64/ld-linux-aarch64.so.1 (0x0000ffff97775000)
libz.so.1 => /lib64/libz.so.1 (0x0000ffff9685e000)
```

Therefore, in order for the software to run on `Alt Linux version 10`, the following packages must be installed, if they are not present:
```bash
 sudo apt-get update
```

```bash
 sudo apt-get install -y libssl-devel
```

```bash
 sudo apt-get install -y gcc
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
```

```bash
./target/release/pkg-cmp -o output.json first-branch-only
```

```bash
./target/release/pkg-cmp -o out.txt p11 p10 second-branch-only
```

## Output

You can put the option `-o` or `--output-file`, and then write the path to the file where you want to write the result of the command, if there is no existing file, a new file will be created, for example:
```bash
./target/release/pkg-cmp -o out.txt first-branch-only
```

For example, let's print the first 10 lines of the resulting file to the console:

```bash
head out.txt
```

Next, you can see the first 10 lines of output

```bash
Package { name: "lact", version: "0.5.5", release: "alt1", arch: "aarch64" }
Package { name: "i586-drgn", version: "0.0.27", release: "alt1", arch: "x86_64-i586" }
Package { name: "python3-module-rtmixer-debuginfo", version: "0.1.4", release: "alt1", arch: "aarch64" }
Package { name: "libboost_python3-1.85.0-debuginfo", version: "1.85.0", release: "alt3", arch: "aarch64" }
Package { name: "i586-qca-qt6-cyrus-sasl", version: "2.3.8", release: "alt1", arch: "x86_64-i586" }
Package { name: "Lfunction-debuginfo", version: "2.0.5", release: "alt1", arch: "aarch64" }
Package { name: "i586-libgupnp1.6", version: "1.6.6", release: "alt1.1", arch: "x86_64-i586" }
Package { name: "i586-libspiro1", version: "20221101", release: "alt1_1", arch: "x86_64-i586" }
Package { name: "i586-baresip-vp9", version: "3.14.0", release: "alt1", arch: "x86_64-i586" }
Package { name: "hyprwayland-scanner", version: "0.4.0", release: "alt1", arch: "aarch64" }
```

Another example of the resulting file in .json format
```bash
./target/release/pkg-cmp --arch x86_64 --output-file out.json sisyphus-newer
```
If you do not specify a flag, the output will be output to the console
