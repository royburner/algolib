extern crate algolib;

use algolib::*;

#[test]
fn test_selection_sort_from(){
	let a = selection_sort::Board::from(vec![0,1,2,3]);
	assert!(a.val(0)==0);
}

#[test]
fn test_selection_sort_sort(){
	let mut a = selection_sort::Board::from(vec![0,3,2,1]);
	a.sort();

	assert_eq!(a.val(0),0);
	assert_eq!(a.val(1),1);
	assert_eq!(a.val(2),2);
	assert_eq!(a.val(3),3);
}