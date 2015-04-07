pub struct Board{
	root : Vec<usize>,
	sz : Vec<usize>,
}

impl Board{

	pub fn new(size:usize) -> Board{
		let mut root = vec![0;size];
		for i in 0..size{
			root[i] = i;
		}
		Board {root : root, sz : vec![1;size]}
	}

	pub fn from(root:Vec<usize>) -> Board{
		let len = root.len();
		Board {root : root, sz : vec![1;len]}
	}

	pub fn root(&mut self, i:usize) -> usize {
		let mut root_index = i;
		let nb_root_search = 0;
		while self.root[root_index]!=root_index && nb_root_search<=self.root.len(){
			self.root[root_index] = self.root[self.root[root_index]];//simple path compression
			root_index = self.root[root_index];
		}
		root_index
	}

	pub fn connected(&mut self,p:usize,q:usize)->bool{
		self.root(p)==self.root(q)
	}

	pub fn union(&mut self,p:usize,q:usize){
		let root_p = self.root(p);
		let root_q = self.root(q);
		if root_p != root_q {
			if self.sz[root_p] < self.sz[root_q] {
				self.root[root_p] = root_q;
				self.sz[root_q] += self.sz[root_p];
			}else{
				self.root[root_q] = root_p;
				self.sz[root_p] += self.sz[root_q];
			}
		}
	}
}