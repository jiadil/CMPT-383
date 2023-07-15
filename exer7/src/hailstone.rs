pub fn hailstone(n: u64) -> u64 {
    if (n % 2) == 0 {
        n / 2
    } else {
        3 * n + 1
    }
}

pub fn hailstone_sequence_append(n: u64) -> Vec<u64> {
    let mut res = Vec::new();
    let mut value = n;

    while value != 1 {
        res.push(value);
        value = hailstone(value);
    }

    res.push(1);

    res
}

pub fn hailstone_sequence_prealloc(n: u64) -> Vec<u64> {
    let mut length = 1;
    let mut i = n;

    while i != 1 {
        i = hailstone(i);
        length += 1;
    }

    let mut res = Vec::with_capacity(length);
    let mut value = n;

    while value != 1 {
        res.push(value);
        value = hailstone(value);
    }

    res.push(1);

    res
}

/****************************** Relative speed of two hailstone functions ******************************/
/*
n = 7:
n=7/hailstone_sequence_append_7
                        time:   [173.34 ns 183.19 ns 193.41 ns]

n=7/hailstone_sequence_prealloc_7
                        time:   [32.068 ns 32.194 ns 32.366 ns]
Found 29 outliers among 100 measurements (29.00%)


n = 162964:
n=162964/hailstone_sequence_append_162964
                        time:   [228.85 ns 235.68 ns 241.99 ns]

n=162964/hailstone_sequence_prealloc_162964
                        time:   [80.395 ns 80.992 ns 81.639 ns]
Found 6 outliers among 100 measurements (6.00%)


n = 686901248:
n=686901248/hailstone_sequence_append_686901248
                        time:   [520.57 ns 531.20 ns 542.74 ns]
Found 16 outliers among 100 measurements (16.00%)

n=686901248/hailstone_sequence_prealloc_686901248
                        time:   [512.30 ns 512.94 ns 513.63 ns]
Found 13 outliers among 100 measurements (13.00%)
*/