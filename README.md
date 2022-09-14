<p align="center">
    <img src="./assets/icon.png">
</p>

# epick

[![Build Status](https://github.com/vv9k/epick/workflows/epick%20CI/badge.svg)](https://github.com/vv9k/epick/actions?query=workflow%3A%22epick+CI%22)

Color picker for creating harmonic color palettes that works on Linux, Windows, macOS and web.

## Get it

You can checkout the web demo over [here](https://vv9k.github.io/epick/) or get a native binary from the [GitHub release page](https://github.com/vv9k/epick/releases).

If you happen to use Arch Linux you can grab **epick** from [AUR](https://aur.archlinux.org/packages/epick/) using your favorite package manager:
```bash
$ paru -S epick
```

## Build

Install required libraries (only required on Linux):
```
$ apt-get install libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev libxkbcommon-dev
```

To manually build **epick** you'll need the latest Rust with `cargo`. To build the project run:
```
$ make
```

To start the web version locally run:
```
$ make start_web
```
This will build the WASM files and start a simple http server listening at `127.0.0.1:8080`.

To build without `make` checkout the build instructions in the `Makefile`

## Demo

To checkout the latest build of master branch head over to the [web demo](https://vv9k.github.io/epick).

## Keyboard shortcuts

Here are some handy shortcuts to enhance the usage of **epick**:
- Only supported with screen picker:
   - `p` to pick a color from under the cursor
   - `s` to save a color from under the cursor
- Other:
   - `h` toggle side panel

## Custom color format

To fully customize the way the colors are presented textually on the screen and the way they are copied to clipboard, a
simple formatting language has been introduced that resembles the syntax of rust formatting macros. Here is a simple
example of such a formatting string:
```
"{r} {g} {b}"
// Same as
"{ r } {  g} {    b     }"
```

Above example shows how to display a value of a field. The name of the field is enclosed in curly braces and can contain
multiple white space characters following the opening brace and preceeding the closing brace. This format string will
print red, green and blue values of the color in the 0.0 ..= 1.0 range.

To specify the precision of a floating point number:
```
"{r:.2} {g:.0} {b:.4}"
```

To display a value in decimal, octal, hex or uppercase hex respectively:
```
"{lab_l:d} {r255:o} {g255:x} {b255:X}"
```
Using this flags on floating values will automatically truncate the fractional part and treat the number as an integer.

### Supported color fields:
| Field       | Color value    | Value range      |
|-------------|----------------|------------------|
| `r`         | Red            | 0.0 ..= 1.0      |
| `g`         | Green          | 0.0 ..= 1.0      |
| `b`         | Blue           | 0.0 ..= 1.0      |
| `r255`      | Red            | 0 ..= 255        |
| `g255`      | Green          | 0 ..= 255        |
| `b255`      | Blue           | 0 ..= 255        |
| `cmyk_c`    | Cyan           | 0.0 ..= 1.0      |
| `cmyk_m`    | Magenta        | 0.0 ..= 1.0      |
| `cmyk_y`    | Yellow         | 0.0 ..= 1.0      |
| `cmyk_k`    | Key            | 0.0 ..= 1.0      |
| `cmyk_c100` | Cyan           | 0.0 ..= 100.0    |
| `cmyk_m100` | Magenta        | 0.0 ..= 100.0    |
| `cmyk_y100` | Yellow         | 0.0 ..= 100.0    |
| `cmyk_k100` | Key            | 0.0 ..= 100.0    |
| `hsl_h`     | HSL Hue        | 0.0 ..= 1.0      |
| `hsl_s`     | HSL Saturation | 0.0 ..= 1.0      |
| `hsl_l`     | HSL Light      | 0.0 ..= 1.0      |
| `hsl_h360`  | HSL Hue        | 0.0 ..= 360.0    |
| `hsl_s100`  | HSL Saturation | 0.0 ..= 100.0    |
| `hsl_l100`  | HSL Light      | 0.0 ..= 100.0    |
| `hsv_h`     | HSV Hue        | 0.0 ..= 1.0      |
| `hsv_s`     | HSV Saturation | 0.0 ..= 1.0      |
| `hsv_v`     | HSV Value      | 0.0 ..= 1.0      |
| `hsv_h360`  | HSV Hue        | 0.0 ..= 360.0    |
| `hsv_s100`  | HSV Saturation | 0.0 ..= 100.0    |
| `hsv_v100`  | HSV Value      | 0.0 ..= 100.0    |
| `lab_l`     | Lab Light      | 0.0 ..= 100.0    |
| `lab_a`     | Lab a          | -127.0 ..= 128.0 |
| `lab_b`     | Lab b          | -127.0 ..= 128.0 |
| `luv_l`     | Luv Light      | 0.0 ..= 100.0    |
| `luv_u`     | Luv u          | -134.0 ..= 220.0 |
| `luv_v`     | Luv v          | -140.0 ..= 122.0 |
| `lch_ab_l`  | LCH(ab) Light  | 0.0 ..= 100.0    |
| `lch_ab_c`  | LCH(ab) Chroma | 0.0 ..= 270.0    |
| `lch_ab_h`  | LCH(ab) Hue    | 0.0 ..= 360.0    |
| `lch_uv_l`  | LCH(uv) Light  | 0.0 ..= 100.0    |
| `lch_uv_c`  | LCH(uv) Chroma | 0.0 ..= 270.0    |
| `lch_uv_h`  | LCH(uv) Hue    | 0.0 ..= 360.0    |
| `xyy_x`     | xyY x          |                  |
| `xyy_y`     | xyY y          |                  |
| `xyy_Y`     | xyY Y          |                  |
| `xyz_x`     | XYZ X          |                  |
| `xyz_y`     | XYZ Y          |                  |
| `xyz_z`     | XYZ Z          |                  |

## License
[GPLv3](https://github.com/vv9k/epick/blob/master/LICENSE)
