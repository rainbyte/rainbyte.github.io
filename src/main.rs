use std::{collections::HashSet, fs::{self, File}, io::Write, path::PathBuf};

use atom_syndication::{Entry, FeedBuilder, Link, Person, Text};
use chrono::DateTime;
use fronma::parser::parse;
use glob::glob;
use sailfish::TemplateOnce;
use serde::Deserialize;

#[derive(TemplateOnce)]
#[template(path = "../templates/default.stpl")]
struct DefaultTemplate {
    title: String,
    body: String,
}

#[derive(TemplateOnce)]
#[template(path = "../templates/post.stpl")]
struct PostTemplate {
    date: String,
    author: Option<String>,
    tags: Vec<String>,
    commentsIssue: Option<String>,
    body: String,
}

#[derive(TemplateOnce)]
#[template(path = "../templates/tag.stpl")]
struct TagTemplate {
    tag: String,
}

#[derive(TemplateOnce)]
#[template(path = "../templates/post_item.stpl")]
struct PostItemTemplate {
    url: String,
    title: String,
    date: String,
    tags: Vec<String>,
}

#[derive(TemplateOnce)]
#[template(path = "../templates/post_list.stpl")]
struct PostListTemplate {
    posts: Vec<String>,
}

#[derive(TemplateOnce)]
#[template(path = "../templates/index.stpl")]
struct IndexTemplate {
    posts: Vec<String>,
}

#[derive(Deserialize)]
struct PageHeaders {
    title: String,
}

#[derive(Deserialize)]
struct PostHeaders {
    title: String,
    author: String,
    published: String,
    tags: String,
    language: Option<String>,
    commentsIssue: Option<String>,
}

struct PostItem {
    url: String,
    title: String,
    date: String,
    tags: Vec<String>,
}

fn main() {
    let patterns_copy = ["CNAME", "images/*", "css/*"];
    let globs_copy = patterns_copy
        .iter()
        .map(|p| glob(p).expect("Failed to read file to copy"))
        .flatten();

    for x in globs_copy {
        match x {
            Ok(path) => {
                let dest = PathBuf::from("docs").join(&path);
                if let Some(parent) = dest.parent() {
                    fs::create_dir_all(parent).expect("Couldn't create dir");
                }
                fs::copy(&path, dest).expect("Couldn't copy file");
            },
            Err(e) => println!("{:?}", e),

        }
    }

    for page in glob("*.md").expect("Failed to read root pages") {
        match page {
            Ok(path) => {
                let contents = fs::read_to_string(&path).unwrap();
                let fronma = parse::<PageHeaders>(&contents).unwrap();
                let parser = pulldown_cmark::Parser::new(fronma.body);
                let mut body_html = String::new();
                pulldown_cmark::html::push_html(&mut body_html, parser);
                let ctx_default = DefaultTemplate {
                    title: fronma.headers.title,
                    body: body_html
                };
                let stem = path.file_stem().unwrap().to_str().unwrap();
                let mut file = File::create(format!("docs/{stem}.html")).unwrap();
                let _ = file.write_all(ctx_default.render_once().unwrap().as_bytes());
            },
            Err(e) => println!("{:?}", e),
        }
    }

    let mut sites: Vec<PostItem> = Vec::new();
    let mut tags_set: HashSet<String> = HashSet::new();

    let mut feed_entries = Vec::new();

    let posts_dir = PathBuf::from("docs/posts");
    fs::create_dir_all(posts_dir).expect("Couldn't create posts dir");
    for post in glob("posts/**/*.md").expect("Failed to read posts") {
        match post {
            Ok(path) => {
                let contents = fs::read_to_string(&path).unwrap();
                let fronma = parse::<PostHeaders>(&contents).unwrap();
                let parser = pulldown_cmark::Parser::new(fronma.body);
                let mut body_html = String::new();
                pulldown_cmark::html::push_html(&mut body_html, parser);
                let tags: Vec<String> = fronma.headers.tags
                    .split(", ")
                    .map(|s| s.to_string())
                    .collect();
                let ctx_post = PostTemplate {
                    author: Some(fronma.headers.author.to_owned()),
                    date: fronma.headers.published.to_owned(),
                    tags: tags
                        .iter()
                        .map(|s| TagTemplate { tag: s.clone() }.render_once().unwrap())
                        .collect(),
                    body: body_html,
                    commentsIssue: fronma.headers.commentsIssue
                };
                let body_post = ctx_post.render_once().unwrap();
                let ctx_default = DefaultTemplate {
                    title: fronma.headers.title.to_owned(),
                    body: body_post.to_owned(),
                };
                let stem = path.file_stem().unwrap().to_str().unwrap();
                sites.push(PostItem {
                    url: format!("posts/{stem}.html"),
                    title: fronma.headers.title.to_owned(),
                    date: fronma.headers.published.to_owned(),
                    tags: tags.to_owned(),
                });
                for tag in &tags {
                    tags_set.insert(tag.to_owned());
                }
                let mut file = File::create(format!("docs/posts/{stem}.html")).unwrap();
                let _ = file.write_all(ctx_default.render_once().unwrap().as_bytes());
                feed_entries.push(Entry {
                    title: Text::plain(fronma.headers.title),
                    published: DateTime::parse_from_str(&format!("{} -03", fronma.headers.published), "%Y-%m-%d %H:%M:%S %#z").ok(),
                    updated: DateTime::parse_from_str(&format!("{} -03", fronma.headers.published), "%Y-%m-%d %H:%M:%S %#z").unwrap(),
                    authors: vec![Person {
                        name: fronma.headers.author,
                        email: None,
                        uri: None
                    }],
                    summary: Some(Text::html(body_post)),
                    links: vec![Link {
                        href: format!("https://rainbyte.net.ar/posts/{stem}.html"),
                        ..Default::default()
                    }],
                    id: format!("https://rainbyte.net.ar/posts/{stem}.html"),
                    ..Default::default()
                });
            },
            Err(e) => println!("{:?}", e),
        }
    }

    sites.sort_by(|a, b| b.date.cmp(&a.date));

    // Index page
    {
        let ctx_index = IndexTemplate {
            posts: sites
                .iter()
                .take(5)
                .map(|t| PostItemTemplate {
                    url: t.url.to_owned(),
                    title: t.title.to_owned(),
                    date: t.date.to_owned(),
                    tags: t.tags
                        .iter()
                        .map(|s| TagTemplate { tag: s.clone() }.render_once().unwrap())
                        .collect(),
                })
                .map(|t| t.render_once().unwrap())
                .collect()
        };
        let ctx_default = DefaultTemplate {
            title: "Home".to_string(),
            body: ctx_index.render_once().unwrap()
        };
        let mut file = File::create(format!("docs/index.html")).unwrap();
        let _ = file.write_all(ctx_default.render_once().unwrap().as_bytes());
    }

    // Archive page
    {
        let ctx_post_list = PostListTemplate {
            posts: sites
                .iter()
                .map(|t| PostItemTemplate {
                    url: t.url.to_owned(),
                    title: t.title.to_owned(),
                    date: t.date.to_owned(),
                    tags: t.tags
                        .iter()
                        .map(|s| TagTemplate { tag: s.clone() }.render_once().unwrap())
                        .collect(),
                })
                .map(|t| t.render_once().unwrap())
                .collect()
        };
        let ctx_default = DefaultTemplate {
            title: "All posts".to_string(),
            body: ctx_post_list.render_once().unwrap()
        };
        let mut file = File::create(format!("docs/posts.html")).unwrap();
        let _ = file.write_all(ctx_default.render_once().unwrap().as_bytes());
    }

    let tags_dir = PathBuf::from("docs/tags");
    fs::create_dir_all(tags_dir).expect("Couldn't create tags dir");
    for tag in tags_set {
        let ctx_post_list = PostListTemplate {
            posts: sites
                .iter()
                .filter(|t| t.tags.contains(&tag))
                .map(|t| PostItemTemplate {
                    url: t.url.to_owned(),
                    title: t.title.to_owned(),
                    date: t.date.to_owned(),
                    tags: t.tags
                        .iter()
                        .map(|s| TagTemplate { tag: s.clone() }.render_once().unwrap())
                        .collect(),
                })
                .map(|t| t.render_once().unwrap())
                .collect()
        };
        let ctx_default = DefaultTemplate {
            title: format!("Tag: {tag}"),
            body: ctx_post_list.render_once().unwrap()
        };
        let mut file = File::create(format!("docs/tags/{tag}.html")).unwrap();
        let _ = file.write_all(ctx_default.render_once().unwrap().as_bytes());
    }

    feed_entries.sort_by(|a, b| b.updated.cmp(&a.updated));
    let feed_updated = feed_entries.first().unwrap().updated;

    let feed_file = File::create("docs/atom.xml").unwrap();
    let feed = FeedBuilder::default()
        .title("(λblog.rainbyte)")
        .subtitle(Text::plain("A site about things I enjoy and would like to share"))
        .authors([Person {
            name: "rainbyte".to_string(),
            email: None,
            uri: None
        }])
        .base("https://rainbyte.net.ar".to_string())
        .entries(feed_entries)
        .id("https://rainbyte.net.ar/atom.xml")
        .updated(feed_updated)
        .links(vec![
            Link {
                href: "https://rainbyte.net.ar".to_string(),
                ..Default::default()
            },
            Link {
                href: "https://rainbyte.net.ar/atom.xml".to_string(),
                rel: "self".to_string(),
                ..Default::default()
            },
        ])
        .build();
    feed.write_to(feed_file).unwrap();
}
