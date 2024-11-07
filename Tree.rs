fn main() {
    let triangles = 5;

    for i in 1..=triangles {
        let max_width = 2 * i - 1;

        for j in 1..=i {
            let stars = 2 * j - 1;
            let spaces = max_width - stars;

            (0..spaces / 2).for_each(|_| print!(" "));
            (0..stars).for_each(|_| print!("*"));
            (0..spaces / 2).for_each(|_| print!(" "));

            println!();
        }
    }

    let trunk_width = triangles * 2 - 1;
    let trunk_height = 1;

    for _ in 0..trunk_height {
        (0..(trunk_width / 2)).for_each(|_| print!(" "));
        (0..trunk_width).for_each(|_| print!("*"));
        println!();
    }
}
