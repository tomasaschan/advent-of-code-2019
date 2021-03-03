use intcode::Builder;

pub fn solve_a(input: &String) -> i128 {
    let mut count = 0;
    for y in 0..50 {
        for x in 0..50 {
            if is_inside(x, y, input) {
                count += 1;
            }
        }
    }
    count
}

pub fn solve_b(input: &String) -> i128 {
    let (mut x, mut y) = (0, 0);
    // find starting point inside the beam
    for j in 0..10 {
        for i in 0..10 {
            if is_inside(i, j, input) {
                x = i;
                y = j;
            }
        }
    }
    for i in 0..x {
        if is_inside(x - i, y, input) {
            x = x - i;
        }
    }
    // (x,y) should now be at the bottom edge of the beam

    loop {
        while !is_inside(x, y, input) {
            x += 1;
        }

        if is_inside(x + 99, y - 99, input) {
            return x * 10000 + (y - 99);
        }

        y += 1;
    }
    // not 13530863
}

fn is_inside(x: i128, y: i128, input: &String) -> bool {
    let o = Builder::new()
        .parse(input)
        .run_noninteractive(&mut vec![x, y].into_iter());
    o.get(0).unwrap() == &1
}
