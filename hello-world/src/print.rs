pub fn run() {
    // print to console
    println!("{} is from {}", "oskar", "wwa");
    //positional args
    println!(
        "{0} is from {1} and {0} likes {2}",
        "oskar", "wwa", "vaping"
    );
    // named args
    println!(
        "{name} likes {activity}",
        name = "oskar",
        activity = "vaping"
    )
}
