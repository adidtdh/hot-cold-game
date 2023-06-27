use rand::Rng;

struct Point{
    x: usize,
    y: usize,
}

impl Point{
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
    fn as_str(&self) -> &str {
        match self {
            Obsticle::ROCK => "O",
            Obsticle::PLAIN => " "
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
    fn as_str(&self) -> &str {
        self.terr.as_str()
    }
}

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

    pub fn display_board(self){

        let target_pos = self.target.get_index(self.width);
        let player_pos = self.player.get_index(self.width);

        for (i,cell) in self.map.iter().enumerate(){
            if i % self.width == 0{
                println!();
            }

            match i {
                _ if target_pos == i => print!("X"),
                _ if player_pos == i => print!("C"),
                _ => print!("{}", cell.as_str()),
            }

        }


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


