use std::iter::FromIterator;

pub fn solve_a(input: &String) -> i32 {
    layers(input.clone())
        .iter()
        .min_by_key(|chunk| count_of(&'0', chunk))
        .map(|chunk| count_of(&'1', chunk) * count_of(&'2', chunk))
        .expect("No chunk had fewest 0s... :/") as i32
}

pub fn solve_b(input: &String) -> String {
    let image = Image::new(input.chars().collect(), 6, 25);
    let pixels: Vec<char> = image
        .map(|c| match c {
            '0' => ' ',
            '1' => 'X',
            _ => c,
        })
        .collect::<Vec<char>>();

    pixels
        .chunks(25)
        .map(|row| String::from_iter(vec!['\n'].iter().chain(row)))
        .collect::<Vec<String>>()
        .join("")
}

pub fn count_of(c: &char, cs: &Vec<char>) -> usize {
    let mut length = 0;
    cs.iter().filter(|x| x == &c).for_each(|_| length += 1);
    length
}

pub fn layers(input: String) -> Vec<Vec<char>> {
    let chars = input.chars();
    let collected = chars.collect::<Vec<char>>();
    collected
        .chunks(25 * 6)
        .map(|layer| layer.to_vec())
        .collect()
}

struct Image {
    height: usize,
    width: usize,
    data: Vec<char>,

    px: usize,
    layer: usize,
}

impl Image {
    pub fn new(data: Vec<char>, height: usize, width: usize) -> Image {
        Image {
            data,
            height,
            width,
            px: 0,
            layer: 0,
        }
    }

    fn index(&self) -> usize {
        self.layer * self.height * self.width + self.px
    }

    fn inbounds(&self) -> bool {
        self.index() < self.data.len() && self.px < self.height * self.width
    }

    fn current_val(&self) -> Option<char> {
        if self.inbounds() {
            Some(self.data[self.index()])
        } else {
            None
        }
    }
}

impl Iterator for Image {
    type Item = char;

    fn next(&mut self) -> Option<char> {
        while self.current_val() == Some('2') {
            self.layer += 1;
        }

        if self.inbounds() {
            let color = self.data[self.index()];
            self.layer = 0;
            self.px += 1;
            Some(color)
        } else {
            None
        }
    }
}
