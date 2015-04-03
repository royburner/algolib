pub fn connected(a:&Vec<usize>,p:usize,q:usize)->bool{
	a[p]==a[q]
}

pub fn union(mut a:Vec<usize>,p:usize,q:usize)->Vec<usize>{
	let p_val = a[p];
	let q_val = a[q];
	for i in 0..a.len(){
		if a[i]==p_val {
			a[i]=q_val;
		}
	}
	a
}