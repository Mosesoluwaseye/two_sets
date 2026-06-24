pub fn two_sets_greedy(n: u64) -> bool {
    let total_sum = n * (n + 1) / 2;
    total_sum % 2 == 0
}

pub fn two_sets_naive(n: u64) -> bool {
    let total_sum = n * (n + 1) / 2;

    if total_sum % 2 != 0 {
        return false;
    }

    let target = total_sum / 2;

    fn search(current: u64, target: i64) -> bool {
        if target == 0 {
            return true;
        }

        if current == 0 || target < 0 {
            return false;
        }

        search(current - 1, target - current as i64)
            || search(current - 1, target)
    }

    search(n, target as i64)
}