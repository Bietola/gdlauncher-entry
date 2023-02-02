use std::fs;
use std::collections::{hash_map::Entry, HashMap};

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
    let mut win = inp.next_chunk::<{WIN_LEN}>().unwrap();
    let mut win_st = 0;
    // TODO: ELG: Try using folds.
    let mut map: HashMap<i128, u8> = HashMap::new();
    win.iter().for_each(|&n| {
        map.entry(n).and_modify(|e| *e += 1).or_insert(1);
    });

    inp.filter(|&sum| {
        (win_st..win_st+WIN_LEN)
            .map(|i| win[i % WIN_LEN])
            // .inspect(|&lh| println!("lh: {:?}", lh))
            .find(|lh|
                map.get(&(sum - lh))
                    // .filter(|set| !set.contains(idx) || set.len() != 1)
                    .is_some()
            )
            .map(|_| {
                // let prev = win.pop_front().unwrap();
                let prev = win[win_st];

                if let Entry::Occupied(mut o) = map.entry(prev) {
                    let v = o.get_mut();
                    *v -= 1;
                    if *v == 0 { o.remove_entry(); }
                } else {
                    panic!();
                }

                // win.push_back(sum);
                win[win_st] = sum;
                win_st = (win_st + 1) % WIN_LEN;

                map.entry(sum).and_modify(|e| *e += 1).or_insert(1);
            })
            .is_none()
    }).collect()
}
