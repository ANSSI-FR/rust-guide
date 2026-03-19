//! Largely inspired by (grammalecte_client)[https://gitea.communiquons.org/pierre/GrammalecteClient]
//!
//! from Pierre Hubert <pierre.git@communiquons.org>

use std::io::Write;

/// Check spelling result
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CheckResult {
    pub lang: String,
    #[serde(rename = "data")]
    pub paragraphs: Vec<Paragraph>,
}

/// Check spell result of a given paragraph
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Paragraph {
    #[serde(rename = "iParagraph")]
    pub num: usize,
    #[serde(rename = "lGrammarErrors")]
    pub grammars: Vec<GrammarError>,
    #[serde(rename = "lSpellingErrors")]
    pub spelling: Vec<SpellingError>,
}

/// Single grammar error
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GrammarError {
    #[serde(rename = "nStart")]
    pub offset_start: usize,
    #[serde(rename = "nEnd")]
    pub offset_end: usize,
    #[serde(rename = "sLineId")]
    pub rule_line_id: String,
    #[serde(rename = "sRuleId")]
    pub rule_id: String,
    #[serde(rename = "sType")]
    pub rule_type: String,
    #[serde(rename = "aColor")]
    pub rule_underline_color: Option<Vec<u8>>,
    #[serde(rename = "sMessage")]
    pub message: String,
    #[serde(rename = "aSuggestions")]
    pub suggestions: Vec<String>,
    #[serde(rename = "URL")]
    pub url: String,
}

/// Spelling error information
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SpellingError {
    pub i: usize,
    #[serde(rename = "nStart")]
    pub offset_start: usize,
    #[serde(rename = "nEnd")]
    pub offset_end: usize,
    #[serde(rename = "sValue")]
    pub bad_word: String,
    #[serde(rename = "sType")]
    pub error_type: String,
}

pub(crate) fn run_grammalecte(text: &str) -> CheckResult {
    let mut file = tempfile::NamedTempFile::new().unwrap();
    file.write_all(text.as_bytes()).unwrap();
    let file_path = file.path();
    let mut cmd = std::process::Command::new("grammalecte-cli.py");

    cmd.args(["-f", file_path.as_os_str().to_str().unwrap()])
        .arg("-owe")
        .args(["-off", "apos", "esp", "nbsp", "typo", "maj"])
        .arg("-j");

    let res = cmd.output().expect("cannot run grammalecte");
    serde_json::from_slice(&res.stdout).expect("cannot parse grammalecte result")
}
