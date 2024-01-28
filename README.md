# HTML Tag

HTML Tag, the enigmatic way to write HTML in your Rust code.

## What is it?

HTML Tag is a wrapper struct that allows you to write HTML in your Rust code.
It essentially removes the use of raw strings or the need to use macros such as the `html!` macro from the `html` crate to return a `String` of HTML.

## Why?

When I was experimenting with the HTMX library, I found that I was writing a lot of raw HTML strings in my Rust code.

Stuff like this

```rust
find_files(&search.q).iter().for_each(|x| {
        html += format!("<p><a class='link' href='/assets/{}'>{}</a></p>", x, x).as_str();
});
```

was not very readable and was prone to errors.
Hence I wanted to find a way to write HTML in my Rust code in a more readable and debug-able way.

Enter HTML Tag.

Using HTML Tag, the above code can be written as

```rust
html = HtmlTag::new("div");
find_files(&search.q).iter().for_each(|x| {
    let mut tag = HtmlTag::new("p")
        .add_child(HtmlTag::new("a")
        .add_class("link")
        .set_href(format!("/assets/{}", x))
        .set_body(x));
    html += tag.to_string();
});
```

## How to use it?

HTML Tag is very simple to use.
Basically, you create a new `HtmlTag` struct with the tag name as the argument.
Then you can add attributes to the tag using the `add_attribute` method.
However, for the common attributes and classes, there are methods that you can use to add them.

You can go through the [documentation](https://docs.rs/html_tag) for more information.

## When to use it?

First up, html_tag is not a replacement for the `html!` macro from the `html` crate.
It is more of a wrapper struct that allows you to write HTML in your Rust code.

You can use it when you want to write HTML in your Rust code, especially when you are dealing with HATEOAS APIs.

If you want full HTML generation you are better off using something like [html_builder](https://docs.rs/html-builder/0.5.1/html_builder/)

Moreover, this also means that you can have modular control of the html string that you are generating.

Hence, you can do something like this for example

```rust
let mut html = HtmlTag::new("div");
let interested = true
if interested {
    html.with_child(HtmlTag::new("p").with_body("I am interested"));
} else {
    html.with_child(HtmlTag::new("p").with_body("I am not interested"));
}
```

This is a very simple example, but you can see how you can have modular control over the HTML string that you are generating.

## How to contribute?

If you have any ideas on how to improve this crate, feel free to open an issue or a pull request.

I am a complete noob when it comes to Rust and especially when it comes to writing crates that are well optimised and well documented.
So any help is appreciated.

Some areas that I would like to improve on are

- [ ] Make the crate more memory efficient
- [ ] Make the crate more performant
- [ ] Improve the documentation
- [ ] Add more examples

Any of the above or any other improvements are welcome.

## License

This crate is licensed under the MIT License.
You can read the license [here](LICENSE).
