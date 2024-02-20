use std::{collections::HashSet, fs::{self, File}, io::Write, path::PathBuf};

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
                let dest = PathBuf::from("docs");
                fs::copy(&path, dest.join(&path))
                    .expect("Couldn't copy file");
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
                    author: Some(fronma.headers.author),
                    date: fronma.headers.published.to_owned(),
                    tags: tags
                        .iter()
                        .map(|s| TagTemplate { tag: s.clone() }.render_once().unwrap())
                        .collect(),
                    body: body_html,
                    commentsIssue: fronma.headers.commentsIssue
                };
                let ctx_default = DefaultTemplate {
                    title: fronma.headers.title.to_owned(),
                    body: ctx_post.render_once().unwrap()
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
            },
            Err(e) => println!("{:?}", e),
        }
    }

    sites.sort_by(|a, b| b.date.cmp(&a.date));

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
}
