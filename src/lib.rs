use digest::Digest;
use sha2::Sha256;
use std::collections::HashMap;

// this is a generic function for hashing
// it takes a mutable ref, and copies the
// output into that buffer
pub fn hash<D: Digest>(data: &str, output: &mut [u8]) {
    let mut hasher = D::new();
    hasher.input(data.as_bytes());
    output.copy_from_slice(hasher.result().as_slice())
}

// This function build the prelabel based on the
// the typed hash map format
// len(child.type), child.type, len(child.label), child.label
// this is a helper function
pub fn format_pre_label(data: [u8; 32]) -> String {
    return format!(
        "[{}]-[{}]-[{}]-[{}]",
        // "{}{}{}{}",
        "0",
        "",
        data.len(),
        base64::encode(data)
    );
}

pub fn format_pre_label_string(data: String) -> String {
    let mut buf: [u8; 32] = [0; 32];
    hash::<Sha256>(&data, &mut buf);

    return format!(
        "[{}]-[{}]-[{}]-[{}]",
        // "{}{}{}{}",
        "0",
        "",
        buf.len(),
        // data //
        base64::encode(buf)
    );
}

// // generic version format_prelabel::<String> or format_prelabel::<[0; 32]>
// pub fn format_prelabel<T>(data: T) -> String {
//     let mut buf: [u8; 32] = [0; 32];
//     hash::<Sha256>(&data, &mut buf);

//     return format!(
//         "[{}]-[{}]-[{}]-[{}]",
//         // "{}{}{}{}",
//         "0",
//         "",
//         buf.len(),
//         // data //
//         base64::encode(buf)
//     );
// }

//
pub fn get_parents_for_leaf_nodes(data: &Vec<(&str, &str)>) -> Vec<[u8; 32]> {
    let mut nodes = Vec::new();
    for i in (0..&data.len() - 1).step_by(2) {
        let value1 = &data[i];
        let value2 = &data[i + 1];
        let mut buf: [u8; 32] = [0; 32];

        let _my_data1 = format!(
            "{}{}{}{}",
            value1.0.len(),
            value1.0,
            value1.1.len(),
            value1.1
        );
        let _my_data2 = format!(
            "{}{}{}{}",
            value2.0.len(),
            value2.0,
            value2.1.len(),
            value2.1
        );
        let prelabel = format!("{}{}", _my_data1, _my_data2);
        hash::<Sha256>(&prelabel, &mut buf);
        nodes.push(buf.clone())
    }
    nodes
}

pub fn get_parent_for_internal_nodes(leaves: Vec<[u8; 32]>) -> Vec<[u8; 32]> {
    let mut nodes = Vec::new();
    for i in (0..&leaves.len() - 1).step_by(2) {
        let _left = &leaves[i];
        let _right = &leaves[i + 1];
        let mut buf: [u8; 32] = [0; 32];
        let _my_data1 = format_pre_label(*_left);
        let _my_data2 = format_pre_label(*_right);
        let prelabel = format!("{}{}", _my_data1, _my_data2);
        hash::<Sha256>(&prelabel, &mut buf);
        nodes.push(buf.clone())
    }
    if &leaves.len() % 2 != 0 {
        nodes.push(*leaves.last().unwrap());
    }
    nodes
}
pub fn get_prelabels_for_leaf_nodes(data: &Vec<(&str, &str)>) -> Vec<String> {
    let mut nodes = Vec::new();
    for i in (0..&data.len() - 1).step_by(2) {
        let value1 = &data[i];
        let value2 = &data[i + 1];
        let mut buf: [u8; 32] = [0; 32];

        let _my_data1 = format!(
            "{}{}{}{}",
            value1.0.len(),
            value1.0,
            value1.1.len(),
            value1.1
        );
        let _my_data2 = format!(
            "{}{}{}{}",
            value2.0.len(),
            value2.0,
            value2.1.len(),
            value2.1
        );
        let prelabel = format!("{}{}", _my_data1, _my_data2);
        hash::<Sha256>(&prelabel, &mut buf);
        nodes.push(prelabel.clone())
    }
    nodes
}

pub fn get_prelabels_for_internal_nodes(leaves: Vec<String>) -> Vec<String> {
    let mut nodes = Vec::new();
    for i in (0..&leaves.len() - 1).step_by(2) {
        let _left = &leaves[i];
        let _right = &leaves[i + 1];
        let _my_data1 = format_pre_label_string(_left.clone());
        let _my_data2 = format_pre_label_string(_right.clone());
        let prelabel = format!("{}{}", _my_data1, _my_data2);
        nodes.push(prelabel.clone())
    }
    if &leaves.len() % 2 != 0 {
        let _right = leaves.last().unwrap();
        let _my_data2 = format_pre_label_string(_right.clone());
        let prelabel = format!("{}{}", _my_data2, _my_data2);
        nodes.push(prelabel);
    }
    nodes
}

pub fn index_tree(data: Vec<Vec<[u8; 32]>>) -> Vec<(usize, [u8; 32])> {
    let depth = data.len();
    let mut leaves: Vec<(usize, [u8; 32])> = Vec::new();

    // create first level tuple with fixed depth
    let mut firstlevel = Vec::new();
    for s in data.last().unwrap().to_vec() {
        firstlevel.push((depth - 1, s));
    }
    leaves.extend(firstlevel);

    // start at 2 since we fixed above
    let mut distance_from_bottom = 2;

    loop {
        if distance_from_bottom > depth {
            break;
        }

        let len_level_current = leaves.len();
        let level_above_current = data[depth - distance_from_bottom].clone();
        let len_level_above_current = level_above_current.len();

        let y = (len_level_current / len_level_above_current) + 1;
        let mut i = 0;

        for item in level_above_current {
            let tail = leaves.len() - i;
            leaves.push((depth - distance_from_bottom, item));
            leaves[i..].rotate_left(tail);
            i = i + y;
        }

        distance_from_bottom = distance_from_bottom + 1;
    }
    leaves
}

pub fn index_tree_prelabels(data: Vec<Vec<String>>) -> Vec<(usize, String)> {
    let depth = data.len();
    let mut leaves: Vec<(usize, String)> = Vec::new();

    // create first level tuple with fixed depth
    let mut firstlevel = Vec::new();
    for s in data.last().unwrap().to_vec() {
        firstlevel.push((depth - 1, s));
    }
    leaves.extend(firstlevel);

    // start at 2 since we fixed above
    let mut distance_from_bottom = 2;

    loop {
        if distance_from_bottom > depth {
            break;
        }

        let len_level_current = leaves.len();
        let level_above_current = data[depth - distance_from_bottom].clone();
        let len_level_above_current = level_above_current.len();

        let y = (len_level_current / len_level_above_current) + 1;
        let mut i = 0;

        for item in level_above_current {
            let tail = leaves.len() - i;
            leaves.push((depth - distance_from_bottom, item));
            leaves[i..].rotate_left(tail);
            i = i + y;
        }

        distance_from_bottom = distance_from_bottom + 1;
    }
    leaves
}

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

pub fn make_tree(data: &Vec<(&str, &str)>) -> Vec<Vec<[u8; 32]>> {
    let mut levels = Vec::new();
    let mut leaves = Vec::new();

    // make typed k/v's into hashes
    // we H(type,value)
    for _data in data {
        let mut buf: [u8; 32] = [0; 32];
        let _my_data = format!("{}{}", _data.0, _data.1);
        hash::<Sha256>(&_my_data, &mut buf);
        leaves.push(buf)
    }
    // this is out bottom level - the leaf level
    levels.push(leaves.clone());

    // next we get one level up from the leaves
    // these nodes are dependent on the values in the leaves
    leaves = get_parents_for_leaf_nodes(&data);
    levels.push(leaves.clone());

    // after that, all the types are ""
    // and the values are the labels
    // so we can hardcode 0 as the first len
    // and "" as the value
    while &leaves.len() > &1 {
        leaves = get_parent_for_internal_nodes(leaves);
        levels.push(leaves.clone());
    }
    // we want to see root first
    levels.reverse();
    levels
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
