
use crate::direction::Direction;
use crate::point::Point;

#[derive(Debug)]
pub struct Snake {
    body: Vec<Point>,
    direction: Direction,
    digestion: bool,
}

impl Snake{
    pub fn new(start: Point, length: u16, direction: Direction) -> Self {
        let opposite = direction.opposite();
        let body: Vec<Point> = (0..length)
            .into_iter()
            .map(|i| start.transform(opposite, i))
            .collect();
        Self{body, direction, digestion: false}
    }

    pub fn get_head_point(&self) -> Point {
        self.body.first().unwrap().clone() //clone get the actual value, not the immutable reference
    }

    pub fn get_direction(&self) -> Direction {
        self.direction.clone()
    }

    pub fn contains_points(&self, point: &Point) -> bool {
        self.body.contains(point)
    }

    pub fn slither(&mut self) { //make snake move across the grid
        self.body.insert(0, self.body.first().unwrap().transform(self.direction, 1));
        if !self.digestion{
            self.body.remove(self.body.len()-1);
        }
        else {
            self.digestion = false;
        }
    }

    pub fn set_direction(&mut self, direction: Direction){
        self.direction = direction;
    }

    pub fn grow(&mut self) {
        self.digestion = true;
    }

    pub fn get_body_points(&self) -> Vec<Point> {
        self.body.clone()
    }
}