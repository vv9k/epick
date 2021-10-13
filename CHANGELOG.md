# 0.5.0
- Add ability to spawn a window that will follow the cursor and display a zoomed in image with a pointer to a pixel (Only X11 and Windows)
- Make RGB and CMYK sliders adjust color when values change
- Add different display formats like: 'css rgb', 'css hsl' and previously available 'hex', 'hex uppercase'
- Fix hex input conversion to RGB
- Add settings to disable each colorspace [#13](https://github.com/vv9k/epick/pull/13)
- Updated light theme colors
- Add square colors
- Add monochromatic colors
- Rename `schemes` to `harmonies`
- CMYK sliders are now scaled from 0 ..= 100 [#14](https://github.com/vv9k/epick/pull/14)
- HSV sliders are now scaled 0..=360, 0..=100, 0..=100 respectively [#14](https://github.com/vv9k/epick/pull/14)
- HSL sliders are now scaled 0..=360, 0..=100, 0..=100 respectively [#14](https://github.com/vv9k/epick/pull/14)
- The triangle displaying current position on a slider will be displayed in the correct position [#14](https://github.com/vv9k/epick/pull/14)
- Add CIE Luv and CIE LCH(uv) sliders

# 0.4.0

- When moving the value slider of HSV or key slider of CMYK to edge values - 0 and 1 respectively, all other values would get reset to 0. Now the values are saved and when the value or key is brought back a little bit the values are restored
- Add HSL slider
- Windows opened by tabs like `hues`, `tints` or `shades` are now free to move around
- Monospaced text now correctly uses a custom font `FiraCode` for rendering
- Support for displaying color under the cursor (X11 and Windows supported for now)
- Added Windows and MacOS builds to GitHub releases
- Use a native-dialog when selecting the path to export the saved colors palette
