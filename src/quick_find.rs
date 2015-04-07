pub struct Board{
	root : Vec<usize>,
}

impl Board{
	
	pub fn new(size:usize) -> Board{
		let mut root = vec![0;size];
		for i in 0..size{
			root[i] = i;
		}
		Board {root : root}
	}
	
	pub fn from(root:Vec<usize>) -> Board{
		Board {root : root}
	}
	
	pub fn connected(&self, p:usize,q:usize)->bool{
		self.root[p]==self.root[q]
	}
	
	pub fn union(&mut self, p:usize,q:usize){
		let p_val = self.root[p];
		let q_val = self.root[q];
		for i in 0..self.root.len(){
			if self.root[i]==p_val {
				self.root[i]=q_val;
			}
		}
	}
	
}