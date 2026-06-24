use two_sets::{two_sets_greedy, two_sets_naive};

#[test]
fn test_n_7() {
    assert_eq!(two_sets_greedy(7), true);
    assert_eq!(two_sets_naive(7), true);
}

#[test]
fn test_n_6() {
    assert_eq!(two_sets_greedy(6), false);
    assert_eq!(two_sets_naive(6), false);
}

#[test]
fn test_n_3() {
    assert_eq!(two_sets_greedy(3), true);
    assert_eq!(two_sets_naive(3), true);
}

#[test]
fn test_n_4() {
    assert_eq!(two_sets_greedy(4), true);
    assert_eq!(two_sets_naive(4), true);
}