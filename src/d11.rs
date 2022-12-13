pub fn part_01() -> i64 {
    let mut monkeys = inputs();

    for _round_ix in 0..20 {
        monkeys = play_round(monkeys, true);
    }

    monkeys.sort_by_key(|monkey| std::cmp::Reverse(monkey.inspected));

    monkeys.into_iter().take(2).map(|m| m.inspected).product()
}

const MOD_BY: i64 = 19 * 3 * 13 * 7 * 5 * 11 * 17 * 2;

fn play_round(monkeys: Vec<Monkey>, part_1: bool) -> Vec<Monkey> {
    let mut new_state = monkeys.clone();

    for ix in 0..monkeys.len() {
        let monkey = new_state[ix].clone();
        for item in monkey.items.clone().iter() {
            let mut new = (monkey.operation)(*item);
            if part_1 {
                new = new / 3;
            } else {
                new = new % MOD_BY;
            }

            new_state[(monkey.test)(new)].items.push(new);
        }
        new_state[ix].inspected += monkey.items.len() as i64;
        new_state[ix].items.clear();
    }

    new_state
}

pub fn part_02() -> i64 {
    let mut monkeys = inputs();

    for _round_ix in 0..10000 {
        monkeys = play_round(monkeys, false);
    }

    monkeys.sort_by_key(|monkey| std::cmp::Reverse(monkey.inspected));

    monkeys.into_iter().take(2).map(|m| m.inspected).product()
}

#[derive(Clone, Debug)]
struct Monkey {
    items: Vec<i64>,
    operation: fn(i64) -> i64,
    test: fn(i64) -> usize,
    inspected: i64,
}

fn inputs() -> Vec<Monkey> {
    vec![
        Monkey {
            items: vec![85, 77, 77],
            operation: |x| x * 7,
            test: |x| if x % 19 == 0 { 6 } else { 7 },
            inspected: 0,
        },
        Monkey {
            items: vec![80, 99],
            operation: |x| x * 11,
            test: |x| if x % 3 == 0 { 3 } else { 5 },
            inspected: 0,
        },
        Monkey {
            items: vec![74, 60, 74, 63, 86, 92, 80],
            operation: |x| x + 8,
            test: |x| if x % 13 == 0 { 0 } else { 6 },
            inspected: 0,
        },
        Monkey {
            items: vec![71, 58, 93, 65, 80, 68, 54, 71],
            operation: |x| x + 7,
            test: |x| if x % 7 == 0 { 2 } else { 4 },
            inspected: 0,
        },
        Monkey {
            items: vec![97, 56, 79, 65, 58],
            operation: |x| x + 5,
            test: |x| if x % 5 == 0 { 2 } else { 0 },
            inspected: 0,
        },
        Monkey {
            items: vec![77],
            operation: |x| x + 4,
            test: |x| if x % 11 == 0 { 4 } else { 3 },
            inspected: 0,
        },
        Monkey {
            items: vec![99, 90, 84, 50],
            operation: |x| x * x,
            test: |x| if x % 17 == 0 { 7 } else { 1 },
            inspected: 0,
        },
        Monkey {
            items: vec![50, 66, 61, 92, 64, 78],
            operation: |x| x + 3,
            test: |x| if x % 2 == 0 { 5 } else { 1 },
            inspected: 0,
        },
    ]
}
