pub enum ParseState {
    Number(i16, u32),
    Else,
}

pub const NEIGHBOURS: [(i16, i16); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
    (1, 0),
    (1, -1),
    (0, -1),
];
