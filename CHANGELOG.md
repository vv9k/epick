# 0.6.0
- Add support for persistent configuration. A configuration file with a YAML syntax will be loaded on startup from appropriate config directory depending on OS.
  Supported options are the same as in settings window. The settings window now contains a `save settings` button.
- Persist colors throughout launches. On exit all current colors will be saved to a cache file and loaded back on startup.
- Add option to disable color caching
- Add keyboard shortcuts to display zoomed window, pick and save the color under the cursor
- Fix color rendering
- Add display picker on macOS
- Add shortcut to toggle side panel
- Add a help window with keybindings
- Fix circle of zoomed display picker on X11, the circle will now correctly point to a single pixel rather than four at a time.
- Add setting to set default color harmony
- Add 2D slider to HSV
- Add persistent colors and settings to WASM by using local browser storage
- All windows will now open in the empty area after sliders so that nothing is hidden
- Use different cursor when hovering over different elements
- Add custom color format support, you can now fully customise the way the colors next to color boxes are displayed.
  A simple formatting language has been introduced that has a syntax simillar to Rust's format macros.

# 0.5.1
- Fix switching between working spaces and illuminants

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
- Add setting to select RGB working space
- Add CIE Lab and CIE LCH(ab) sliders
- The sidebar and topbar icons are now on the right side
- Fix invisible pointer on slider when using light theme
- Shades, tints and hues can now all be open at the same time
- Add settings to change the reference white illuminant
- The harmonies section is now on the top
- Fix sliders behaviour in the 0.0 ..= 1.0 range

# 0.4.0

- When moving the value slider of HSV or key slider of CMYK to edge values - 0 and 1 respectively, all other values would get reset to 0. Now the values are saved and when the value or key is brought back a little bit the values are restored
- Add HSL slider
- Windows opened by tabs like `hues`, `tints` or `shades` are now free to move around
- Monospaced text now correctly uses a custom font `FiraCode` for rendering
- Support for displaying color under the cursor (X11 and Windows supported for now)
- Added Windows and MacOS builds to GitHub releases
- Use a native-dialog when selecting the path to export the saved colors palette
