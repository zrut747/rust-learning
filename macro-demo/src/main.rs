use derives_demo::HelloMacro;

#[derive(HelloMacro)]
enum Robot {
    #[oic_column(name = "alex_name", age = 22, comment = "这是Alex")]
    Alex,
    #[oic_column(name = "bob_name", age = 50, comment = "这是Bob")]
    Bob,
    Apple,
}

fn main() {
    // test comment: Alex: "这是Alex", ----- Bob: "这是Bob"
    println!("test comment: Alex: {:?}, ----- Bob: {:?}", Robot::Alex.comment(), Robot::Bob.comment());
    // test comment apple: ""
    println!("test comment apple: {:?}", Robot::Apple.comment());
}
