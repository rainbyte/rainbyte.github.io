use crate::{PageHeaders, PostHeaders};

#[test]
fn page_header_empty_contents() {
    let contents = "";
    let result = PageHeaders::parse(contents);
    assert!(result.is_err());
}

#[test]
fn page_header_empty_headers() {
    let contents = "---\n\
        ---";
    let result = PageHeaders::parse(contents);
    assert!(result.is_err());
}

#[test]
fn page_header_no_headers() {
    let contents = "\n\
        abc\n\
        def";
    let result = PageHeaders::parse(contents);
    assert!(result.is_err());
}

#[test]
fn page_header_empty_title() {
    let contents = "---\n\
        title:\n\
        ---\n\
        abc\n\
        def";
    let result = PageHeaders::parse(contents);
    assert!(result.is_err());
}

#[test]
fn page_header_single_word_title() {
    let contents = "---\n\
        title: foo\n\
        ---\n\
        abc\n\
        def";
    let result = PageHeaders::parse(contents);
    let fronma = result.unwrap();
    assert_eq!(
        "foo".to_string(),
        fronma.headers.title
    );
    assert_eq!(
        "abc\n\
        def",
        fronma.body
    )
}

#[test]
fn page_header_multi_word_title() {
    let contents = "---\n\
        title: foo bar baz quux\n\
        ---\n\
        abc\n\
        def";
    let result = PageHeaders::parse(contents);
    let fronma = result.unwrap();
    assert_eq!(
        "foo bar baz quux".to_string(),
        fronma.headers.title
    );
    assert_eq!(
        "abc\n\
        def",
        fronma.body
    )
}

//----------------------------------------------------------------------

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
            title: "How-to decrease gnome title-bar height".to_string(),
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
