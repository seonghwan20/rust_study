fn main() {
    let cat = ("Furry McFurson", 3.5);

    // TODO: Destructure the `cat` tuple in one statement so that the println works.
    // let /* your pattern here */ = cat;
    let name = &cat.1;
    let age = &cat.0;
    println!("{name} is {age} years old");
}