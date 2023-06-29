use rand::Rng;


#[derive(Copy, Clone)]
struct Point{
    x: usize,
    y: usize,
}

impl Point{
    fn new(x : usize, y: usize) -> Point{
        Point{
            x,
            y,
        }
    }
    fn random(x_bound: usize, y_bound: usize) -> Point{
        Point{
            x: rand::thread_rng().gen_range(0..x_bound),
            y: rand::thread_rng().gen_range(0..y_bound),
        }
    }

    fn get_index(self, x_bound: usize) -> usize{
        //println!("x_bound:{x_bound}, y:{}, x:{}", self.y, self.x);
        return x_bound * self.y + self.x;

    }
}


#[derive(Clone)]
enum Obsticle{
    PLAIN,
    ROCK,
}

impl Obsticle {
    fn as_char(&self) -> char {
        match self {
            Obsticle::ROCK => 'O',
            Obsticle::PLAIN => '='
        }

    }

}

#[derive(Clone)]
struct Cell{
    terr: Obsticle,
}

impl Cell {
    fn new() -> Cell {
        return Cell{terr:Obsticle::PLAIN};
    }
    fn as_char(&self) -> char {
        self.terr.as_char()
    }
}

#[derive(Clone)]
pub struct Map{
    width: usize,
    height: usize,
    map: Vec<Cell>,
    target: Point,
    player: Point,
}

impl Map{
    fn new(width: usize, height: usize, target: Point, player: Point) -> Map{
        let mut mapvec = Vec::<Cell>::new();

        for _ in 0..(width*height){
            mapvec.push(Cell::new());
        }
        Map{
            width,
            height,
            map: mapvec,
            target,
            player,
        }
    }
    fn insert_obsticle(&mut self, point: Point, target: Obsticle){
        self.map[point.get_index(self.width)].terr = target;
    }

    pub fn display_board(&self) -> String{

        let mut board = String::new();
        let target_pos = self.target.get_index(self.width);
        let player_pos = self.player.get_index(self.width);

        for (i,cell) in self.map.iter().enumerate(){
            if i % self.width == 0{
                board.push('\n')
            }

            match i {
                _ if target_pos == i => board.push('X'),
                _ if player_pos == i => board.push('C'),
                _ => board.push(cell.as_char()),
            }

        }

        return board;


    }

    fn desired_target_movement(&self, movement_command: char) -> Point {
        match movement_command{
            'w' => Point::new(self.player.x, self.player.y+1),
            'a' => Point::new(self.player.x-1, self.player.y),
            's' => Point::new(self.player.x, self.player.y-1),
            'd' => Point::new(self.player.x+1, self.player.y),
            _   => Point::new(self.player.x, self.player.y),
        }
    }

    fn check_player_movement_legality(&self, movement_target: Point) -> bool{
        if matches!(self.map[movement_target.get_index(self.width)].terr,Obsticle::PLAIN) {
            return false;
        }
        else if movement_target.x > self.width{
            return false;
        }
        else if movement_target.y > self.height{
            return false;
        }
        else if movement_target.x < 0 {
            return false;
        }
        else if movement_target.y < 0 {
            return false;
        }
        else{
            return true;
        }

    }

    pub fn move_player(&mut self, movement_command: char){

        let target_movement = self.desired_target_movement(movement_command);
        if !self.check_player_movement_legality(target_movement){
            return;
        }

        self.player = target_movement;

    }
}


pub fn gen_map(width: usize, height: usize) -> Map{

    let target = Point::random(width, height);
    let player = Point::random(width, height);


    let mut map = Map::new(width, height, target, player);

    for _ in 0..(map.width*map.height/20){
        let obs = Point::random(map.width, map.height);
        map.insert_obsticle(obs, Obsticle::ROCK);
    }

    return map;
}


