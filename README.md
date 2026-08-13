<h1 align=center><code>Shaper</code></h1>

<hr />

<p align="center">
    <a href="#about">About</a> ·
    <a href="#installation">Installation</a> ·
    <a href="#quick-start">Quick Start</a> ·
    <a href="#thanks">Thanks</a>
</p>

<hr />

## About
**Shaper** is a small and simple program for managing templates.
It lets you organise your file templates all in one place and instantly bring them into your working directory.

### Video Demo
https://github.com/user-attachments/assets/318e70b0-b7e8-4d42-b446-23a0fc497e67

## Installation
Precompiled binaries are published on [GitHub Releases](placeholder).

### Building from source
```bash
git clone https://github.com/notseriouss/shaper.git && cd shaper
cargo build --release
```

### Nix
> Repository contains flake.nix

## Quick Start
> NOTE:
> **Shaper** only modifies your file system when you **applying** the templates, hence, you would need to setup the config directory with all the necessary templates yourself.

1. **Create the templates directory**:
```bash
mkdir -p ~/.config/shaper/templates
```
> Any folder inside becomes a template

2. **Create your first template**
```bash
mkdir ~/.config/shaper/templates/my-shell
echo '#!/bin/bash' > ~/.config/shaper/templates/my-shell/script.sh # example
```
> Now, just put all the files/folders you want the **Shaper** to bring when you will apply the template

3. **Create config.toml and [[template]] entry**
```bash
touch ~/.config/shaper/config.toml
```
> Example of the configuration file can be found [here](#configuration)

4. **Apply the template**
```bash
shaper apply my-shell
```
> Your working directory now contains `script.sh`.

## Usage
**Shaper** operates in 2 modes, cli and tui

* TUI:
> Runs when no commands passed, lets you quickly choose templates.
> Templates are shown in a virtual hierarchy cunstructed from each template groups.

* CLI:
```bash
Template manager

Usage: shaper [COMMAND]

Commands:
  apply  Apply specific template(s)
  list   List all defined templates
  help   Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```

## Configuration
> NOTE:
> **Shaper** looks for its config.toml location as follows:
> looking at the full path provided with: $SHAPER_CONFIG_PATH 
> or at the default location: $HOME/.config/shaper/config.toml

```toml
# required variable, it simply points to the directory where we store the templates
templates_dir = "/home/user/.config/shaper/templates"

# optional variable, sets the default option for dryrun when applying the template, can be overridden through cli
dryrun = false # shows the templates that would be applied without applying them

# optional variable, sets the default option for overwrite when applying the template, can be overridden through cli
overwrite = false # if true, overwrites files with the same name as in template folder

[[template]]
# required variable, sets a unique folder name of the template inside templates_dir
folder = "my-shell"
# optional variable, falls back to the folder name
name = "My shell"
# optional variable, falls back to "No description"
description = "Example shell skeleton"
# optional variable, used for creating virtual "groups" inside TUI for separating templates
groups = ["examples", "shells"] # in TUI, my-shell template will be located under examples/shells/[[template]], if some other template uses the same groups, it will put the templates together
```

## Thanks
* [Rust](https://github.com/rust-lang/rust), its [std](https://github.com/rust-lang/rust/tree/main/library) library and community
* [crossterm](https://github.com/crossterm-rs/crossterm)
* [serde](https://github.com/serde-rs/serde)
* [clap](https://github.com/clap-rs/clap)
* to everyone who is not explicitly mentioned here, but without whom this repository would not exist
