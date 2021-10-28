fn main() {
    println!("Hello, world!");
}

#[test]
fn t() {
    (0..3)
        .map(|x| {
            // 0 1 2
            println!("after (..): {}", x);
            x
        })
        // [0, 0, ]
        .flat_map(|x| x * 1..x * 2)
        .map(|x| {
            // 1 2 3
            println!("after flat_map: {}", x);
            return x;
        })
        .enumerate()
        .filter(|&(i, x)| (i + x) % 3 == 0)
        .for_each(|(i, x)| println!("{}:{}", i, x));
}
