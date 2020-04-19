use std::collections::HashMap;

pub fn pretty_print_tree<T: std::fmt::Display + std::fmt::Debug + std::convert::AsRef<[u8]>>(
    tree_in_print_order: Vec<(usize, T)>,
    encoder: Option<fn(T) -> std::string::String>,
) {
    let mut map: HashMap<usize, i64> = HashMap::new();
    let mut last_level = 0_usize;
    for leaf in tree_in_print_order {
        let pos = leaf.0;
        let val = leaf.1;

        match map.get_mut(&pos) {
            Some(v) => *v += 1,
            None => {
                map.insert(pos, 1);
            }
        }

        let is_end = 2_i64.pow(pos as u32) == *map.get(&pos).unwrap();

        let mut sym = "├─ ";
        if is_end {
            sym = "╰─ ";
            last_level = last_level + 1;
        }

        let mut _spacer = std::iter::repeat("    ")
            .take(last_level)
            .collect::<String>();
        if is_end {
            _spacer = std::iter::repeat("    ").take(pos).collect::<String>();
        }

        let mut h = 0;
        if last_level < pos {
            h = pos - last_level
        }
        let lines = std::iter::repeat("│   ").take(h).collect::<String>();
        let spacer = format!("{}{}", _spacer, lines);

        let _final = match encoder {
            Some(ecdr) => format!("{}{} {}", spacer, sym, ecdr(val)),
            None => format!("{}{} {}", spacer, sym, val),
        };

        // let _final = format!("{}{} {:?}", spacer, sym, val); //base64::encode(val));
        println!("{}", _final);
    }
}

pub fn pretty_print_tree_bytes<T: std::fmt::Debug + std::convert::AsRef<[u8]>>(
    tree_in_print_order: Vec<(usize, T)>,
    encoder: Option<fn(T) -> std::string::String>,
) {
    let mut map: HashMap<usize, i64> = HashMap::new();
    let mut last_level = 0_usize;
    for leaf in tree_in_print_order {
        let pos = leaf.0;
        let val = leaf.1;

        match map.get_mut(&pos) {
            Some(v) => *v += 1,
            None => {
                map.insert(pos, 1);
            }
        }

        let is_end = 2_i64.pow(pos as u32) == *map.get(&pos).unwrap();

        let mut sym = "├─ ";
        if is_end {
            sym = "╰─ ";
            last_level = last_level + 1;
        }

        let mut _spacer = std::iter::repeat("    ")
            .take(last_level)
            .collect::<String>();
        if is_end {
            _spacer = std::iter::repeat("    ").take(pos).collect::<String>();
        }

        let mut h = 0;
        if last_level < pos {
            h = pos - last_level
        }
        let lines = std::iter::repeat("│   ").take(h).collect::<String>();
        let spacer = format!("{}{}", _spacer, lines);

        let _final = match encoder {
            Some(ecdr) => format!("{}{} {}", spacer, sym, ecdr(val)),
            None => format!("{}{} {:?}", spacer, sym, val),
        };

        // let _final = format!("{}{} {:?}", spacer, sym, val); //base64::encode(val));
        println!("{}", _final);
    }
}
