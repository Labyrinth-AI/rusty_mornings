struct Releases<'a> {
    years: &'a [i64],    // A reference to a slice of i64 values representing all years
    eighties: &'a [i64], // A reference to a slice of i64 values representing the 80s
    nineties: &'a [i64], // A reference to a slice of i64 values representing the 90s
}
fn jazz_releases<'a>(years: &'a [i64]) -> Releases<'a> {
    let eighties: &'a [i64] = &years[0..2];
    let nineties: &'a [i64] = &years[2..4];

    Releases {
        years,
        eighties,
        nineties,
    }
}

pub fn main() {
    let all_years: Vec<i64> = 
    // alloc
        vec![
            1980, 1985, 1990, 1995, 2000, 2000
        ];
    let releases: Releases<'_> = {
        jazz_releases(&all_years)
    }; 

    for year in releases.eighties.iter() {
        println!("Eighties year: {}", year);
    }
    // dealloc
}