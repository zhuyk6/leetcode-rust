
pub fn restore_matrix(mut row_sum: Vec<i32>, mut col_sum: Vec<i32>) -> Vec<Vec<i32>> {
	let m = row_sum.len();
	let n = col_sum.len();
	let mut mat: Vec<Vec<i32>> = vec![vec![0; n]; m];

	for i in 0..m {
		for j in 0..n {
			let v = row_sum[i].min(col_sum[j]);
			mat[i][j] = v;
			row_sum[i] -= v;
			col_sum[j] -= v;
			if row_sum[i] == 0 {
				break;
			}
		}
	}
	mat
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn sample() {
		let rowsum = vec![3, 8];
		let colsum = vec![4, 7];
		assert_eq!(restore_matrix(rowsum, colsum), vec![vec![3, 0], vec![1, 7]]);
	}
}