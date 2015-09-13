use std::vec::Vec;

/// 
/// Selection sort trait.
///
/// Anything implementing this trait must implement Selection sort algorithm in the sel_sort function.
///
pub trait SelSort{
	fn sel_sort(&mut self);
}

impl<T:PartialOrd> SelSort for Vec<T>{
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
}

