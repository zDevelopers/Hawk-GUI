use std::collections::BTreeMap;
use std::collections::HashMap;

use rocket_contrib::templates::tera::{FilterFn, from_value, Result, to_value, Value};

/// Parses Minecraft color codes into HTML formatting.
///
/// # Example
///
/// ```
/// use lib::minecraft::parse_color_codes;
/// assert_eq!(parse_color_codes(String::from("§2Dark green")), String::from("<span style=\"color: #00AA00;\">Dark green</span>"));
/// ```
pub fn parse_color_codes(raw_string: String) -> String {
    if raw_string.eq("§") {
        return String::from("§");
    }

    let mut formatted_string = String::with_capacity(raw_string.len() * 2);

    let mut colors_map = HashMap::new();
    let mut formattings_map = HashMap::new();

    colors_map.insert('0', "000000");
    colors_map.insert('1', "0000AA");
    colors_map.insert('2', "00AA00");
    colors_map.insert('3', "00AAAA");
    colors_map.insert('4', "AA0000");
    colors_map.insert('5', "AA00AA");
    colors_map.insert('6', "FFAA00");
    colors_map.insert('7', "AAAAAA");
    colors_map.insert('8', "555555");
    colors_map.insert('9', "5555FF");
    colors_map.insert('a', "55FF55");
    colors_map.insert('b', "55FFFF");
    colors_map.insert('c', "FF5555");
    colors_map.insert('d', "FF55FF");
    colors_map.insert('e', "FFFF55");
    colors_map.insert('f', "FFFFFF");

    formattings_map.insert('l', "font-weight: bold;");
    formattings_map.insert('m', "text-decoration: line-through;");
    formattings_map.insert('n', "text-decoration: underline;");
    formattings_map.insert('o', "font-style: italic;");
    formattings_map.insert('r', "color: inherit;");

    let colors_map = colors_map;
    let formattings_map = formattings_map;

    let mut code_detected = false;
    let mut nested_formatters = 0u8;

    raw_string.chars().for_each(|c| match c {

        // Control character
        '§' => code_detected = true,

        // Color codes. They reset all past formattings.
        '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | 'a' | 'b' | 'c' | 'd' | 'e' | 'f' | 'A' | 'B' | 'C' | 'D' | 'E' | 'F' => if code_detected {
            code_detected = false;

            // We close all past formatting <span> tags
            for _ in 0..nested_formatters { formatted_string += &String::from("</span>") }

            formatted_string += &format!("<span style=\"color: #{};\">", colors_map.get(&match c {
                'A' => 'a',
                'B' => 'b',
                'C' => 'c',
                'D' => 'd',
                'E' => 'e',
                'F' => 'f',
                _   => c
            }).unwrap_or(&"inherit"));
            nested_formatters = 1;
        } else { formatted_string += &c.to_string() },

        // Formatting codes. They add their formatting to the previous ones.
        'l' | 'm' | 'n' | 'o' | 'L' | 'M' | 'N' | 'O' => if code_detected {
            code_detected = false;

            formatted_string += &format!("<span style=\"{}\">", formattings_map.get(&match c {
                'L' => 'l',
                'M' => 'm',
                'N' => 'n',
                'O' => 'o',
                _   => c
            }).unwrap_or(&""));
            nested_formatters += 1;
        } else { formatted_string += &c.to_string() },

        // Reset code. It removes all existing formatting.
        'r' | 'R' => if code_detected {
            code_detected = false;
            for _ in 0..nested_formatters { formatted_string += &String::from("</span>") }
            nested_formatters = 0;
        } else { formatted_string += &c.to_string() },

        _ => {
            // If we had a formatter sign before but it's not
            // a correct code, we add the sign we left previously
            // to the string.
            if code_detected {
                formatted_string += &String::from("§")
            }
            formatted_string += &c.to_string();
            code_detected = false;
        }
    });

    // We close all remaining <span> tags, if any
    for _ in 0..nested_formatters { formatted_string += &String::from("</span>") }

    formatted_string.shrink_to_fit();

    let formatted_string = formatted_string;

    formatted_string
}


#[cfg(test)]
mod tests {
    use super::*;

    fn assert_pcc(raw: &'static str, should_be: &'static str) {
        assert_eq!(parse_color_codes(String::from(raw)), String::from(should_be));
    }

    #[test]
    fn colors_only() {
        assert_pcc("§2Dark green", "<span style=\"color: #00AA00;\">Dark green</span>");
        assert_pcc("§2§4Dark red", "<span style=\"color: #00AA00;\"></span><span style=\"color: #AA0000;\">Dark red</span>");
        assert_pcc("§5Purple §dpink", "<span style=\"color: #AA00AA;\">Purple </span><span style=\"color: #FF55FF;\">pink</span>");
        assert_pcc("Inherit and §dpink", "Inherit and <span style=\"color: #FF55FF;\">pink</span>");

        assert_pcc("§5Purple §Dpink", "<span style=\"color: #AA00AA;\">Purple </span><span style=\"color: #FF55FF;\">pink</span>");
        assert_pcc("Inherit and §Dpink", "Inherit and <span style=\"color: #FF55FF;\">pink</span>");
    }

    #[test]
    fn formatters_only() {
        assert_pcc("§lBold", "<span style=\"font-weight: bold;\">Bold</span>");
        assert_pcc("§lBold + §oItalics + §mStrikethrough", "<span style=\"font-weight: bold;\">Bold + <span style=\"font-style: italic;\">Italics + <span style=\"text-decoration: line-through;\">Strikethrough</span></span></span>");
        assert_pcc("§l§m§n§oEVERYTHING", "<span style=\"font-weight: bold;\"><span style=\"text-decoration: line-through;\"><span style=\"text-decoration: underline;\"><span style=\"font-style: italic;\">EVERYTHING</span></span></span></span>");

        assert_pcc("§LBold", "<span style=\"font-weight: bold;\">Bold</span>");
        assert_pcc("§LBold + §oItalics + §mStrikethrough", "<span style=\"font-weight: bold;\">Bold + <span style=\"font-style: italic;\">Italics + <span style=\"text-decoration: line-through;\">Strikethrough</span></span></span>");
        assert_pcc("§L§M§N§OEVERYTHING", "<span style=\"font-weight: bold;\"><span style=\"text-decoration: line-through;\"><span style=\"text-decoration: underline;\"><span style=\"font-style: italic;\">EVERYTHING</span></span></span></span>");
    }

    #[test]
    fn both_colors_and_formatters() {
        assert_pcc("§2Dark §lgreen", "<span style=\"color: #00AA00;\">Dark <span style=\"font-weight: bold;\">green</span></span>");
        assert_pcc("§2Dark §lgreen §band aqua", "<span style=\"color: #00AA00;\">Dark <span style=\"font-weight: bold;\">green </span></span><span style=\"color: #55FFFF;\">and aqua</span>");

        assert_pcc("§2Dark §Lgreen", "<span style=\"color: #00AA00;\">Dark <span style=\"font-weight: bold;\">green</span></span>");
        assert_pcc("§2Dark §Lgreen §Band aqua", "<span style=\"color: #00AA00;\">Dark <span style=\"font-weight: bold;\">green </span></span><span style=\"color: #55FFFF;\">and aqua</span>");
    }

    #[test]
    fn colors_resets_formatters() {
        assert_pcc("§lBold §6No longer bold (but gold!)", "<span style=\"font-weight: bold;\">Bold </span><span style=\"color: #FFAA00;\">No longer bold (but gold!)</span>");
        assert_pcc("§LBold §6No longer bold (but gold!)", "<span style=\"font-weight: bold;\">Bold </span><span style=\"color: #FFAA00;\">No longer bold (but gold!)</span>");
    }

    #[test]
    fn reset() {
        assert_pcc("§2Dark §rgreen", "<span style=\"color: #00AA00;\">Dark </span>green");
        assert_pcc("§2§4Dark §rred", "<span style=\"color: #00AA00;\"></span><span style=\"color: #AA0000;\">Dark </span>red");
        assert_pcc("§5Purple §dpi§rnk", "<span style=\"color: #AA00AA;\">Purple </span><span style=\"color: #FF55FF;\">pi</span>nk");
        assert_pcc("Inherit and §dpi§rnk", "Inherit and <span style=\"color: #FF55FF;\">pi</span>nk");

        assert_pcc("§2Dark §Rgreen", "<span style=\"color: #00AA00;\">Dark </span>green");
        assert_pcc("§2§4Dark §Rred", "<span style=\"color: #00AA00;\"></span><span style=\"color: #AA0000;\">Dark </span>red");
        assert_pcc("§5Purple §dpi§Rnk", "<span style=\"color: #AA00AA;\">Purple </span><span style=\"color: #FF55FF;\">pi</span>nk");
        assert_pcc("Inherit and §dpi§Rnk", "Inherit and <span style=\"color: #FF55FF;\">pi</span>nk");
    }

    #[test]
    fn not_a_formatter() {
        assert_pcc("§2Dark §r§green", "<span style=\"color: #00AA00;\">Dark </span>§green");
        assert_pcc("§i", "§i");
        assert_pcc("§", "§");
    }
}
