mod mapgen;
use cursive::{Cursive, CursiveExt};
use cursive::views::*;
use cursive::event::{Event, EventResult, Key};
use unicode_width::UnicodeWidthStr; // To get the width of some text.

fn main() {
    let map = mapgen::gen_map(40, 10);

    let mut siv = Cursive::new();
    let source = map.display_board();

    let dialog = Dialog::around(TextView::new(source))
        .button("HOW DO I PLAY?????", |s| s.quit())
        .button("Quit", |s| s.quit());


    siv.add_layer(dialog);




    siv.add_global_callback('q', |s| s.quit());

    siv.run();


}
