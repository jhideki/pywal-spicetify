use serde::Deserialize;
#[derive(Debug, Deserialize)]
pub struct Colors {
    pub color0: String,
    pub color1: String,
    pub color2: String,
    pub color3: String,
    pub color4: String,
    pub color5: String,
    pub color6: String,
    pub color7: String,
    pub color8: String,
    pub color9: String,
    pub color10: String,
    pub color11: String,
    pub color12: String,
    pub color13: String,
    pub color14: String,
    pub color15: String,
}

#[derive(Debug, Deserialize)]
pub struct Special {
    pub background: String,
    pub foreground: String,
    pub cursor: String,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub wallpaper: String,
    pub alpha: String,
    pub special: Special,
    pub colors: Colors,
}
