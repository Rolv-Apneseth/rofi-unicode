<h1 align="center">rofi-unicode</h1>

<p align="center">Unicode selector plugin for rofi</p>

<p align="center">
  <img src="https://img.shields.io/badge/License-AGPL_v3-green.svg" alt="License: AGPL v3" />
  <img src="https://img.shields.io/github/v/tag/rolv-apneseth/rofi-unicode?label=version&color=blueviolet" alt="version" />
  <a href="https://aur.archlinux.org/packages/rofi-unicode"><img src="https://img.shields.io/aur/version/rofi-unicode" alt="AUR version" /></a>
</p>

![rofi-unicode - demo GIF](https://github.com/user-attachments/assets/e6ac8573-ce34-4ac9-a47a-168a6e982caf)

## Dependencies

### Wayland

- [wl-clipboard](https://github.com/bugaevc/wl-clipboard)
- [wtype](https://github.com/atx/wtype)

---

### X11

- [xclip](https://github.com/astrand/xclip)
- [xdotool](https://github.com/jordansissel/xdotool)

## Installation

### AUR

```bash
paru -S rofi-unicode
```

---

### just

1. Clone repo:

    ```bash
    git clone https://github.com/Rolv-Apneseth/rofi-unicode.git
    ```

2. Use `just` to install (requires `cargo` and `just`)

    ```bash
    cd rofi-unicode && sudo just install
    ```

---

### Manual (not recommended)

```bash
git clone https://github.com/Rolv-Apneseth/rofi-unicode.git
cd rofi-unicode
cargo build --release --lib
sudo cp target/release/librofi_unicode.so /usr/lib/rofi/unicode.so
```

If you are using the latest changes from the rofi repo (e.g. rofi-lbonn-wayland-git, rofi-git), then the build step needs to be preceded by RUSTFLAGS="--cfg rofi_next" for it to work

## Usage

After installing, simply run the following command:

```bash
rofi -modi unicode -show unicode
```

However, I also recommend setting a theme with the `-theme` flag. The demo uses the builtin Monokai theme (`-theme Monokai`), and for my own use I have custom themes [here](https://github.com/Rolv-Apneseth/.dotfiles/tree/main/rofi/.config/rofi), as seen on the demo image for [rofi-nerdy](https://github.com/Rolv-Apneseth/rofi-nerdy). Experiment with what works for you.

## Keybinds

| Keybind           | Default rofi keybind              | Action                              |
| ----------------- | --------------------------------- | ----------------------------------- |
| `kb-accept-entry` | <kbd>Enter</kbd>                  | Copy unicode character              |
| `kb-accept-alt`   | <kbd>Shift</kbd>+<kbd>Enter</kbd> | Attempt to insert unicode character |

- To change a `rofi` keybind, you can, for example, use `-kb-accept-entry Ctrl+c`

## Other Rofi plugins

- [rofimoji](https://github.com/fdw/rofimoji): I wish I had found this one before bothering to work on
  this ... probably better all around, and allows choosing
  specific groups, and even nerd font icons
- [rofi-emoji](https://github.com/Mange/rofi-emoji): much better than this plugin for emojis specifically
- [rofi-calc](https://github.com/svenstaro/rofi-calc): calculator with natural language parsing - pretty cool

And of course, shameless plug of my own ones:

- [rofi-games](https://github.com/Rolv-Apneseth/rofi-games): launch games installed from various sources on your system
- [rofi-nerdy](https://github.com/Rolv-Apneseth/rofi-nerdy): like this plugin but for nerd font icons

## Acknowledgement

- [This SO Q+A](https://stackoverflow.com/questions/65158620/official-repository-of-unicode-character-names) for pointing me towards the [Unicode Character Database](https://unicode.org/ucd/)
- [rofi-mode-rs](https://github.com/SabrinaJewson/rofi-mode.rs) for the Rust bindings to create `Rofi` plugins
- All the other  [dependencies](./Cargo.toml) of this project, and all [their dependencies](./Cargo.lock) too
