use std::fs;
use std::collections::{HashSet, HashMap, VecDeque};

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
            .into_iter()
            .enumerate()
    );
    // TODO: ELG: Try using iterators.
    let mut map: HashMap<i128, HashSet<usize>> = HashMap::new();
    win.iter().for_each(|(i, n)| {
        map.entry(*n).or_default().insert(*i);
    });

    inp.filter(|&sum| {
        win.iter()
            .copied()
            // .inspect(|&lh| println!("lh: {:?}", lh))
            .find(|(_idx, lh)|
                map.get(&(sum - lh))
                    // .filter(|set| !set.contains(idx) || set.len() != 1)
                    .is_some()
            )
            .map(|(idx, _)| {
                let (_, prev) = win.pop_front().unwrap();
                map.get_mut(&prev).unwrap().remove(&idx);

                let new_idx = (win[win.len() - 1].0 + 1) % WIN_LEN;
                win.push_back((new_idx, sum));

                map.entry(sum).or_default().insert(new_idx);
            })
            .is_none()
    }).collect()
}
