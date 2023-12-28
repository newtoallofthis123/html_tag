use std::fmt::Display;

/// A struct that represents a HTML tag.
///
/// This has some of the most used HTML tags as enums.
/// However, there is nothing inherently special about these tags.
/// These are just to make it a bit more convenient to use.
///
/// You can essentially declare a `p` tag using the P enum or using the
/// `Custom` enum.
///
/// The only thing that determines whether a tag is valid or not is the
/// `TagType` enum.
/// Moreover, the `TagType` enum also implements `Ord` and `PartialOrd` traits
/// so that you can sort the tags and use it in a `BTreeMap` or `BTreeSet`.
///
/// # Examples
///
/// ```
/// use html_tag::tags::TagType;
///
/// let mut tags = vec![
///   TagType::P,
///   TagType::Div,
///   TagType::Span,
///   TagType::A,
/// ];
///
/// tags.sort();
/// tags.reverse();
///
/// assert_eq!(tags, vec![
///   TagType::Div,
///   TagType::A,
///   TagType::Span,
///   TagType::P,
/// ]);
///
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TagType {
    P,
    Div,
    Span,
    A,
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
    Img,
    Table,
    Tr,
    Td,
    Th,
    Custom(String),
}

impl TagType {
    /// Creates a new `TagType` from the given string.
    ///
    /// Tries to essentially convert the string to a `TagType` enum.
    /// If the string is not a valid HTML tag, then it will be converted
    /// to a `Custom` tag.
    ///
    /// # Examples
    ///
    /// ```
    /// use html_tag::TagType;
    ///
    /// let tag = TagType::from("p");
    /// assert_eq!(tag, TagType::P);
    /// ```
    ///
    /// If a tag is not valid, then it will be converted to a `Custom` tag.
    pub fn from(tag: &str) -> TagType {
        let tag = tag.to_lowercase();
        match tag.as_str() {
            "p" => TagType::P,
            "div" => TagType::Div,
            "span" => TagType::Span,
            "a" => TagType::A,
            "h1" => TagType::H1,
            "h2" => TagType::H2,
            "h3" => TagType::H3,
            "h4" => TagType::H4,
            "h5" => TagType::H5,
            "h6" => TagType::H6,
            "img" => TagType::Img,
            "table" => TagType::Table,
            "tr" => TagType::Tr,
            "td" => TagType::Td,
            "th" => TagType::Th,
            _ => TagType::Custom(tag.to_string()),
        }
    }

    /// Returns the HTML representation of the tag.
    ///
    /// # Examples
    ///
    /// ```
    /// use html_tag::TagType;
    ///
    /// let tag = TagType::from("p");
    /// assert_eq!(tag.html(), "p");
    /// ```
    ///
    /// This is essentially the same as the `Display` trait.
    pub fn html(&self) -> String {
        match self {
            TagType::P => "p".to_string(),
            TagType::Div => "div".to_string(),
            TagType::Span => "span".to_string(),
            TagType::A => "a".to_string(),
            TagType::H1 => "h1".to_string(),
            TagType::H2 => "h2".to_string(),
            TagType::H3 => "h3".to_string(),
            TagType::H4 => "h4".to_string(),
            TagType::H5 => "h5".to_string(),
            TagType::H6 => "h6".to_string(),
            TagType::Img => "img".to_string(),
            TagType::Custom(tag) => tag.to_string(),
            TagType::Table => "table".to_string(),
            TagType::Tr => "tr".to_string(),
            TagType::Td => "td".to_string(),
            TagType::Th => "th".to_string(),
        }
    }
}

impl Display for TagType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.html())
    }
}

impl PartialOrd for TagType {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for TagType {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (&self, other) {
            (TagType::Div, _) => std::cmp::Ordering::Greater,
            (_, TagType::Div) => std::cmp::Ordering::Less,
            (TagType::H1, _) => std::cmp::Ordering::Greater,
            (_, TagType::H1) => std::cmp::Ordering::Less,
            (TagType::H2, _) => std::cmp::Ordering::Greater,
            (_, TagType::H2) => std::cmp::Ordering::Less,
            (TagType::H3, _) => std::cmp::Ordering::Greater,
            (_, TagType::H3) => std::cmp::Ordering::Less,
            (TagType::H4, _) => std::cmp::Ordering::Greater,
            (_, TagType::H4) => std::cmp::Ordering::Less,
            (TagType::H5, _) => std::cmp::Ordering::Greater,
            (_, TagType::H5) => std::cmp::Ordering::Less,
            (TagType::H6, _) => std::cmp::Ordering::Greater,
            (_, TagType::H6) => std::cmp::Ordering::Less,
            (TagType::Img, _) => std::cmp::Ordering::Greater,
            (_, TagType::Img) => std::cmp::Ordering::Less,
            (TagType::Custom(_), _) => std::cmp::Ordering::Greater,
            (_, TagType::Custom(_)) => std::cmp::Ordering::Less,
            (TagType::P, _) => std::cmp::Ordering::Less,
            (_, TagType::P) => std::cmp::Ordering::Greater,
            (TagType::Span, _) => std::cmp::Ordering::Less,
            (_, TagType::Span) => std::cmp::Ordering::Greater,
            (TagType::A, _) => std::cmp::Ordering::Less,
            (_, _) => std::cmp::Ordering::Equal,
        }
    }
}
