use std::collections::HashMap;
use std::convert::TryInto;
use std::ffi::OsStr;
use std::fs::{create_dir_all, read_dir, remove_dir_all, File};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::{env, fs};

use anyhow::{Context, Result};
use emoji_builder::builders::blobmoji::{Blobmoji, PNG_DIR, TMPL_TTX_TMPL, TMPL_TTX_TMPL_CONTENT};
use emoji_builder::changes::FileHashes;
use emoji_builder::emoji::Emoji;
use emoji_builder::sha2::digest::generic_array::GenericArray;
use emoji_builder::sha2::{Digest, Sha256};
use emoji_builder::usvg::fontdb;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Config {
    start_point: u32,
    global_emotes: bool,
    channels: Vec<String>,
    custom_emotes: Option<PathBuf>,
    output_font: PathBuf,
    output_map: PathBuf,
}

#[tokio::main]
async fn main() -> Result<()> {
    let token = env::var("TWITCHMOTES_TOKEN").ok();
    let file: PathBuf = env::args()
        .nth(1)
        .context("No config file was passed")?
        .try_into()
        .with_context(|| "Invalid filepath passed for config")?;
    let config: Config = toml::from_str(&{
        let mut string = String::new();
        File::open(&file)
            .with_context(|| format!("Unable to open config file: {}", file.display()))?
            .read_to_string(&mut string)
            .expect("Config file to contain only valid UTF-8");
        string
    })
    .with_context(|| format!("Failed to parse config file: {}", file.display()))?;

    let bm = Blobmoji {
        build_path: PathBuf::from("./build"),
        hashes: FileHashes(HashMap::new()),
        aliases: None,
        render_only: false,
        default_font: String::from(""),
        fontdb: fontdb::Database::new(),
        waveflag: false,
        reduce_colors: None,
        build_win: false,
    };

    // Create the PNG directory clear it if it exists
    let png_dir = bm.build_path.join(PNG_DIR);
    if png_dir.exists() {
        remove_dir_all(&png_dir).with_context(|| {
            format!("Unable to remove old png build dir: {}", png_dir.display())
        })?;
    }
    create_dir_all(&png_dir)
        .with_context(|| format!("Unable to create png build dir: {}", png_dir.display()))?;

    let ttx_tmpl_path = bm.build_path.join(TMPL_TTX_TMPL);

    let mut file = File::create(&ttx_tmpl_path).with_context(|| {
        format!(
            "Unable to create template file in build directory: {}",
            ttx_tmpl_path.display(),
        )
    })?;
    file.write_all(TMPL_TTX_TMPL_CONTENT)
        .expect("Be able to write content of template file");

    // Create the emoji_u*.pngs

    // Custom Emotes
    let mut code_point = config.start_point;
    let mut emotes = vec![];

    if let Some(custom_dir) = config.custom_emotes {
        let mut custom_emotes = custom_emotes(&custom_dir, &png_dir, &mut code_point)?;
        emotes.append(&mut custom_emotes);
    }
    // TODO twitch download

    // Create mapping
    let mut csv = String::new();

    for (name, code_point) in &emotes {
        csv.push_str(&format!("{},{:x}\n", name, code_point))
    }

    fs::write(config.output_map, csv)?;

    // Prepare emoji_builder compatible inputs

    let emojis = emotes
        .into_iter()
        .map(|(name, code_point)| {
            (
                Emoji {
                    sequence: vec![code_point],
                    name: Some(name),
                    kinds: None,
                    svg_path: None,
                },
                code_point,
            )
        })
        .collect::<Vec<_>>();
    let emojis = emojis
        .iter()
        .map(|(emoji, code_point)| {
            (
                emoji,
                Ok((
                    png_dir.join(format!("emoji_u{:x}.png", code_point)),
                    Ok(vec![0; 32]
                        .into_iter()
                        .collect::<GenericArray<u8, <Sha256 as Digest>::OutputSize>>()),
                )),
            )
        })
        .collect();
    bm.build_font(&emojis, &config.output_font, true);
    Ok(())
}

fn custom_emotes(
    from_dir: &Path,
    to_dir: &Path,
    codepoint: &mut u32,
) -> Result<Vec<(String, u32)>> {
    let mut emotes = vec![];

    for file in read_dir(from_dir)? {
        let file = file?.path();
        if let (Some(name), Some(extension)) = (
            file.clone()
                .file_stem()
                .map(OsStr::to_string_lossy)
                .map(|s| s.to_string()),
            file.extension(),
        ) {
            match extension.to_string_lossy().to_lowercase().as_str() {
                "png" => {
                    fs::copy(file, to_dir.join(format!("emoji_u{:x}.png", codepoint)))?;
                }
                _ => continue, // incompatible file, ignored
            }
            emotes.push((name, *codepoint));
            *codepoint += 1;
        }
    }

    Ok(emotes)
}
