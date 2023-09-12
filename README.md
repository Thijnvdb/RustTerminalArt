# Terminal art from image generator
Generate terminal art from image.

NOTE: color support is currently limited to terminal emulators supporting Truecolor or ansi256

```
termart 0.4.0

Usage: termart [OPTIONS] <FILE>

Arguments:
  <FILE>  Input file to read

Options:
  -m, --mode <MODE>                  Print mode to use [possible values: ascii, ascii-colored, pixels]
  -c, --color-format <COLOR_FORMAT>  Color format to use [possible values: truecolor, ansi256]
  -e, --export-dir <EXPORT_DIR>      Directory to export to
  -s, --size <SIZE>                  Max size for width/height
      --contrast <CONTRAST>          Value to change contrast by (negative values are allowed)
      --hue <HUE>                    Value to change the hue by (negative values are allowed)
      --brightness <BRIGHTNESS>      Value to change the brightnes by (negative values are allowed)
  -h, --help                         Print help
  -V, --version                      Print version
```
