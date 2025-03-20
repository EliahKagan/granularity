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
- no current `output-macos.txt` for macOS 15.3.2, but it is forthcoming

## History and acknowledgements

This was orignally made while investigating
[GitoxideLabs/gitoxide#1896](https://github.com/GitoxideLabs/gitoxide/issues/1896).

The change of including acquision and release of the temporary directory (the
latter of which deletes the directory and the files created inside of it) in
the per-experiment timings is motivated by the insight of
[**@Byron**](https://github.com/Byron) expressed in
[GitoxideLabs/gitoxide#1899 (comment)](https://github.com/GitoxideLabs/gitoxide/pull/1899#discussion_r2004613319).

## License

[0BSD](LICENSE)
