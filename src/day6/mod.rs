pub fn solve() {
    let s1 = get_number_of_ways(59., 597.);
    let s2 = get_number_of_ways(79., 1234.);
    let s3 = get_number_of_ways(65., 1032.);
    let s4 = get_number_of_ways(75., 1328.);

    dbg!(s1 * s2 * s3 * s4);

    let p2 = get_number_of_ways(59796575., 597123410321328.);
    dbg!(p2);
}

fn get_number_of_ways(time: f64, best_dist: f64) -> u32 {
    // (time - t) * t = best_dist
    // - t^2 + time * t  - best_dist = 0
    // t1 = (-time + sqrt(time ^2 - 4 * -1 * -best_dist))/-2
    // t2 = (-time - sqrt(time ^2 - 4 * -1 * -best_dist))/-2

    let t1 = (-1. * time + (time * time - 4. * best_dist).sqrt()) / -2.;
    let t2 = (-1. * time - (time * time - 4. * best_dist).sqrt()) / -2.;

    let t1 = if t1 == t1.floor() { t1 + 1. } else { t1 };
    let t2 = if t2 == t2.floor() { t2 - 1. } else { t2 };

    dbg!(t1, t2);

    (t2.floor() - t1.ceil() + 1.) as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_number_of_ways_works() {
        assert_eq!(get_number_of_ways(7., 9.), 4);
        assert_eq!(get_number_of_ways(15., 40.), 8);
        assert_eq!(get_number_of_ways(30., 200.), 9);
    }
}
