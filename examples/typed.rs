use typed_hash_tree::print::{pretty_print_tree, pretty_print_tree_bytes};
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
    let prelabel_levels = make_prelabel_tree(&data);

    println!("\nPrelabel Tree");

    let tree_in_print_order = index_tree::<String>(prelabel_levels.clone());
    pretty_print_tree::<String>(tree_in_print_order, None);

    println!("\n\nHash Tree");

    let tree_in_print_order = index_tree::<[u8; 32]>(levels);
    pretty_print_tree_bytes::<[u8; 32]>(tree_in_print_order, Some(base64::encode));
}
