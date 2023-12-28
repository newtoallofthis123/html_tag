use std::fmt::Display;

use crate::tags::TagType;

/// A struct representing an HTML tag.
/// The concept of a HTML tag is represented using this
/// struct. It contains all the information needed to
/// construct a HTML string.
///
/// The heart of the crate, it contains all the necessary traits
/// and methods to construct a HTML string.
///
/// # Examples
///
/// ```
/// use html_tag::HtmlTag;
///
/// let mut a = HtmlTag::new("a");
/// a.set_body("Hello World");
/// a.add_class("test");
/// a.set_href("https://example.com");
///
/// assert_eq!(a.to_html(), "<a class=\"test\" href=\"https://example.com\">Hello World</a>");
///
/// ```
/// This also has a `Display` implementation, so you can
/// print it directly.
///
/// Moreover, the elements can be nested, like so:
///
/// ```
/// use html_tag::HtmlTag;
///
/// let mut div = HtmlTag::new("div");
/// div.add_class("test");
/// let mut p = HtmlTag::new("p");
/// p.set_body("Hello World");
/// div.add_child(p);
///
/// assert_eq!(div.to_html(), "<div class=\"test\"><p>Hello World</p></div>");
/// ```
///
/// Hence, you can scaffold a HTML element quite easily.
///
/// # Custom Tags
///
/// You can also use custom tags, like so:
///
/// ```
/// use html_tag::HtmlTag;
///
/// let mut custom = HtmlTag::new("custom");
/// custom.set_body("Hello World");
///
/// assert_eq!(custom.to_html(), "<custom>Hello World</custom>");
/// ```
///
/// Remember, all of these can be nested as well as modifies using
/// the methods provided.
#[derive(Clone, PartialEq, Eq)]
pub struct HtmlTag {
    pub tag_type: TagType,
    pub class_names: Vec<String>,
    pub id: Option<String>,
    pub body: Option<String>,
    pub children: Option<Vec<HtmlTag>>,
    pub custom_attributes: Option<Vec<(String, String)>>,
}

impl HtmlTag {
    /// Creates a new `HtmlTag` with the given tag type.
    ///
    /// The tag type can be any valid HTML tag, or a custom tag.
    /// The crate is smart enough to handle both.
    ///
    /// This initializes the `HtmlTag` with the given tag type,
    /// although none of the other fields are initialized.
    /// Hence, all are set to `None` or empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use html_tag::HtmlTag;
    ///
    /// let mut a = HtmlTag::new("a");
    ///
    /// assert_eq!(a.to_html(), "<a></a>");
    /// ```
    ///
    /// Essentially a initializer for the struct.
    pub fn new(tag_type: &str) -> HtmlTag {
        HtmlTag {
            tag_type: TagType::from(tag_type),
            class_names: Vec::new(),
            id: None,
            body: None,
            children: None,
            custom_attributes: None,
        }
    }

    /// Creates a new `HtmlTag` with the given tag type and body.
    ///
    /// This is a more pragmatic approach to creating a new `HtmlTag`.
    /// You specify the exact tag type, the body, and the class names.
    ///
    /// # Examples
    ///
    /// ```
    /// use html_tag::HtmlTag;
    /// use html_tag::TagType;
    ///
    /// let mut a = HtmlTag::fresh(TagType::A, Some("Hello World"), vec!["test"]);
    ///
    /// assert_eq!(a.to_html(), "<a class=\"test\">Hello World</a>");
    /// ```
    ///
    /// This is the most commonly used scaffold for creating a new `HtmlTag`.
    pub fn fresh(tag_type: TagType, body: Option<&str>, class_names: Vec<&str>) -> HtmlTag {
        HtmlTag {
            tag_type,
            class_names: class_names.iter().map(|s| s.to_string()).collect(),
            id: None,
            body: body.map(|s| s.to_string()),
            children: None,
            custom_attributes: None,
        }
    }

    /// Adds a child of the type `HtmlTag` to the current `HtmlTag`.
    ///
    /// This is used to essentially nest HTML tags.
    ///
    /// # Examples
    ///
    /// ```
    /// use html_tag::HtmlTag;
    ///
    /// let mut div = HtmlTag::new("div");
    /// let mut p = HtmlTag::new("p");  
    /// p.set_body("Hello World");
    /// div.add_child(p);
    ///
    /// assert_eq!(div.to_html(), "<div><p>Hello World</p></div>");
    /// ```
    ///
    /// This needs a mutable reference to the current `HtmlTag`.
    pub fn add_child(&mut self, child: HtmlTag) {
        if let Some(children) = &mut self.children {
            children.push(child);
        } else {
            self.children = Some(vec![child]);
        }
    }

    /// Adds a class name to the current `HtmlTag`.
    pub fn add_class(&mut self, class_name: &str) {
        self.class_names.push(class_name.to_string());
    }

    /// Sets the body of the current `HtmlTag`.
    pub fn set_body(&mut self, body: &str) {
        self.body = Some(body.to_string());
    }

    /// Sets the id of the current `HtmlTag`.
    pub fn set_id(&mut self, id: &str) {
        self.id = Some(id.to_string());
    }

    /// Sets the styles of the current `HtmlTag`.
    pub fn set_style(&mut self, style: &str) {
        self.add_attribute("style", style);
    }

    /// Sets the href of the current `HtmlTag`.
    pub fn set_href(&mut self, href: &str) {
        self.add_attribute("href", href);
    }

    fn get_tags(tag_type: &TagType) -> (String, String) {
        let tag = format!("<{}", tag_type.html());
        let closing_tag = format!("</{}>", tag_type.html());
        (tag, closing_tag)
    }

    fn partial_convert(&self) -> String {
        let mut html_to_return = String::new();
        let (opening_tag, _) = HtmlTag::get_tags(&self.tag_type);
        html_to_return.push_str(&opening_tag);

        if let Some(id) = &self.id {
            html_to_return.push_str(&format!(" id=\"{}\"", id));
        }

        if !self.class_names.is_empty() {
            html_to_return.push_str(&format!(" class=\"{}\"", self.class_names.join(" ")));
        }

        if let Some(custom_attributes) = &self.custom_attributes {
            for (key, value) in custom_attributes {
                html_to_return.push_str(&format!(" {}=\"{}\"", key, value));
            }
        }

        html_to_return
    }

    /// Adds an attribute to the current `HtmlTag`.
    /// This attribute can be a custom attribute, or a
    /// predefined attribute like `class` or `id`.
    ///
    /// # Examples
    ///
    /// ```
    /// use html_tag::HtmlTag;
    ///
    /// let mut div = HtmlTag::new("div");
    /// div.add_attribute("class", "test");
    /// div.add_attribute("id", "test");
    /// div.add_attribute("style", "color: red;");
    ///
    /// assert_eq!(div.to_html(), "<div id=\"test\" class=\"test\" style=\"color: red;\"></div>");
    /// ```
    ///
    /// This is used to add custom attributes as well.
    pub fn add_attribute(&mut self, key: &str, value: &str) {
        match key {
            "class" => self.add_class(value),
            "id" => self.set_id(value),
            _ => self.add_custom_attribute(key, value),
        }
    }

    fn add_custom_attribute(&mut self, key: &str, value: &str) {
        if let Some(custom_attributes) = &mut self.custom_attributes {
            custom_attributes.push((key.to_string(), value.to_string()));
        } else {
            self.custom_attributes = Some(vec![(key.to_string(), value.to_string())]);
        }
    }

    /// Adds multiple custom attributes to the current `HtmlTag`.
    /// You can declare the custom attributes as a vector of tuples
    /// of the form `(&str, &str)`.
    ///
    /// Where the first element of the tuple is the key, and the second
    /// element is the value.
    pub fn add_custom(&mut self, attributes: Vec<(&str, &str)>) {
        let mut custom_attributes = Vec::new();
        for (key, value) in attributes {
            custom_attributes.push((key.to_string(), value.to_string()));
        }
        self.custom_attributes = Some(custom_attributes);
    }

    /// Converts the current `HtmlTag` to a HTML string.
    ///
    /// This is the main form of conversion, and is used
    /// to convert the `HtmlTag` to a HTML string that can
    /// be used in a HTML document.
    ///
    /// # Examples
    ///
    /// ```
    /// use html_tag::HtmlTag;
    ///
    /// let mut div = HtmlTag::new("div");
    /// div.add_class("test");
    /// div.set_id("test");
    ///
    /// assert_eq!(div.to_html(), "<div id=\"test\" class=\"test\"></div>");
    ///
    /// ```
    ///
    /// This method is implemented as the display trait, so you can
    /// print it directly or use it in a format string.
    /// Like this
    ///
    /// ```
    /// use html_tag::HtmlTag;
    ///
    /// let mut div = HtmlTag::new("div");
    /// div.add_class("test");
    /// div.set_id("test");
    ///
    /// println!("{}", div);
    /// ```
    ///
    /// This will print the following: `<div class="test" id="test"></div>`
    pub fn to_html(&self) -> String {
        let mut html = self.partial_convert();
        let (_, closing_tag) = HtmlTag::get_tags(&self.tag_type);

        if let Some(body) = &self.body {
            html.push_str(&format!(">{}</{}>", body, self.tag_type.html()));
            return html;
        } else {
            html.push('>');
        }

        if let Some(children) = &self.children {
            for child in children {
                html.push_str(&child.to_html());
            }
        }

        html.push_str(&closing_tag);

        html
    }
}

impl Display for HtmlTag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_html())
    }
}
