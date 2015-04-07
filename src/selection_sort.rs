pub struct Board{
	val : Vec<usize>,
}

impl Board{

	pub fn from(val : Vec<usize>)->Board{
		Board{val : val}
	}
	
	pub fn val(&self, i : usize)->usize{
		self.val[i]
	}
	
	pub fn sort(&mut self){
		for i in 0..self.val.len()-1{
			let mut min_index = i;
			for j in i..self.val.len(){
				if self.val[j] < self.val[min_index]{
					min_index = j;
				}
			}
			let ival = self.val[i];
			self.val[i] = self.val[min_index];
			self.val[min_index] = ival;
		}
	}

}