# Picterm

[![crates.io](https://img.shields.io/crates/v/picterm.svg)](https://crates.io/crates/picterm)
![releases count](https://img.shields.io/github/release/ksk001100/picterm.svg)
![download count](https://img.shields.io/crates/d/picterm)
![issues count](https://img.shields.io/github/issues/ksk001100/picterm.svg)
![forks count](https://img.shields.io/github/forks/ksk001100/picterm.svg)
![license](https://img.shields.io/github/license/ksk001100/picterm.svg)
![github actions CI](https://github.com/ksk001100/picterm/workflows/CI/badge.svg?branch=main)

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
or

Download [here](https://github.com/ksk001100/picterm/releases)

## Usage
```bash
$ picterm --help # => Show help
$ picterm -h
$ picterm # => Current directory
$ picterm ./
$ picterm $HOME/Downloads/
$ picterm ~/Pictures/sample.png
$ picterm ~/Pictures/sample.png --gray # => Gray scale mode
$ picterm ~/Pictures/ -g # => Gray scale mode
```

## Support file format
- PNG
- JPG
- WebP
- BMP
- GIF
