# epick

[![Build Status](https://github.com/vv9k/epick/workflows/epick%20CI/badge.svg)](https://github.com/vv9k/epick/actions?query=workflow%3A%22epick+CI%22)

Simple color picker that lets the user create harmonic palettes with ease.

## Get it

You can checkout the web demo over [here](https://vv9k.github.io/epick/) or get a native binary from the [GitHub release page](https://github.com/vv9k/epick/releases).

If you happen to use Arch Linux you can grab **epick** from AUR using your favorite package manager:
```bash
$ paru -S epick
```

## Build

Install required libraries:
```
$ apt-get install libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev libxkbcommon-dev
```

To manually build **epick** you'll need the latest Rust with `cargo`. The build command is:
```
$ cargo build --release
```

## Demo

![Demo GIF](https://github.com/vv9k/epick/blob/master/assets/epick.gif)

## Keyboard shortcuts

Here are some handy shortcuts to enhance the usage of **epick**:
 - Only supported with screen picker:
   - `z` to display zoomed window
   - `p` to pick a color from under the cursor
   - `s` to save a color from under the cursor

## License
[GPLv3](https://github.com/vv9k/epick/blob/master/LICENSE)
