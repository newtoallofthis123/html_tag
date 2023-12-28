//! # HTMLTag
//! The enigmatic way to use HTML in Rust.
//! 
//! HTMLTag is a crate that allows you to create HTML tags in a simple and
//! intuitive way.
//! It aims to replace the need for using HTML in the form of strings or the heavy use of 
//! macros in your code to make it more readable and maintainable.
//! 
//! ## Idea
//! 
//! Strings are not always the best way to represent HTML in your code.
//! You could probably use macros to make it more readable, but that is not always the best
//! solution.
//! 
//! Moreover, you could probably use a templating engine or a library like `html_builder` to
//! generate large HTML documents, however, we are talking niche use cases here.
//! 
//! The idea is to make it easy to create HTML tags in as simple and as "Rusty" way as possible.
//! 
//! ## Usage
//! 
//! ```rust
//! use html_tag::HtmlTag;
//! 
//! let mut a = HtmlTag::new("a");
//! a.set_body("Hello World");
//! a.add_class("test");
//! a.set_href("https://example.com");
//! 
//! assert_eq!(a.to_html(), "<a class=\"test\" href=\"https://example.com\">Hello World</a>");
//! ```
//! 
//! As apparent from the example above, you can essentially let HtmlTag act as a builder or 
//! rather a wrapper around the abstract concept of a HTML tag.
//! 
//! You can also nest tags, like so:
//! 
//! ```rust
//! use html_tag::HtmlTag;
//! 
//! let mut div = HtmlTag::new("div");
//! div.add_class("test");
//! let mut p = HtmlTag::new("p");
//! p.set_body("Hello World");
//! div.add_child(p);
//! 
//! assert_eq!(div.to_html(), "<div class=\"test\"><p>Hello World</p></div>");
//! ```
//! 
//! ## Main Features
//! 
//! - Create HTML tags in a simple and intuitive way.
//! - Nest tags inside each other.
//! - Use custom tags.
//! - Use custom attributes.
//! - Use `Display` trait to print the HTML tag.
//! - No dependencies.
//! 
//! ## Contributing
//! 
//! Contributions are welcome. Please open an issue or a PR if you find any bugs or have any
//! suggestions.
//! 
//! ## License
//! 
//! This project is licensed under the MIT License.

/// HTMLTag Related Stuff
pub mod html;
/// TagType Related Stuff
pub mod tags;

pub use crate::html::HtmlTag;
pub use crate::tags::TagType;

// Tests cause they are important
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_tag() {
        let actual_html = "<p></p>";
        let p = html::HtmlTag::new("p");
        assert_eq!(p.to_html(), actual_html);
    }

    #[test]
    fn single_tag_match() {
        let actual_html = "<p class=\"test\">Hello World</p>";
        let mut p = html::HtmlTag::new("p");
        p.add_class("test");
        p.set_body("Hello World");
        assert_eq!(p.to_html(), actual_html);
    }

    #[test]
    fn children_tag_match() {
        let actual_html = "<div class=\"test\"><p>Hello World</p></div>";
        let mut div = html::HtmlTag::new("div");
        div.add_class("test");
        let mut p = html::HtmlTag::new("p");
        p.set_body("Hello World");
        div.add_child(p);
        assert_eq!(div.to_html(), actual_html);
    }

    #[test]
    fn test_styles_id_classes() {
        let actual_html =
            "<div id=\"test\" class=\"test\" style=\"color: red;\"><p>Hello World</p></div>";
        let mut div = html::HtmlTag::new("div");
        div.add_class("test");
        div.set_id("test");
        div.set_style("color: red;");
        let mut p = html::HtmlTag::new("p");
        p.set_body("Hello World");
        div.add_child(p);
        assert_eq!(div.to_html(), actual_html);
    }

    #[test]
    fn test_href() {
        let actual_html = "<a href=\"https://www.google.com\">Hello World</a>";
        let mut a = html::HtmlTag::new("a");
        a.set_href("https://www.google.com");
        a.set_body("Hello World");
        assert_eq!(a.to_html(), actual_html);
    }

    #[test]
    fn test_custom_tag() {
        let actual_html = "<custom>Hello World</custom>";
        let mut custom = html::HtmlTag::new("custom");
        custom.set_body("Hello World");
        assert_eq!(custom.to_html(), actual_html);
    }

    #[test]
    fn test_custom_attributes() {
        let actual_html = "<div id=\"test\" class=\"test\" style=\"color: red;\" data-test=\"test\"><p>Hello World</p></div>";
        let mut div = html::HtmlTag::new("div");
        div.add_class("test");
        div.set_id("test");
        div.set_style("color: red;");
        div.add_attribute("data-test", "test");
        let mut p = html::HtmlTag::new("p");
        p.set_body("Hello World");
        div.add_child(p);
        assert_eq!(div.to_html(), actual_html);
    }

    #[test]
    fn print_test() {
        let actual_html = "<p class=\"test\">Hello World</p>";
        let mut p = html::HtmlTag::new("p");
        p.add_class("test");
        p.set_body("Hello World");
        assert_eq!(format!("{}", p), actual_html);
    }

    #[test]
    fn ordering_test() {
        let mut tags = vec![
            tags::TagType::P,
            tags::TagType::Div,
            tags::TagType::Span,
            tags::TagType::A,
            tags::TagType::H1,
            tags::TagType::H2,
            tags::TagType::H3,
            tags::TagType::H4,
            tags::TagType::H5,
            tags::TagType::H6,
            tags::TagType::Img,
            tags::TagType::Custom("custom".to_string()),
        ];
        tags.sort();
        tags.reverse();
        assert_eq!(
            tags,
            vec![
                tags::TagType::Div,
                tags::TagType::H1,
                tags::TagType::H2,
                tags::TagType::H3,
                tags::TagType::H4,
                tags::TagType::H5,
                tags::TagType::H6,
                tags::TagType::Img,
                tags::TagType::Custom("custom".to_string()),
                tags::TagType::A,
                tags::TagType::Span,
                tags::TagType::P,
            ]
        );
    }
}
