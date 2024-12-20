pub fn test() {
    let mut t = vec![];
    for r in -20..=20_i32 {
        for c in -20..=20_i32 {
            if r.abs() + c.abs() <= 20 && !(r == 0 && c == 0) {
                t.push((r, c, r.abs() + c.abs()));
            }
        }
    }
    t.sort();
    t.dedup();
    println!("oops {:?}", t);
}
