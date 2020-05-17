pub fn run() {
    println!("Hello from the print file!");

    println!("{}", 1);

    //Positional arguments
    println!("{0} is from {1} and {0} likes to {2}", "Brad", "Mass", "code");

    //Named arguments
    println!("{name} like to play {activity}", name = "John", activity = "baseball");

    //Placeholder traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);

    //Placeholder for debug trait
    println!("{:?}", (12, true, "hello"));

    //Basic math
    println!("10 + 10 = {}", 10 + 10)

}