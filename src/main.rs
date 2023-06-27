mod mapgen;

fn main() {
    let map = mapgen::gen_map(20, 10);

    map.display_board()

}
