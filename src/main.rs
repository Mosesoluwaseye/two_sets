use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let n: u64 = input.trim().parse().unwrap();

    let total = n * (n + 1) / 2;

    if total % 2 != 0 {
        println!("NO");
        return;
    }

    println!("YES");

    let target = total / 2;

    let mut set1 = Vec::new();
    let mut set2 = Vec::new();

    let mut remaining = target;

    for i in (1..=n).rev() {
        if i <= remaining {
            set1.push(i);
            remaining -= i;
        } else {
            set2.push(i);
        }
    }

    println!("{}", set1.len());
    println!(
        "{}",
        set1.iter()
            .rev()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );

    println!("{}", set2.len());
    println!(
        "{}",
        set2.iter()
            .rev()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );
}