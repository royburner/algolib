pub fn connected(a:&Vec<i32>,p:usize,q:usize)->bool{
	a[p]==a[q]
}

pub fn union(mut a:Vec<i32>,p:usize,q:usize)->Vec<i32>{
	let p_val = a[p];
	let q_val = a[q];
	for i in 0..a.len(){
		if a[i]==p_val {
			a[i]=q_val;
		}
	}
	a
}

