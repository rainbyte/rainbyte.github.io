use std::{fs::{self, File}, io::Write, path::PathBuf};

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

    for post in glob("posts/**/*.md").expect("Failed to read posts") {
        match post {
            Ok(path) => {
                let contents = fs::read_to_string(&path).unwrap();
                let fronma = parse::<PostHeaders>(&contents).unwrap();
                let parser = pulldown_cmark::Parser::new(fronma.body);
                let mut body_html = String::new();
                pulldown_cmark::html::push_html(&mut body_html, parser);
                let ctx_post = PostTemplate {
                    author: Some(fronma.headers.author),
                    date: fronma.headers.published,
                    tags: fronma.headers.tags
                        .split(", ")
                        .map(|s| s.to_string())
                        .collect(),
                    body: body_html,
                    commentsIssue: fronma.headers.commentsIssue
                };
                let ctx_default = DefaultTemplate {
                    title: fronma.headers.title,
                    body: ctx_post.render_once().unwrap()
                };
                let stem = path.file_stem().unwrap().to_str().unwrap();
                let mut file = File::create(format!("docs/posts/{stem}.html")).unwrap();
                let _ = file.write_all(ctx_default.render_once().unwrap().as_bytes());
            },
            Err(e) => println!("{:?}", e),
        }
    }
}
