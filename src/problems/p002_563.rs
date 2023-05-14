
pub fn count_fair_pairs(mut nums: Vec<i32>, lower: i32, upper: i32) -> i64 {
	let n = nums.len();
	nums.sort();
	let mut ans = 0;

	let mut l = 0usize;
	while l < n && nums[0] + nums[l] < lower {
		l += 1;
	}
	let mut r = l;
	while r < n && nums[0] + nums[r] <= upper {
		r += 1;
	}
	ans += (r - l) as i64;
	if nums[0] * 2 >= lower && nums[0] * 2 <= upper {
		ans -= 1;
	}
	println!("{} - {}", l, r);

	for v in &nums[1..] {
		while l > 0 && v + nums[l-1] >= lower {
			l -= 1;
		}
		while r > 0 && v + nums[r-1] > upper {
			r -= 1;
		}
		println!("{} - {}", l, r);
		ans += (r - l) as i64;
		if v * 2 >= lower && v * 2 <= upper {
			ans -= 1;
		}
	}

	ans / 2
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn sample() {
		let nums = vec![1,7,9,2,5];
		let l = 11;
		let r = 11;
		assert_eq!(count_fair_pairs(nums, l, r), 1);

		let nums = vec![0,1,7,4,4,5];
		let l = 3;
		let r = 6;
		assert_eq!(count_fair_pairs(nums, l, r), 6);
	}
}