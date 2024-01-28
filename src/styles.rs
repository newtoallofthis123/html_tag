use std::collections::BTreeMap;

pub type StyleSheet = BTreeMap<String, BTreeMap<String, String>>;
pub type Class = BTreeMap<String, String>;

pub trait Style {
    fn new() -> Self;
    fn get_style_sheet(&self) -> String;
    fn get_with_tag(&self) -> String;
    fn add_style(&mut self, selector: &str, property: &str, value: &str);
    fn add_class(&mut self, selector: &str, properties: BTreeMap<String, String>);
    fn with_style(&mut self, selector: &str, property: &str, value: &str) -> Self;
    fn with_class(&mut self, selector: &str, properties: BTreeMap<String, String>) -> Self;
}

impl Style for StyleSheet {
    fn new() -> Self {
        BTreeMap::new()
    }

    fn get_style_sheet(&self) -> String {
        let mut final_styles = String::new();
        for (selector, properties) in self {
            final_styles.push_str(&format!("{} {{\n", selector));
            for (property, value) in properties {
                final_styles.push_str(&format!("    {}: {};\n", property, value));
            }
            final_styles.push_str("}\n");
        }
        final_styles
    }

    fn get_with_tag(&self) -> String {
        let mut final_styles = String::from("<style>\n");
        final_styles.push_str(&self.get_style_sheet());
        final_styles.push_str("</style>\n");
        final_styles
    }

    fn add_style(&mut self, selector: &str, property: &str, value: &str) {
        if self.contains_key(selector) {
            self.get_mut(selector)
                .unwrap()
                .insert(property.to_string(), value.to_string());
        } else {
            let mut new_style = BTreeMap::new();
            new_style.insert(property.to_string(), value.to_string());
            self.insert(selector.to_string(), new_style);
        }
    }

    fn add_class(&mut self, selector: &str, properties: BTreeMap<String, String>) {
        if self.contains_key(selector) {
            let current_properties = self.get_mut(selector).unwrap();
            for (property, value) in properties {
                current_properties.insert(property, value);
            }
        } else {
            self.insert(selector.to_string(), properties);
        }
    }

    fn with_style(&mut self, selector: &str, property: &str, value: &str) -> Self {
        let mut new_style = self.clone();
        new_style.add_style(selector, property, value);
        new_style
    }

    fn with_class(&mut self, selector: &str, properties: BTreeMap<String, String>) -> Self {
        let mut new_style = self.clone();
        new_style.add_class(selector, properties);
        new_style
    }
}

pub fn convert_to_styles(class: Class) -> String {
    let mut styles = String::new();
    for (property, value) in class {
        styles.push_str(&format!("{}: {};", property, value));
    }
    styles
}

pub fn sanitize_styles(styles: String) -> String {
    styles.replace(['\n', '\t'], "").replace(' ', "")
}
