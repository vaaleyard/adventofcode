use std::{io::{self, BufRead}, collections::HashMap};

pub fn solve() {
    let mut lines = io::stdin().lock().lines();

    let mut deers: HashMap<usize, usize> = HashMap::new();
    let mut deer_id: usize = 1;
    let mut sum: usize = 0;

    while let Some(line) = lines.next() {
        let actual_line = line.unwrap();

        if actual_line.len() > 0 {
            sum += actual_line.parse::<usize>().unwrap();
        } else if actual_line.len() == 0 {
            sum = 0;
            deer_id += 1;
            continue;
        }

        deers.insert(deer_id, sum);
    }

    /* part one */
    let mut calories: Vec<usize> = deers.into_values().collect::<Vec<usize>>().to_owned();
    calories.sort_by(|a, b| b.cmp(a));
    assert_eq!(calories.first(), Some(&64929));

    /* part two */
    assert_eq!(calories.iter().take(3).sum::<usize>(), 193697);
}
