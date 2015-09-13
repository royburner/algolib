extern crate algolib;

use algolib::*;
use algolib::vec_selection_sort::SelSort;

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

#[test]
fn test_vec_selection_sort_sort(){
	let mut a = vec![0,3,2,1];
	a.sel_sort();

	assert_eq!(a[0],0);
	assert_eq!(a[1],1);
	assert_eq!(a[2],2);
	assert_eq!(a[3],3);
}

#[test]
fn test_shell_sort_sort(){
	let mut a = shell_sort::Board::from(vec![0,3,2,1]);
	a.sort();

	assert_eq!(a.val(0),0);
	assert_eq!(a.val(1),1);
	assert_eq!(a.val(2),2);
	assert_eq!(a.val(3),3);
}

