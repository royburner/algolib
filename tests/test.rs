extern crate algolib;

#[test]
fn test_connected() {
	let a = [1, 2, 1];
	assert!(algolib::connected(a,0,2));
}


#[test]
fn test_not_connected() {
	let a = [1, 2, 1];
	assert!(!algolib::connected(a,0,1));
}

#[test]
fn test_union(){
	let mut a = [0, 1, 2];
	assert!(!algolib::connected(a,0,1));
	algolib::union(&mut a,0,1);
	assert!(algolib::connected(a,0,1));
	algolib::union(&mut a,0,2);
	assert!(algolib::connected(a,0,1));
	assert!(algolib::connected(a,1,2));
}