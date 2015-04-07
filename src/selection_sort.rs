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
	
	pub fn sort(&self){
		for i in 0..self.val.len(){
			
		}
	}

}