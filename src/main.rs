mod mapgen;

fn main() {
    let map = mapgen::gen_map(40, 10);

    map.display_board()

}
