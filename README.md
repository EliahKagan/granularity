# granularity - A file timestamp granularity experiment

This looks at how often timestamps for files created close together cluster
into equal values in a `/tmp`-like directory. The granularity of the filesystem
timestamps contributes to this.

The effects vary across systems. In practice, on macOS, all the timestamps are
nearly always distinct, while on Windows and GNU/Linux that is not usually the
case. Whether that is really what happens will depend in part on what kind of
device and filesystem `/tmp` (or an analogous directory on Windows) is located
on.

## Contents

The code is in [`src/main.rs`](src/main.rs). Results are in:

- [`output-ubuntu.txt`](output-ubuntu.txt) for Ubuntu 24.04 LTS (GNU/Linux)
- [`output-windows.txt`](output-windows.txt) for Windows 10 22H2
- [`output-macos.txt`](output-macos.txt) for macOS 15.3.2

## License

[0BSD](LICENSE)
