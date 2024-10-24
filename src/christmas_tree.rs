pub fn main() {
    const TRIANGLES: u8 = 6;
    let width = TRIANGLES * 2 + 1;

    fn m(a: u8, size: u8) -> u8 {
        size - 1 - a
    }

    fn calculate_diff(n: u8) -> u8 {
        TRIANGLES - n
    }

    for n in 0..TRIANGLES {
        let half = width / 2;

        for y in 0..width - TRIANGLES - calculate_diff(n) {
            for x in 0..width {
                let q1 = x + y < half;
                let q2 = m(x, width) + y < half;
                let c = if q1 || q2 { ' ' } else { '*' };
                print!("{c}");
            }
            println!();
        }
    }
}
