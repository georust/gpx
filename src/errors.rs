//! errors provides error generics for the gpx parser.

// This gives us our error boilerplate macros.
error_chain!{
    errors {
        /// InvalidChildElement signifies when an element has a child that isn't
        /// valid per the GPX spec.
        InvalidChildElement(child: String, parent: &'static str) {
            description("invalid child element")
            display("invalid child element '{}' in {}", child, parent)
        }

        /// InvalidClosingTag signifies incorrect XML syntax: A tag was closed that
        /// could not be closed at this point.
        InvalidClosingTag(invalid_tag: String, parent: &'static str) {
            description("invalid closing tag")
            display("invalid closing tag '{}' in {}", invalid_tag, parent)
        }

        /// MissingClosingTag signifies incorrect XML syntax: A tag was not closed.
        MissingClosingTag(parent: &'static str) {
            description("missing closing tag")
            display("missing closing tag in element '{}'", parent)
        }

        /// InvalidElementLacksAttribute signifies when an element is missing a
        /// required attribute.
        InvalidElementLacksAttribute(attr: &'static str) {
            description("invalid element, lacks required attribute")
            display("invalid element, lacks required attribute {}", attr)
        }
    }
}
