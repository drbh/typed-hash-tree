use typed_hash_tree::*;

fn main() {
    let data = vec![
        //
        // pair 1
        ("name", "David"),
        (
            "",
            "33010a2ffb4eb049d94d88187ef91ff2b86e3a08eabb7e24a3593a6dd0d999f1",
        ),
        //
        // pair 2
        ("age", "26"),
        (
            "",
            "3a9992a090e6784b88defd9b23fe8a5f9d031dbadb92e7a1f0803f114b43fe46",
        ),
        //
        // pair 3
        ("country", "US"),
        (
            "",
            "bdaa1893e0a738906e439251fda1ef9f9bf9a30f71468f02d38b24dee4098bdb",
        ),
        //
        // pair 4
        ("lang", "english"),
        (
            "",
            "b4693fa7e3ebd0ef108bd998034d316feea04f2fa23d50ab949b9d188a18f972",
        ),
    ];

    let levels = make_tree(&data);
    //

    let mut prelabel_levels = Vec::new();
    let mut prelabel_leaves = Vec::new();

    // make typed k/v's into hashes
    // we H(type,value)
    for _data in &data {
        let _my_data = format!("{{ key = \"{}\", value = \"{}\" }}", _data.0, _data.1);
        prelabel_leaves.push(_my_data)
    }
    prelabel_levels.push(prelabel_leaves.clone());

    prelabel_leaves = get_prelabels_for_leaf_nodes(&data);
    prelabel_levels.push(prelabel_leaves.clone());
    while &prelabel_leaves.len() > &1 {
        prelabel_leaves = get_prelabels_for_internal_nodes(prelabel_leaves);
        prelabel_levels.push(prelabel_leaves.clone());
    }
    prelabel_levels.reverse();
    // println!("{:#?}", prelabel_levels);
    //
    //

    // for lvl in &levels {
    //     println!("");
    //     for lf in lvl {
    //         println!("{:?}", encode(lf));
    //     }
    // }

    println!("\nPrelabel Tree");

    let tree_in_print_order = index_tree_prelabels(prelabel_levels.clone());
    pretty_print_tree::<String>(tree_in_print_order, None);

    // let tree_in_print_order = index_tree_prelabels(prelabel_levels.clone());
    // pretty_print_tree::<String>(tree_in_print_order, Some(base64::encode));

    println!("");
    println!("\nHash Tree");

    let tree_in_print_order = index_tree(levels);
    pretty_print_tree_bytes::<[u8; 32]>(tree_in_print_order, Some(base64::encode));
}
