extern crate algolib;

#[test]
fn test_connected() {
	let a = vec![1, 2, 1];
	assert!(algolib::connected(&a,0,2));
}

#[test]
fn test_connected_bigger() {
	let a = vec![1, 2, 1, 5, 6, 4, 8, 3, 1, 2];
	assert!(algolib::connected(&a,0,2));
}


#[test]
fn test_not_connected() {
	let a = vec![1, 2, 1];
	assert!(!algolib::connected(&a,0,1));
}

#[test]
fn test_union(){
	let a = vec![0, 1, 2];
	assert!(!algolib::connected(&a,0,1));
	let b = algolib::union(a,0,1);
	assert!(algolib::connected(&b,0,1));
	let c = algolib::union(b,0,2);
	assert!(algolib::connected(&c,0,1));
	assert!(algolib::connected(&c,1,2));
}