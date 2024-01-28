use html_tag::styles::{Style, StyleSheet};

fn main() {
    let mut style = StyleSheet::new();
    style.add_style(".wow", "color", "red");
    style.add_style(".wow", "font-size", "20px");
    style.add_style(".wow", "font-family", "sans-serif");

    style.add_style("h1", "color", "blue");
    style.add_style("h1", "font-size", "30px");

    println!("{}", style.get_style_sheet());
    println!("{}", style.get_with_tag());

    // with HtmlTag
    let div = html_tag::HtmlTag::new("div")
        .with_id("wow")
        .embed_style_sheet(&style)
        .with_child(
            html_tag::HtmlTag::new("h1")
                .with_class("wow")
                .with_body("Hello World"),
        );

    println!("{}", div.to_html());
}
