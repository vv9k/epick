# 0.4.0

- When moving the value slider of HSV or key slider of CMYK to edge values - 0 and 1 respectively, all other values would get reset to 0. Now the values are saved and when the value or key is brought back a little bit the values are restored
- Add HSL slider
- Windows opened by tabs like `hues`, `tints` or `shades` are now free to move around
- Monospaced text now correctly uses a custom font `FiraCode` for rendering
- Support for displaying color under the cursor (X11 and Windows supported for now)
- Added Windows and MacOS builds to GitHub releases
- Use a native-dialog when selecting the path to export the saved colors palette
