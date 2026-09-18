fn concatenate(total: i64, next: i64) -> i64 {
    format!("{}{}", total, next).parse::<i64>().unwrap()
}

fn can_calculate(key: i64, value: &[i64], total: i64) -> bool {
    if value.is_empty() {
        return total == key;
    }

    let next = value[0];
    can_calculate(key, &value[1..], total + next) || can_calculate(key, &value[1..], total * next) || can_calculate(key, &value[1..], concatenate(total, next))
}

pub fn calculate(graph: Vec<(i64, Vec<i64>)>) -> i64 {
    let mut res = 0;

    for (key, value) in graph {
        if can_calculate(key, &value[1..], value[0]) {
            res += key;
        }
    }

    res
}
