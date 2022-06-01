//! Module containing embedded assets

use rust_embed::RustEmbed;

/// Embedded asset accessor
#[derive(RustEmbed)]
#[folder = "assets/"]
pub struct Assets;