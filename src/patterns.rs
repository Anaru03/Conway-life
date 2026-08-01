pub type Pattern = Vec<(i32, i32)>;

// ---- Still lifes ----

pub fn block() -> Pattern {
    vec![(0, 0), (1, 0), (0, 1), (1, 1)]
}

pub fn beehive() -> Pattern {
    vec![(1, 0), (2, 0), (0, 1), (3, 1), (1, 2), (2, 2)]
}

pub fn loaf() -> Pattern {
    vec![(1, 0), (2, 0), (0, 1), (3, 1), (1, 2), (3, 2), (2, 3)]
}

pub fn boat() -> Pattern {
    vec![(0, 0), (1, 0), (0, 1), (2, 1), (1, 2)]
}

pub fn tub() -> Pattern {
    vec![(1, 0), (0, 1), (2, 1), (1, 2)]
}

// ---- Oscillators ----

pub fn blinker() -> Pattern {
    vec![(0, 1), (1, 1), (2, 1)]
}

pub fn toad() -> Pattern {
    vec![(1, 0), (2, 0), (3, 0), (0, 1), (1, 1), (2, 1)]
}

pub fn beacon() -> Pattern {
    vec![(0, 0), (1, 0), (0, 1), (1, 1), (2, 2), (3, 2), (2, 3), (3, 3)]
}

pub fn pulsar() -> Pattern {
    let arm: [i32; 6] = [2, 3, 4, 8, 9, 10];
    let mut cells = Vec::new();

    for &x in arm.iter() {
        cells.push((x, 0));
        cells.push((x, 5));
        cells.push((x, 7));
        cells.push((x, 12));
    }
    for &y in arm.iter() {
        cells.push((0, y));
        cells.push((5, y));
        cells.push((7, y));
        cells.push((12, y));
    }

    cells
}

pub fn pentadecathlon() -> Pattern {
    vec![
        (2, 0), (3, 0),
        (1, 1), (4, 1),
        (0, 2), (5, 2),
        (1, 3), (4, 3),
        (2, 4), (3, 4),
        (2, 5), (3, 5),
        (1, 6), (4, 6),
        (0, 7), (5, 7),
        (1, 8), (4, 8),
        (2, 9), (3, 9),
    ]
}

// ---- Spaceships ----

pub fn glider() -> Pattern {
    vec![(1, 0), (2, 1), (0, 2), (1, 2), (2, 2)]
}

pub fn lwss() -> Pattern {
    vec![
        (1, 0), (4, 0),
        (0, 1),
        (0, 2), (4, 2),
        (0, 3), (1, 3), (2, 3), (3, 3),
    ]
}