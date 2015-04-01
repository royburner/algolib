pub fn connected(a:[i32; 3],p:usize,q:usize)->bool{
	a[p]==a[q]
}

pub fn union(a: &mut [i32; 3],p:usize,q:usize){
	let p_val = a[p];
	let q_val = a[q];
	for i in 0..3{
		if a[i]==p_val {
			a[i]=q_val;
		}
	}
}

