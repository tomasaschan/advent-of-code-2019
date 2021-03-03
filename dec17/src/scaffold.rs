use util::map_2d::WorldMap;

pub fn get_map(s: &String) -> WorldMap<char> {
    let mut map = WorldMap::new();

    for (y, line) in s.split('\n').enumerate() {
        for (x, c) in line.chars().enumerate() {
            map.update((x as i32, y as i32), |_| c)
        }
    }

    map
}

pub fn display_map(map: &WorldMap<char>) {
    let ((xlo, ylo), (xhi, yhi)) = map.corners();

    for y in ylo..yhi + 1 {
        let line: String = (xlo..xhi + 1)
            .map(|x| match map.get(&(x, y)) {
                Some(c) => c,
                None => &' ',
            })
            .collect();

        println!("{}", line);
    }
}
