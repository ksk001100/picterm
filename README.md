# Picterm

TUI image viewer

![](assets/picterm.gif)

## Install
```bash
$ cargo install picterm
```

or

```bash
$ git clone https://github.com/ksk001100/picterm
$ cd picterm
$ cargo install --path .
```

## Usage
```bash
$ picterm --help # => Show help
$ picterm -h
$ picterm # => Current directory
$ picterm ./
$ picterm $HOME/Downloads/
$ picterm ~/Pictures/sample.png
```

## Support file format
- PNG
- JPG
- WebP
- BMP
- GIF