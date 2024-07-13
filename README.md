# pywal-spicetify
A simple cli tool for applying wal generated colors to spicetify. I built this purely for personal use. <br>
I haven't done any test but it works on my machine... <br>
Built in rust btw
## Demo
~[Demo](./assets/demo.gif)

## Installation
From the AUR
```bash
yay -S pywal-spicetify
```
From Source
```bash
git clone https://github.com/jhideki/pywal-spicetify
cargo build --release
cp target/release/bin/pywal-spicetify <wherever you want to save the progra>
export PATH=$PATH:<where you saved pywal-spicetify>
```
## Usage
```bash
Usage: pywal-spicetify [OPTIONS] <theme>

Arguments:
  <theme>

Options:
  -r, --reset
  -h, --help     Print help
  -V, --version  Print version
```
## Example
pywal-spicetify is meant to be called whenever you change wallpapers. Here is how it is set up with `waypaper` <br>
```bash
[Settings]
language = en
folder = ~/dotfiles/wallpapers
wallpaper = ~/dotfiles/wallpapers/wp5089612-scenery-anime-wallpapers.jpg
post_command = wal -i $wallpaper && pywal-spicetify text && killall -SIGUSR2 waybar
```
## Configuration
Colors can be configured from `.confiig/wal/templates/colors-spicetify.ini` <br>
`colors-spicetify.ini` will be auto-generated upon running `pywal-spicetify` or the first time.
```bash
accent             = {color0.strip}
accent-active      = {color2.strip}
accent-inactive    = {color3.strip}
banner             = {color4.strip}
border-active      = {foreground.strip}
border-inactive    = {foreground.strip}
header             = {foreground.strip}
highlight          = {color6.strip}
main               = {background.strip}
notification       = {color7.strip}
notification-error = {color8.strip}
subtext            = {cursor.strip}
text               = {cursor.strip}
```
