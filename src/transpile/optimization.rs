use std::sync::LazyLock;

use crate::CELL_SIZE;

static COMPACTION_MAP: LazyLock<Vec<(Vec<usize>, isize)>> = LazyLock::new(make_compaction_map);

pub fn compaction_map(value: usize) -> (Vec<usize>, isize) {
    if let Some(map_ref) = COMPACTION_MAP.get(value) {
        map_ref.clone()
    } else {
        (vec![value], 0)
    }
}

fn make_compaction_map() -> Vec<(Vec<usize>, isize)> {
    let mut cost = vec![0; CELL_SIZE];
    let mut split = vec![0; cost.len()];
    for i in 2..cost.len() {
        cost[i] = i;
        for d in 2..=i.isqrt() {
            if i % d == 0 {
                let new_cost = cost[d] + cost[i / d] + 7;
                if new_cost < cost[i] {
                    cost[i] = new_cost;
                    split[i] = d;
                }
            }
        }
    }
    fn get_factors(split: &Vec<usize>, value: usize) -> Vec<usize> {
        if split[value] == 0 {
            return vec![value];
        }
        let left = get_factors(split, split[value]);
        [left, get_factors(split, value / split[value])].concat()
    }
    let mut result = vec![0u8; cost.len()]
        .iter()
        .enumerate()
        .map(|(i, _)| (get_factors(&split, i), 0))
        .collect::<Vec<_>>();
    for i in 2..result.len() {
        let mut min_cost = cost[i];
        for j in 2..cost.len().min(i * 2) {
            if cost[j] + i.abs_diff(j) < min_cost {
                min_cost = cost[j] + i.abs_diff(j);
                result[i] = (
                    result[j].0.clone(),
                    i.checked_signed_diff(j).expect("Integer overfllow occured"),
                );
            }
        }
    }
    result
}
