extern crate algolib;

use algolib::*;

#[test]
fn test_selection_sort_from(){
	let a = selection_sort::Board::from(vec![0,1,2,3]);
	assert!(a.val(0)==0);
}