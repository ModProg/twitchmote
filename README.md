# Twitchmotes
This is to create a font containing twitch emotes as emojis in the [Supplementary Private Use Area-A](https://en.wikipedia.org/wiki/Private_Use_Areas#PUA-A).

## Example configuration file
```toml
# config.toml
start_point = 0xF4000
global_emotes = true
channels = ["togglebit", "modprog"]
custom_emotes = "./custom"
output_font = "./twitchmotes.ttf"
output_map = "./map.csv"
```

## Usage
1. Configure all relevant values in `config.toml` (or a name of your choice)
2. You can put pngs named `{emoteName}.png` in the `custom_emotes` directory.
3. Run `twitchmotes config.toml`
4. Copy the font `output_font` in your font path e.g. `~/.local/share/fonts/`
5. Run `fc-cache` to make it available.
6. You can check the mapping from emote name to code point in `output_map`.
7. You can check availability of your font via running `fc-match ":charset={code_point}"` e.g. `fc-match ":charset=f4000"`
8. You should be able to see your emote in terminal now by running `printf '\U{code_point}\n'` e.g. `printf '\Uf4000\n'`

