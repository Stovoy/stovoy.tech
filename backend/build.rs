use std::env;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

use chrono::prelude::*;
use pulldown_cmark::{html, Parser};
use regex::Regex;
use serde::Serialize;

#[derive(Serialize)]
struct BlogMeta {
    title: String,
    date: String,
    slug: String,
}

fn main() {
    if std::env::var("SKIP_BLOG_BUILD").is_ok() {
        return;
    }
    let workspace_root = locate_workspace_root();
    let content_dir = workspace_root.join("content");
    if !content_dir.exists() {
        return;
    }

    let dist_blog_dir = workspace_root.join("dist/blog");
    if fs::create_dir_all(&dist_blog_dir).is_err() {
        return;
    }

    let mut metas = Vec::new();

    for entry in fs::read_dir(&content_dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) != Some("md") {
            continue;
        }

        let slug = path.file_stem().unwrap().to_string_lossy().to_string();

        let mut markdown = String::new();
        File::open(&path).unwrap().read_to_string(&mut markdown).unwrap();

        let (title, date_str) = extract_title_date(&markdown, &path);

        let parser = Parser::new(&markdown);
        let mut html_output = String::new();
        html::push_html(&mut html_output, parser);

        html_output = transform_mermaid(&html_output);
        html_output = transform_img_paths(&html_output);

        let slug_dir = dist_blog_dir.join(&slug);
        fs::create_dir_all(&slug_dir).unwrap();
        let mut file = File::create(slug_dir.join("index.html")).unwrap();
        file.write_all(html_output.as_bytes()).unwrap();

        metas.push(BlogMeta {
            title,
            date: date_str,
            slug,
        });
    }

    metas.sort_by(|a, b| b.date.cmp(&a.date));

    let meta_json = serde_json::to_string(&metas).unwrap();

    let mut meta_file = File::create(dist_blog_dir.join("blogs.json")).unwrap();
    meta_file.write_all(meta_json.as_bytes()).unwrap();

    let mut root_meta_file = File::create(workspace_root.join("dist/blogs.json")).unwrap();
    root_meta_file.write_all(meta_json.as_bytes()).unwrap();

    generate_rss(&workspace_root, &metas);

    copy_static_images(&workspace_root);
}

fn transform_img_paths(input: &str) -> String {
    let re = Regex::new(r#"<img[^>]*src=\"([^\"]+)\"[^>]*>"#).unwrap();
    re.replace_all(input, |caps: &regex::Captures| {
        let src = &caps[1];
        if src.starts_with("http") || src.starts_with('/') {
            caps[0].to_string()
        } else {
            caps[0].replace(src, &format!("/img/{}", src))
        }
    })
    .to_string()
}

fn copy_static_images(root: &Path) {
    let source = root.join("frontend/static/img");
    if !source.exists() {
        return;
    }
    let target = root.join("dist/img");
    fs::create_dir_all(&target).unwrap();
    copy_dir_recursively(&source, &target).unwrap();
}

fn copy_dir_recursively(src: &Path, dst: &Path) -> std::io::Result<()> {
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let path = entry.path();
        let dest_path = dst.join(entry.file_name());
        if path.is_dir() {
            fs::create_dir_all(&dest_path)?;
            copy_dir_recursively(&path, &dest_path)?;
        } else {
            fs::copy(&path, &dest_path)?;
        }
    }
    Ok(())
}


fn transform_mermaid(input: &str) -> String {
    let re = Regex::new(r#"<pre><code class=\"language-(?:dolphin|mermaid)\">([\s\S]*?)</code></pre>"#).unwrap();
    re.replace_all(input, |caps: &regex::Captures| {
        let mut code = caps[1].to_string();
        code = code
            .replace("&amp;", "&")
            .replace("&lt;", "<")
            .replace("&gt;", ">")
            .replace("&quot;", "\"")
            .replace("&#39;", "'");
        format!("<pre class=\"mermaid\">{}</pre>", code)
    })
    .to_string()
}

fn locate_workspace_root() -> PathBuf {
    let mut dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    while dir.parent().is_some() {
        if dir.join("Cargo.toml").exists() && dir.join("frontend").exists() {
            return dir;
        }
        dir = dir.parent().unwrap().to_path_buf();
    }
    dir
}

fn extract_title_date(markdown: &str, path: &Path) -> (String, String) {
    let mut title = String::new();
    let mut date = None;
    for line in markdown.lines() {
        if title.is_empty() && line.starts_with('#') {
            title = line.trim_start_matches('#').trim().to_string();
        }
        if date.is_none() && line.to_ascii_lowercase().starts_with("date:") {
            let d = line.splitn(2, ':').nth(1).unwrap().trim().to_string();
            date = Some(d);
        }
        if !title.is_empty() && date.is_some() {
            break;
        }
    }
    let date_str = date.unwrap_or_else(|| {
        let metadata = fs::metadata(path).unwrap();
        let modified = metadata.modified().unwrap_or(std::time::SystemTime::now());
        let datetime: DateTime<Utc> = DateTime::<Utc>::from(modified);
        datetime.format("%Y-%m-%d").to_string()
    });
    (title, date_str)
}

fn generate_rss(root: &Path, metas: &[BlogMeta]) {
    let channel_title = "stovoy.dev Blog";
    let channel_link = "https://stovoy.dev";
    let channel_description = "Articles from stovoy.dev";

    let items_xml: String = metas
        .iter()
        .map(|meta| {
            format!(
                "<item><title>{}</title><link>{}/blog/{}</link><pubDate>{}</pubDate></item>",
                escape_xml(&meta.title), channel_link, meta.slug, meta.date
            )
        })
        .collect();

    let rss_xml = format!(
        "<rss version=\"2.0\"><channel><title>{}</title><link>{}</link><description>{}</description>{}</channel></rss>",
        channel_title, channel_link, channel_description, items_xml
    );

    let mut file = File::create(root.join("dist/rss.xml")).unwrap();
    file.write_all(rss_xml.as_bytes()).unwrap();
}

fn escape_xml(input: &str) -> String {
    input
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace("\"", "&quot;")
        .replace("'", "&apos;")
}
