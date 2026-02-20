/// Convert a PascalCase or camelCase string to snake_case.
///
/// Handles acronyms: "ADEOfBuilding" → "ade_of_building", "lod2Solid" → "lod2_solid".
pub fn to_snake_case(s: &str) -> String {
    let chars: Vec<char> = s.chars().collect();
    let mut result = String::with_capacity(s.len() + 4);
    for (i, &c) in chars.iter().enumerate() {
        if c.is_uppercase() {
            if i > 0 {
                let prev = chars[i - 1];
                let next_is_lower = chars.get(i + 1).is_some_and(|nc| nc.is_lowercase());
                // Insert _ before uppercase if:
                // - previous was lowercase/digit (word → new Word)
                // - previous was uppercase AND next is lowercase (acronym boundary: ADEOf → ade_of)
                if prev.is_lowercase() || prev.is_ascii_digit() || (prev.is_uppercase() && next_is_lower) {
                    result.push('_');
                }
            }
            result.push(c.to_lowercase().next().unwrap());
        } else {
            result.push(c);
        }
    }
    result
}

/// Convert a snake_case or camelCase string to PascalCase.
pub fn to_pascal_case(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    let mut capitalize_next = true;
    for c in s.chars() {
        if c == '_' || c == '-' {
            capitalize_next = true;
        } else if capitalize_next {
            result.push(c.to_uppercase().next().unwrap());
            capitalize_next = false;
        } else {
            result.push(c);
        }
    }
    result
}

/// Escape Rust keywords used as identifiers by appending `_` suffix.
pub fn escape_keyword(name: &str) -> String {
    match name {
        "type" | "move" | "ref" | "self" | "super" | "crate" | "mod" | "fn" | "let" | "mut"
        | "pub" | "use" | "as" | "in" | "for" | "if" | "else" | "while" | "loop" | "match"
        | "return" | "break" | "continue" | "struct" | "enum" | "trait" | "impl" | "static"
        | "const" | "where" | "async" | "await" | "dyn" | "abstract" | "become" | "box"
        | "do" | "final" | "macro" | "override" | "priv" | "typeof" | "unsized" | "virtual"
        | "yield" | "try" | "class" => {
            let mut escaped = name.to_string();
            escaped.push('_');
            escaped
        }
        _ => name.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_snake_case() {
        assert_eq!(to_snake_case("AbstractBuilding"), "abstract_building");
        assert_eq!(to_snake_case("CityGML"), "city_gml");
        assert_eq!(to_snake_case("lod2Solid"), "lod2_solid");
        assert_eq!(to_snake_case("ADEOfBuilding"), "ade_of_building");
    }

    #[test]
    fn test_pascal_case() {
        assert_eq!(to_pascal_case("abstract_building"), "AbstractBuilding");
        assert_eq!(to_pascal_case("city_gml"), "CityGml");
    }

    #[test]
    fn test_escape_keyword() {
        assert_eq!(escape_keyword("type"), "type_");
        assert_eq!(escape_keyword("class"), "class_");
        assert_eq!(escape_keyword("name"), "name");
    }
}
