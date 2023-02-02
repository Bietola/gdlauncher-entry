use std::fs;
use std::collections::{VecDeque};
use counter::Counter;

// cat assets/challenge_input.txt | nl | grep -E '^\s+\b118\b' -B 100 | grep -E '\b1\b'

pub fn solve() -> Vec<i128> {
    const INPUT_FILE: &str = &"assets/challenge_input.txt";
    const WIN_LEN: usize   = 100;

    let inp = fs::read_to_string(&INPUT_FILE).unwrap();
    let inp = &inp[..inp.len()-1];

    let mut inp = inp
        .split("\n")
        // .inspect(|line| println!("{}", line))
        .map(|line| line.parse::<i128>().unwrap());

    // TODO: OPT: Try reusable iter.
    // TODO: OPT: Use cyclical array of WIN_LEN.
    let mut win = VecDeque::from_iter(
        inp.next_chunk::<{WIN_LEN}>().unwrap()
    );
    // TODO: ELG: Try using folds.
    let mut map = win.iter().copied().collect::<Counter<_>>();

    inp.filter(|&sum| {
        win.iter()
            .copied()
            // .inspect(|&lh| println!("lh: {:?}", lh))
            .find(|lh|
                map.get(&(sum - lh))
                    // .filter(|set| !set.contains(idx) || set.len() != 1)
                    .is_some()
            )
            .map(|_| {
                let prev = win.pop_front().unwrap();

                map.entry(prev).and_modify(|e| *e = if *e == 0 { 0 } else { *e - 1 });

                win.push_back(sum);

                map.entry(sum).and_modify(|e| *e += 1).or_insert(1);
            })
            .is_none()
    }).collect()
}
