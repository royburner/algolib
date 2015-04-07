extern crate algolib;

use algolib::*;

#[test]
fn test_quick_find_connected() {
	let a = quick_find::Board::from(vec![1, 2, 1]);
	assert!(a.connected(0,2));
}

#[test]
fn test_quick_find_connected_bigger() {
	let a = quick_find::Board::from(vec![1, 2, 1, 5, 6, 4, 8, 3, 1, 2]);
	assert!(a.connected(0,2));
}


#[test]
fn test_quick_find_not_connected() {
	let a = quick_find::Board::from(vec![1, 2, 1]);
	assert!(!a.connected(0,1));
}

#[test]
fn test_quick_find_union(){
	let mut a = quick_find::Board::from(vec![0, 1, 2]);
	assert!(!a.connected(0,1));
	a.union(0,1);
	assert!(a.connected(0,1));
	a.union(0,2);
	assert!(a.connected(0,1));
	assert!(a.connected(1,2));
}

#[test]
fn test_quick_union_root(){
	let mut a = quick_union::Board::from(vec![0, 1, 0]);
	assert!(a.root(0)==0);
	assert!(a.root(2)==0);
}

#[test]
fn test_quick_union_connected(){
	let mut a = quick_union::Board::from(vec![0, 1, 0]);
	assert!(a.connected(0, 0));
	assert!(a.connected(0, 2));
	assert!(!a.connected(0, 1));
}

#[test]
fn test_quick_union_union(){
	let mut a = quick_union::Board::from(vec![0, 1, 2]);
	assert!(!a.connected(0,1));
	a.union(0,1);
	assert!(a.connected(0,1));
	a.union(0,2);
	assert!(a.connected(0,1));
	assert!(a.connected(1,2));
}

#[test]
fn test_quick_union_new(){
	let mut a = quick_union::Board::new(3);
	assert!(!a.connected(0,1));
	a.union(0,1);
	assert!(a.connected(0,1));
	a.union(0,2);
	assert!(a.connected(0,1));
	assert!(a.connected(1,2));
}