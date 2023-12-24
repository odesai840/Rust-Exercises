fn main() {
    let mut point = [0, 1];
    let mut _x = point[0];
    let y = &mut point[1];
    _x += 1;
    *y += 1;
    println!("{} {}", point[0], point[1]);
}