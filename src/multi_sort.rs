use std::vec::Vec;

/// 
/// Multiple sort hability
///
pub trait MultiSort{
	/// selection sort algorithm
	fn sel_sort(&mut self);
	
	/// shell sort algorithm
	fn shell_sort(&mut self);
}

impl<T:PartialOrd> MultiSort for Vec<T>{
	fn sel_sort(&mut self){
		for i in 0..self.len()-1{
			let mut min_index = i;
			for j in i..self.len(){
				if self[j] < self[min_index]{
					min_index = j;
				}
			}
			self.swap(i, min_index);
		}
	}
	
	fn shell_sort(&mut self){
		//determines 3x+1 intervall
		let mut x = 0;
		while 3*x + 1 < self.len(){
			x+=1;
		}
		
		//chaining h sort on decreasing h intervall
		for i in (0..x).rev() {
			let h = 3 * i + 1;
			
			for j in h..self.len(){
				let mut k = j;
				while k > 0 && self[k] < self[k-h]{
					self.swap(k, k-h);
					k -= h;
				}
			}
		}
	}
}


