extern crate algolib;

use algolib::*;
use algolib::multi_sort::MultiSort;

#[test]
fn test_multi_sort_sel_sort(){
	let mut a = vec![0,3,2,1];
	a.sel_sort();

	assert_eq!(a[0],0);
	assert_eq!(a[1],1);
	assert_eq!(a[2],2);
	assert_eq!(a[3],3);
}

#[test]
fn test_multi_sort_shell_sort(){
	let mut a = vec![0,3,2,1];
	a.shell_sort();

	assert_eq!(a[0],0);
	assert_eq!(a[1],1);
	assert_eq!(a[2],2);
	assert_eq!(a[3],3);
}


