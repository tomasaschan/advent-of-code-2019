use intcode::Builder;
use std::sync::mpsc::channel;
use util::map_2d::*;

pub fn solve_a(input: &String) -> i128 {
    let mut map: WorldMap<i128> = WorldMap::new();
    paint_hull(&mut map, 0, input);
    map.get_terrain().keys().count() as i128
}

pub fn solve_b(input: &String) -> (i128) {
    let mut map: WorldMap<i128> = WorldMap::new();
    map.update((0, 0), |_| 1);
    paint_hull(&mut map, 1, input);

    let ((xmin, ymin), (xmax, ymax)) = map.corners();

    for y in ymin..(ymax + 1) {
        for x in xmin..(xmax + 1) {
            print!(
                "{}",
                map.get(&(x, ymin + ymax - y))
                    .map(|i| match i {
                        0 => '.',
                        1 => '#',
                        _ => panic!("unknown color {}", i),
                    })
                    .unwrap_or('.')
            );
        }
        println!("");
    }
    0
}

pub fn paint_hull(map: &mut WorldMap<i128>, start: i128, program: &String) {
    let (in_tx, in_rx) = channel();
    let (out_tx, out_rx) = channel();

    Builder::new().parse(program).run(in_rx, out_tx);

    let mut pos = (0, 0);
    let mut dir = Direction::Up;

    in_tx.send(start).unwrap();

    loop {
        let clr = out_rx.recv().expect("error getting color");
        let trn = out_rx.recv().expect("error getting turn");

        map.update(pos, |_| clr);
        dir = match trn {
            0 => turn_left(&dir),
            1 => turn_right(&dir),
            _ => panic!("Got instruction to turn an unknown direction: {}", trn),
        };
        pos = move_to(&dir, pos);

        let new_clr = map.get(&pos).unwrap_or(&0);
        match in_tx.send(*new_clr) {
            Ok(_) => (),
            Err(_) => break,
        }
    }
}
