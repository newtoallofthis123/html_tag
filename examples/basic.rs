use html_tag::HtmlTag;

fn main() {
    let mut main = HtmlTag::new("div")
        .with_id("main")
        .with_style("color: red;");

    let to_add = ["Ram", "Jake", "John", "Jill", "Jenny"];

    to_add.iter().enumerate().for_each(|(i, name)| {
        let mut p = HtmlTag::new("p")
            .with_id(&format!("p-{}", i))
            .with_class("person");

        p.set_body(&format!("{}. {}", i + 1, name));

        main.add_child(p);
    });

    println!("{}", main.to_html());
}
