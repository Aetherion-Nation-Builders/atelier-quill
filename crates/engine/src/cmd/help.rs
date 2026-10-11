//! Help → the Nation and project links (the Nation, the Atelier suite, this app's page and its
//! GitHub repository). The commands return the URL; the frontend opens it. The command ids keep
//! their upstream names (`help.discord`, `help.website`) so menus and shortcuts stay untouched.

use serde_json::{Value, json};

use super::*;

/// The app's id in its web address and repository name.
pub const APP_ID: &str = "quill";
/// Where people gather: the Nation (the `help.discord` command).
pub const DISCORD_URL: &str = "https://www.aetherionnnation.com";
/// The suite (the `help.website` command).
pub const WEBSITE_URL: &str = "https://www.aetherionnnation.com/atelier";

/// This app's page on the website.
pub fn app_page_url() -> String {
    format!("{WEBSITE_URL}/{APP_ID}")
}

/// This app's source repository.
pub fn github_url() -> String {
    format!("https://github.com/Aetherion-Nation-Builders/atelier-{APP_ID}")
}

pub fn specs() -> Vec<CommandSpec> {
    vec![
        cmd!(query "help.discord", "Join the Nation", ["Help"], None, "{} → {url} the Nation", always, |_, _| url(DISCORD_URL.into())),
        cmd!(query "help.website", "Atelier Suite", ["Help"], None, "{} → {url}", always, |_, _| url(WEBSITE_URL.into())),
        cmd!(query "help.appPage", "Atelier Quill Website", ["Help"], None, "{} → {url} this app's page", always, |_, _| url(app_page_url())),
        cmd!(query "help.github", "Atelier Quill on GitHub", ["Help"], None, "{} → {url} source code, issues and releases", always, |_, _| url(github_url())),
        cmd!(query "help.links", "Links", [], None, "{} → {discord, website, appPage, github}", always, |_, _| {
            Ok(json!({ "discord": DISCORD_URL, "website": WEBSITE_URL, "appPage": app_page_url(), "github": github_url() }))
        }),
    ]
}

fn url(u: String) -> Result<Value> {
    Ok(json!({ "url": u }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn links() {
        let mut s = Session::new();
        assert_eq!(s.execute("help.discord", &json!({})).unwrap()["url"], "https://www.aetherionnnation.com");
        assert_eq!(s.execute("help.appPage", &json!({})).unwrap()["url"], "https://www.aetherionnnation.com/atelier/quill");
        assert_eq!(s.execute("help.github", &json!({})).unwrap()["url"], "https://github.com/Aetherion-Nation-Builders/atelier-quill");
        assert_eq!(s.execute("help.links", &json!({})).unwrap()["website"], "https://www.aetherionnnation.com/atelier");
    }
}
