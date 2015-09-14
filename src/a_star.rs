use std::collections::{HashSet, HashMap};

#[derive(Hash, Eq, PartialEq, Clone, Copy)]
struct Point{
	x : usize,
	y : usize
}

struct Board{
	width : usize,
	height : usize,
	start : Point,
	goal : Point
	
}

impl Board{
	
	fn new(width : usize)->Board{
		Board{width : width, height : width, start:Point{x:0, y:0}, goal:Point{x:width, y:width}}
	}
	
	fn newGScoreWithInfinity(&self) -> HashMap<Point, usize>{
		let mut gScore : HashMap<Point, usize> = HashMap::new();
		for i in 0..self.width{
			for j in 0..self.height{
				gScore.insert(Point{x:i, y:j}, usize::max_value());
			}
		}
		gScore
	}
	
	fn heuristic_cost_estimate(&self, point:Point)->usize{
		let absX = match(point.x>self.goal.x){//TODO refactor
			true => point.x-self.goal.x,
			false => self.goal.x-point.x
		};
		let absY = match(point.y>self.goal.y){
			true => point.y-self.goal.y,
			false => self.goal.y-point.y
		};
		absX + absY
	}
	
	fn a_star(&self) -> Option<Vec<Point>>{
		let mut closedSet : HashSet<Point> = HashSet::new();// The set of nodes already evaluated.
		let mut openSet : HashSet<Point> = HashSet::new();// The set of tentative nodes to be evaluated
		openSet.insert(self.start);//initially containing the start node
		let mut cameFrom : HashMap<Point, Point> = HashMap::new();  // The map of navigated nodes.
		
		let mut gScore = self.newGScoreWithInfinity();
		gScore.insert(self.start, 0);// Cost from start along best known path.
		
		// Estimated total cost from start to goal through y.
		let mut fScore = self.newGScoreWithInfinity();
		fScore.insert(self.start, self.heuristic_cost_estimate(self.start));
		
		while !openSet.is_empty() {
			//TODO to be continued
			//https://en.wikipedia.org/wiki/A*_search_algorithm
		}
		
		
		
		None
	}
	
	
}