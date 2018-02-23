//! errors provides error generics for the gpx parser.

// This gives us our error boilerplate macros.
error_chain!{
    errors {
        /// InvalidChildElement signifies when an element has a child that isn't
        /// valid per the GPX spec.
        InvalidChildElement(child: String, parent: &'static str) {
            description("invalid child element")
            display("invalid child element '{}' in {}", child, String::from(*parent))
        }

        /// InvalidElementLacksAttribute signifies when an element is missing a
        /// required attribute.
        InvalidElementLacksAttribute(attr: &'static str) {
            description("invalid element, lacks required attribute")
            display("invalid element, lacks required attribute {}", String::from(*attr))
        }
    }
}
