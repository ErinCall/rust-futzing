enum Bin {
    CityA,
    CityB,
}

struct Counter {
    bins: Vec<Option<Bin>>,
    n: uint
}

impl Counter {
    fn binAt(&self, i:uint) -> Option<Bin> {
        match *self {
            Counter {n: n, bins: bins} if i <= (2 * n) && i >= (-2 * n) => bins.slice(i, i)[0]
        }
    }
}


fn main() {
}
