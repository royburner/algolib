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
	
		//determines 3x+1 intervall
		let mut x = 0;
		while 3*x + 1 < self.val.len(){
			x+=1;
		}
		
		//chaining h sort on decreasing h intervall
		for i in (0..x).rev() {
			let h = 3 * i + 1;
			
			for j in h..self.val.len(){
				let mut k = j;
				while k > 0 && self.val[k] < self.val[k-h]{				
					let valk = self.val[k];
					self.val[k] = self.val[k-h];
					self.val[k-h] = valk;
					k -= h;
				}
			}
		}
	}
}
