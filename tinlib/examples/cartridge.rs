use std::io::Cursor;

use tinlib::cartridge::Cartridge;

fn main() {
    let cart = Cartridge {
        // An incomplete game cart with empty fonts, map and cover.
        version: 17,
        name: "Dungeons of the Dungeon".to_string(),
        desc: "A cool game about dungeons inside dungeons.".to_string(),
        author: "Luiz de Prá".to_string(),
        palette: vec![
            0x2d, 0x1b, 0x000, // dark
            0x1e, 0x60, 0x6e, // dark greenish
            0x5a, 0xb9, 0xa8, // greenish
            0xc4, 0xf0, 0xc2, // light greenish
        ],
        code: "def main:\n    pass".to_string(),
        ..Default::default()
    };

    println!("Pre-save Cart: {:?}\n\n", &cart);

    // Saving the cart data into a cursor (file or anything that implements Write).
    let mut cursor = Cursor::new(vec![]);
    cart.save(&mut cursor).expect("failed to save cart");

    println!("File data: {:?}\n\n", &cursor);

    // Loading the cart data from a cursor (file, or anything that implements Read).
    cursor.set_position(0);
    let new_cart = Cartridge::from_reader(&mut cursor).expect("failed to load cart");

    println!("Post-load Cart: {:?}\n\n", &new_cart);

    println!("They has the same data? {}\n\n", cart == new_cart);
}
