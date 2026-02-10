---
title: Migration from fronma library to inhouse frontmatter parsing
author: rainbyte
published: 2026-02-10 03:29:00
tags: rust, blog, refactoring
language: en
---

I've been using the `fronma` library for my static blog generator for
more than four years since I migrated my blog from Haskell to Rust
implementation.

After a period of inactivity, now I reactivated my blog and started
looking at the code again. That's when I noticed how much I could
improve it!

Over time I realized I didn't need most of what `fronma` provided, even
if it was fine at the beginning I noticed dependencies are huge for my
usecase.

Just take a look, my frontmatter format is very simple:

- Page fields:
  - `title` (just this one)
- Blog post fields:
  - `title`
  - `author`
  - `published`
  - `tags`
  - `language` (optional)
  - `commentsIssue` (optional)

As you can see I'm not using complex nesting, custom validators,
multiple date formats, or any advanced conversions from YAML format.

Having all those unused parts means depending on all `serde` machinery,
including `serde_yaml` which was deprecated 3 years ago, and other big
dependencies.

Another reason to change was learning itself!

Given that I built this blog generator to understand how things work in
Rust, sticking with `fronma` meant relying on a library for something I
could implement myself.

<!-- more -->

## The migration

The first thing I did was implementing inline frontmatter parsing in a
compatible way before removing `fronma`, so that almost all the existing
code was kept untouched.

Here is what was changed.

### Old dependencies

My target was removing all these dependencies from `Cargo.toml`:

```toml
fronma = "0.4"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0.149"
serde_yaml = "0.9"
```

And in the end I was able to do it.

### New approach

Just using the standard library and `TryFrom` trait instead of an
external frontmatter parsing library.

The idea was to apply traditional string processing subroutines and
common data structures.

### Code size

- Old implementation size:
  - More than 120 Kb in obvious dependencies D:
    - 4 Kb just from the `fronma` library itself
    - 82 Kb from `serde` serialization framework
    - 40 Kb from `serde_yaml` lib (deprecated)
  - And many more, but I stopped counting there...
- New implementation size:
  - 187 lines in my `src/main.rs`
  - This one is tiny! :D

## Implementation details

I wrote a generic `Frontmatter::parse` function that works with any
struct type comaptible with `TryFrom<HashMap<String, String>>` trait.

The parsing flow:

1. Initialize empty `HashMap<String, String>`
2. Read content and extract frontmatter lines between `---` delimiters
3. Parse each line as `key: value` and append into the hashmap
4. Use `TryFrom` to convert the hashmap to a typed struct

Here's the function skeleton:

```rust
impl<'a, T> Frontmatter<'a, T>
    where T: TryFrom<HashMap<String, String>, Error = String>
{
    fn parse(contents: &'a str) -> Result<Self, String> {
        // Parse frontmatter into HashMap
        // Validate required fields
        // Return typed Frontmatter
    }
}
```

Each header type should be compatible `TryFrom` trait, like this one:

```rust
impl TryFrom<HashMap<String, String>> for PageHeaders {
    type Error = String;

    fn try_from(
        map: HashMap<String, String>
    ) -> Result<Self, Self::Error> {
        match map.get("title") {
            Some(title) => Ok(Self {
                title: title.clone(),
            }),
            None => Err("Missing title in frontmatter".to_string()),
        }
    }
}
```

For blog posts validation is stricter, but simple to understand:

```rust
impl TryFrom<HashMap<String, String>> for PostHeaders {
    type Error = String;

    fn try_from(
        map: HashMap<String, String>
    ) -> Result<Self, Self::Error> {
        let title = match map.get("title") {
            Some(title) => title.clone(),
            None => return Err(
                "Missing title in frontmatter".to_string()
            ),
        };
        let author = match map.get("author") {
            Some(author) => author.clone(),
            None => return Err(
                "Missing author in frontmatter".to_string()
            ),
        };
        let published = match map.get("published") {
            Some(published) => published.clone(),
            None => return Err(
                "Missing published in frontmatter".to_string()
            ),
        };
        let tags = match map.get("tags") {
            Some(tags) => tags.clone(),
            None => return Err(
                "Missing tags in frontmatter".to_string()
            ),
        };
        let language = map.get("language").cloned();
        let comments_issue =
            map.get("commentsIssue").cloned();

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
```

Usage in `main` stayed the same, except for these two lines which called
my own parsing wrappers in order to keep code readable:

```rust
...
let fronma = PageHeaders::parse(&contents)?;
...
let fronma = PostHeaders::parse(&contents)?;
...
```

As you can see I'm passing contents as reference, so the result will
internally borrow the post body from there as an slice, to avoid the
costly operation of cloning the data into a separated String.

## Immediate benefits

### 1. Reduced dependencies

My project now depends only on what I actually use, avoiding `fronma`
and `serde_yaml`.

### 2. Better understanding

Now I know exactly how frontmatter parsing works in my codebase, as it
is just a few lines of code.

In case of debugging, I won't need to browse for library source code.

### 3. Customization

I can easily extend the parser:

- Support different delimiter formats
- Add per-field validators
- Implement field transformations

### 4. Test coverage

To verify parsing worked correctly for my usecase I created a set of
tests.

There were some issues at the beginning with whitespace, but I solved
them by using the explicit `\n\` syntax for avoiding spaces on multiline
strings.

Each test ended up very similar to this one:

```rust
#[test]
fn post_header_good() {
    let contents = "---\n\
        title: How-to decrease gnome title-bar height\n\
        author: rainbyte\n\
        published: 2015-07-02 03:15:07\n\
        tags: gnome, snippets, css\n\
        ---\n\
        abc\n\
        def";
    let result = PostHeaders::parse(contents);
    let fronma = result.unwrap();
    assert_eq!(
        PostHeaders {
            title: "How-to decrease gnome title-bar height"
                .to_string(),
            author: "rainbyte".to_string(),
            published: "2015-07-02 03:15:07".to_string(),
            tags: "gnome, snippets, css".to_string(),
            language: None,
            comments_issue: None,
        },
        fronma.headers
    );
    assert_eq!(
        "abc\n\
        def",
        fronma.body
    )
}
```

So as result all my old posts were rendered to exactly the same html as
before, bit by bit!

## Future ideas

Currently most of the fields are saved as strings, but now that I have
more control over the types I'm planning to type check them by using
stricter timedate and collection types.

Also given that I reactivated my blog and will be focusing on Rust
development, I'm planning to extend it with more Rust-related content.

## Conclusion

Migrating from `fronma` to inline inhouse parsing was rewarding, given
that I reduced dependencies, expanded my knowledge, and add some test
coverage.

For my simple use case, custom parsing is the best choice, and removing
huge dependencies is sometimes the right kind of growth for a project.

`Happy hacking` 🐱
