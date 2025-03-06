const WIDTH: usize = 30;
const HEIGHT: usize = 30;

fn main() {
    let w = WIDTH as f32;
    let h = HEIGHT as f32;

    for col in 0..HEIGHT {
        for row in 0..WIDTH {
            let x = col as f32;
            let y = row as f32;

            let cx = (2.0 * x - w) / h;
            let cy = (2.0 * y - h) / h;
            let mut d = (cx * cx + cy * cy).sqrt();
            d -= 0.5;
            d += 0.01 * h / (2.0 * (x - y) + h - w);
            d = d.abs();
            d = 0.1 / d;

            let chars = " -+=<@#".chars().collect::<Vec<char>>();
            let index = ((d / (1.0 + d) * 7.0).floor() as usize).min(chars.len() - 1);
            let c = chars[index];

            print!("{}{}", c, c);
        }
        println!();
    }
}
