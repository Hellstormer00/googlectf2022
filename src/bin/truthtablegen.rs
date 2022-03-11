use itertools::Itertools;

fn main() {
    make_truthtable(|a0, b0| vec![a0 && b0])
}

fn make_truthtable(func: fn(bool, bool) -> Vec<bool>) {
    let combinations = vec![true, false]
        .into_iter()
        .cartesian_product(vec![true, false]);
    for (a, b) in combinations {
        println!("{} . {} -> {:?}", a, b, func(a, b));
    }
}

// fn S51(O: bool, N: bool) -> bool {}
