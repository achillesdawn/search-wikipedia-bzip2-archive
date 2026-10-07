use serde::Deserialize;

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct Page {
    pub title: String,
    pub ns: i32,
    pub id: u64,
    pub redirect: Option<Redirect>,
    // MediaWiki dumps can have one or multiple revisions per page
    #[serde(default)]
    pub revision: Vec<Revision>,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct Redirect {
    #[serde(rename = "@title")]
    pub title: String,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct Revision {
    pub id: u64,
    pub parentid: Option<u64>,
    pub timestamp: String,
    pub contributor: Option<Contributor>,
    /// `<minor />` is present as an empty tag when true, absent when false
    pub minor: Option<()>,
    pub comment: Option<String>,
    pub model: Option<String>,
    pub format: Option<String>,
    pub text: Option<RevisionText>,
    pub sha1: Option<String>,
}

impl Revision {
    /// Helper to check if the edit was marked as minor
    pub fn is_minor(&self) -> bool {
        self.minor.is_some()
    }
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct Contributor {
    // Registered users have username & id:
    pub username: Option<String>,
    pub id: Option<u64>,
    // Anonymous users have an IP address instead:
    pub ip: Option<String>,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct RevisionText {
    #[serde(rename = "@bytes")]
    pub bytes: Option<u64>,

    #[serde(rename = "@xml:space")]
    pub space: Option<String>,

    /// Inner text of the `<text>` element
    #[serde(rename = "$value", default)]
    pub body: String,
}
