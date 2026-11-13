//! Small pages for the web preview's frame, so that the gallery and its checks never
//! load another site. Plain HTML: no scripts, and styles of its own.

use topcoat::{
    Result,
    context::Cx,
    router::{content::Html, route},
};

use super::kit::query_param;

/// `/preview?page=home`, `about` or `pricing`.
#[route(GET)]
async fn preview(cx: &Cx) -> Result<Html<String>> {
    let page = query_param(cx, "page").unwrap_or_default();
    let (title, body) = match page.as_str() {
        "about" => (
            "About",
            "A small team that builds tools for writing software.",
        ),
        "pricing" => ("Pricing", "Free for one project; paid plans add more."),
        _ => ("Home", "A landing page made by the assistant."),
    };
    Ok(Html(format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="utf-8">
<title>{title}</title>
<style>
  body {{ margin: 0; font: 16px/1.5 system-ui, sans-serif; color: #1f2937; background: #ffffff; }}
  nav, main {{ padding: 12px 20px; }}
  nav a {{ margin-inline-end: 12px; color: #1d4ed8; }}
  h1 {{ margin: 0 0 8px; font-size: 24px; }}
</style>
</head>
<body>
<nav aria-label="Site"><a href="/preview?page=home">Home</a><a href="/preview?page=about">About</a><a href="/preview?page=pricing">Pricing</a></nav>
<main aria-label="{title} page"><h1>{title}</h1><p>{body}</p></main>
</body>
</html>
"#
    )))
}
