use std::{
    collections::{HashMap, HashSet},
    fs::{self, File},
    io::Write,
    path::PathBuf,
};

use atom_syndication::{Entry, FeedBuilder, Link, Person, Text};
use chrono::DateTime;
use glob::glob;
use sailfish::TemplateOnce;

#[cfg(test)]
mod tests;

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
    comments_issue: Option<String>,
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

#[derive(Debug)]
struct Frontmatter<'a, T> {
    headers: T,
    body: &'a str,
}

impl<'a, T> Frontmatter<'a, T>
where
    T: TryFrom<HashMap<String, String>, Error = String>,
{
    fn parse(contents: &'a str) -> Result<Self, String> {
        if contents.trim().is_empty() {
            return Err("Empty content".to_string());
        }

        let lines_vec: Vec<&str> = contents.lines().collect();

        if lines_vec.len() < 3 || !lines_vec[0].trim().starts_with("---") {
            return Err("Missing frontmatter delimiter".to_string());
        }

        let mut in_frontmatter = false;
        let mut frontmatter_lines = Vec::new();
        let mut frontmatter_len = 0;

        for line in lines_vec.iter() {
            frontmatter_len += line.len() + "\n".len();
            let trimmed = line.trim();
            if trimmed == "---" {
                if in_frontmatter {
                    break;
                } else {
                    in_frontmatter = true;
                    continue;
                }
            }
            if in_frontmatter {
                frontmatter_lines.push(*line);
            }
        }

        if !in_frontmatter || frontmatter_lines.is_empty() {
            return Err("Empty frontmatter".to_string());
        }

        let mut map = HashMap::new();
        for line in frontmatter_lines {
            let trimmed = line.trim();
            if let Some((key, value)) = trimmed.split_once(':') {
                let key = key.trim().to_string();
                let value = value.trim().trim_matches('"').to_string();
                if !value.is_empty() {
                    map.insert(key, value);
                }
            }
        }

        let headers = T::try_from(map)?;

        Ok(Frontmatter {
            headers,
            body: &contents[frontmatter_len..],
        })
    }
}

#[derive(Debug, PartialEq, Eq)]
struct PageHeaders {
    title: String,
}

impl TryFrom<HashMap<String, String>> for PageHeaders {
    type Error = String;

    fn try_from(map: HashMap<String, String>) -> Result<Self, Self::Error> {
        match map.get("title") {
            Some(title) => Ok(Self {
                title: title.clone(),
            }),
            None => Err("Missing title in frontmatter".to_string()),
        }
    }
}

impl PageHeaders {
    fn parse(contents: &str) -> Result<Frontmatter<'_, Self>, String> {
        Frontmatter::parse(contents)
    }
}

#[derive(Debug, PartialEq, Eq)]
struct PostHeaders {
    title: String,
    author: String,
    published: String,
    tags: String,
    language: Option<String>,
    comments_issue: Option<String>,
}

impl TryFrom<HashMap<String, String>> for PostHeaders {
    type Error = String;

    fn try_from(map: HashMap<String, String>) -> Result<PostHeaders, String> {
        let title = match map.get("title") {
            Some(title) => title.clone(),
            None => return Err("Missing title in frontmatter".to_string()),
        };
        let author = match map.get("author") {
            Some(author) => author.clone(),
            None => return Err("Missing author in frontmatter".to_string()),
        };
        let published = match map.get("published") {
            Some(published) => published.clone(),
            None => return Err("Missing published in frontmatter".to_string()),
        };
        let tags = match map.get("tags") {
            Some(tags) => tags.clone(),
            None => return Err("Missing tags in frontmatter".to_string()),
        };
        let language = map.get("language").cloned();
        let comments_issue = map.get("commentsIssue").cloned();

        Ok(Self {
            title,
            author,
            published,
            tags,
            language,
            comments_issue,
        })
    }
}

impl PostHeaders {
    fn parse(contents: &str) -> Result<Frontmatter<'_, Self>, String> {
        Frontmatter::parse(contents)
    }
}

struct PostItem {
    url: String,
    title: String,
    date: String,
    tags: Vec<String>,
}

fn main() -> Result<(), String> {
    let patterns_copy = ["CNAME", "images/*", "css/*"];
    let globs_copy = patterns_copy
        .iter()
        .flat_map(|p| glob(p).expect("Failed to read file to copy"));

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
                let fronma = PageHeaders::parse(&contents)?;
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
                let fronma = PostHeaders::parse(&contents)?;
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
                    comments_issue: fronma.headers.comments_issue
                };
                let body_post = ctx_post.render_once().unwrap();
                let ctx_default = DefaultTemplate {
                    title: fronma.headers.title.to_owned(),
                    body: body_post.to_owned(),
                };
                let stem = path.file_stem().unwrap().to_str().unwrap();
                sites.push(PostItem {
                    url: format!("/posts/{stem}.html"),
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
        let mut file = File::create("docs/index.html").unwrap();
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
        let mut file = File::create("docs/posts.html").unwrap();
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

    Ok(())
}
