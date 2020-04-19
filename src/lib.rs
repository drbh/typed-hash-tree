use digest::Digest;
use sha2::Sha256;

pub mod print;

// this is a generic function for hashing
// it takes a mutable ref, and copies the
// output into that buffer
pub fn hash<D: Digest>(data: &str, output: &mut [u8]) {
    let mut hasher = D::new();
    hasher.input(data.as_bytes());
    output.copy_from_slice(hasher.result().as_slice())
}

#[derive(Debug)]
pub enum NodeTreeTypes {
    Bytes([u8; 32]),
    String(String),
}

//                 _       _          _
//                | |     | |        | |
//  _ __  _ __ ___| | __ _| |__   ___| |
// | '_ \| '__/ _ \ |/ _` | '_ \ / _ \ |
// | |_) | | |  __/ | (_| | |_) |  __/ |
// | .__/|_|  \___|_|\__,_|_.__/ \___|_|
// | |
// |_|

// This function build the prelabel based on the
// the typed hash map format
// len(child.type), child.type, len(child.label), child.label
// this is a helper function
// generic version format_prelabel::<String> or format_prelabel::<[0; 32]>
pub fn format_prelabel(data: NodeTreeTypes) -> String {
    let mut buf: [u8; 32] = [0; 32];
    match data {
        NodeTreeTypes::Bytes(expr) => buf = expr,
        NodeTreeTypes::String(data) => {
            hash::<Sha256>(&data, &mut buf);
        }
    }
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
        let _my_data1 = format_prelabel(NodeTreeTypes::String(_left.clone()));
        let _my_data2 = format_prelabel(NodeTreeTypes::String(_right.clone()));
        let prelabel = format!("{}{}", _my_data1, _my_data2);
        nodes.push(prelabel.clone())
    }
    if &leaves.len() % 2 != 0 {
        let _right = leaves.last().unwrap();
        let _my_data2 = format_prelabel(NodeTreeTypes::String(_right.clone()));
        let prelabel = format!("{}{}", _my_data2, _my_data2);
        nodes.push(prelabel);
    }
    nodes
}

//  _               _              _
// | |             | |            | |
// | |__   __ _ ___| |__   ___  __| |
// | '_ \ / _` / __| '_ \ / _ \/ _` |
// | | | | (_| \__ \ | | |  __/ (_| |
// |_| |_|\__,_|___/_| |_|\___|\__,_|

// here were build the first level up from the leaves
pub fn get_parents_for_leaf_nodes(data: &Vec<(&str, &str)>) -> Vec<[u8; 32]> {
    let mut nodes = Vec::new();
    for i in (0..&data.len() - 1).step_by(2) {
        let value1 = &data[i];
        let value2 = &data[i + 1];
        let mut buf: [u8; 32] = [0; 32];

        let left_prelabel = format!(
            "{}{}{}{}",
            value1.0.len(),
            value1.0,
            value1.1.len(),
            value1.1
        );
        let right_prelabel = format!(
            "{}{}{}{}",
            value2.0.len(),
            value2.0,
            value2.1.len(),
            value2.1
        );
        let prelabel = format!("{}{}", left_prelabel, right_prelabel);
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
        let _my_data1 = format_prelabel(NodeTreeTypes::Bytes(*_left));
        let _my_data2 = format_prelabel(NodeTreeTypes::Bytes(*_right));
        let prelabel = format!("{}{}", _my_data1, _my_data2);
        hash::<Sha256>(&prelabel, &mut buf);
        nodes.push(buf.clone())
    }
    if &leaves.len() % 2 != 0 {
        nodes.push(*leaves.last().unwrap());
    }
    nodes
}

//  _
// | |
// | |_ _ __ ___  ___
// | __| '__/ _ \/ _ \
// | |_| | |  __/  __/
//  \__|_|  \___|\___|

pub fn index_tree<T: std::clone::Clone>(data: Vec<Vec<T>>) -> Vec<(usize, T)> {
    let depth = data.len();
    let mut leaves: Vec<(usize, T)> = Vec::new();

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

// this functions makes the tree
pub fn make_tree(data: &Vec<(&str, &str)>) -> Vec<Vec<[u8; 32]>> {
    let mut levels = Vec::new();
    let mut leaves = Vec::new();

    // make typed k/v's into hashes
    // we H(type,value)
    // these hashes don't actually impact the trees root
    // thier parents - which use data from the leaf do
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

pub fn make_prelabel_tree(data: &Vec<(&str, &str)>) -> Vec<Vec<String>> {
    let mut prelabel_levels = Vec::new();
    let mut prelabel_leaves = Vec::new();

    // make typed k/v's into hashes
    // we H(type,value)
    for _data in data {
        let _my_data = format!("{{ key = \"{}\", value = \"{}\" }}", _data.0, _data.1);
        prelabel_leaves.push(_my_data)
    }
    prelabel_levels.push(prelabel_leaves.clone());

    // prelabel_leaves = get_prelabels_for_leaf_nodes(&data);

    prelabel_levels.push(prelabel_leaves.clone());
    while &prelabel_leaves.len() > &1 {
        prelabel_leaves = get_prelabels_for_internal_nodes(prelabel_leaves);
        prelabel_levels.push(prelabel_leaves.clone());
    }
    prelabel_levels.reverse();
    prelabel_levels
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

// // here were build the first level up from the leaves
// //
// pub fn level_above_leaves(data: &Vec<(&str, &str)>, for_prelabels: bool) -> Vec<NodeTreeTypes> {
//     let mut nodes = Vec::new();
//     for i in (0..&data.len() - 1).step_by(2) {
//         let value1 = &data[i];
//         let value2 = &data[i + 1];
//         let mut buf: [u8; 32] = [0; 32];

//         let _my_data1 = format!(
//             "{}{}{}{}",
//             value1.0.len(),
//             value1.0,
//             value1.1.len(),
//             value1.1
//         );
//         let _my_data2 = format!(
//             "{}{}{}{}",
//             value2.0.len(),
//             value2.0,
//             value2.1.len(),
//             value2.1
//         );
//         let prelabel = format!("{}{}", _my_data1, _my_data2);
//         hash::<Sha256>(&prelabel, &mut buf);

//         if for_prelabels {
//             // string array
//             nodes.push(NodeTreeTypes::String(prelabel.clone()))
//         } else {
//             // // byte array
//             nodes.push(NodeTreeTypes::Bytes(buf.clone()))
//         }
//     }
//     nodes
// }
