fn greet_world() {
    println!("Hello, world!");
    let southern_germany = "Grüß Gott!";
    let japan = "ハロー・ワールド";
    let portuguese = "Olá, Mundo!";
    let regions = [southern_germany, japan, portuguese];
    for region in regions.iter() {
        println!("{}", &region);
    }
}

fn main() {
    greet_world();
}