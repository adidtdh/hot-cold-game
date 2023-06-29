mod mapgen;
use cursive::{
    direction::Direction,
    event::{Event, EventResult, MouseButton, MouseEvent},
    theme::{BaseColor, Color, ColorStyle},
    view::CannotFocus,
    views::{Button, Dialog, LinearLayout, Panel, SelectView, Canvas},
    Cursive, Printer, Vec2, XY
};


use cursive::views::TextView;
use cursive::{CursiveExt};

use mapgen::Map;

struct BoardView{
    board : Map,
}

impl BoardView {
    pub fn new(board: Map) -> BoardView{
        BoardView{
            board,
        }
    }
}


impl cursive::view::View for BoardView{
    fn draw(&self, printer: &Printer) {
        let text = self.board.display_board();

        for (i,c) in text.chars().enumerate(){
            let x = (i % 40) * 1;
            let y = i / 40;
            printer.print(XY::new(x, y), &c.to_string());


        }
    }

    fn take_focus(&mut self, _: Direction) -> Result<EventResult, CannotFocus> {
        Ok(EventResult::Consumed(None))
    }

    fn required_size(&mut self, constraint: Vec2) -> Vec2 {
        XY{
            x: 40,
            y: 10,
        }
    }

}

fn main() {
    let map = mapgen::gen_map(40, 10);
    let board = BoardView::new(map);
    let mut siv = Cursive::new();

    let canvas = Dialog::around(board);

    siv.add_layer(canvas);

    siv.add_global_callback('q', |s| s.quit());

    siv.run();



}



