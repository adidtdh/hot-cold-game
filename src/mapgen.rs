use rand::Rng;

struct Point{
    x: usize,
    y: usize,
}

impl Point{
    fn random(x_bound: usize, y_bound: usize) -> Point{
        Point{
            x: rand::thread_rng().gen_range(0..=x_bound),
            y: rand::thread_rng().gen_range(0..=y_bound),
        }
    }

    fn get_index(self, x_bound: usize) -> usize{
        return x_bound * self.y + self.x;

    }
}

struct Map{
    width: usize,
    height: usize,
    map: Vec<bool>,
    target: Point,
    player: Point,
}

impl Map{
    fn insert_obsticle(&mut self, point: Point, target: bool){
        self.map[point.get_index(self.width)] = target;
    }
}


fn gen_map(width: usize, height: usize) -> Map{

    let target = Point::random(width, height);
    let player = Point::random(width, height);


    let mut map = Map{
        width,
        height,
        map: vec![false; width*height],
        target,
        player,
    };

    for i in 0..(map.width*map.height/20){
        let obs = Point::random(map.width, map.height);
        map.insert_obsticle(obs, true);
    }

    return map;
}


