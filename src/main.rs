mod game;

fn main() {
    println!("Hello, world!");

    let mut b = game::Board::new(8);
    b.block((3,4)).unwrap();
    b.play((3,5), 1).unwrap();
    b.get((3,5));
}
