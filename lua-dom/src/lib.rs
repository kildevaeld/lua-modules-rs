mod document;
pub mod element_ref;
mod matcher;
pub mod node;
mod selection;
mod tree;

pub use self::{
    document::Document,
    matcher::{MatchScope, Matcher},
    selection::Selection,
};

pub mod bindings;

pub use bindings::register_module;

pub use tendril::StrTendril;

#[cfg(test)]
mod test {

    use super::document::*;

    #[test]
    fn test() {
        let dom = Document::parse(
            r#"<ul>
            <li class="class-name">
                <span></span>
            </li>
        </ul>"#,
        );

        let selection = dom.select(".class-name span");

        println!("{:#?}", selection);
    }
}
