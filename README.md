# Twitchmotes
This is to create a font containing twitch emotes as emojis in the [Supplementary Private Use Area-A](https://en.wikipedia.org/wiki/Private_Use_Areas#PUA-A).

You can use this with emote_mapper: https://nest.pijul.com/ModProg/emote_mapper

## Prerequisites
This tool requires some python tools, you can install those with:
```sh
pip install -r requirements.txt
```

## Example configuration file
```toml
# config.toml

# Code point of the first emote
start_point = 0xF4000
# Should global emotes be added
global_emotes = true
# Channels whose emotes are used 
channels = ["togglebit", "modprog"]
# Directory containing Images to generate custom emotes from 
# The file name without extension is used as the emote name
custom_emotes = "./custom"
# File to store the font
output_font = "./twitchmotes.ttf"
# File to store the map from emote name to unicode code point
output_map = "./map.csv"
# Either 1 => 28px, 2 => 56px, 3 => 112px
emote_scale = 3
# The number of parallel downloads when fetching the twitch emotes
parallel_downloads = 32
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

