use std::io;

fn main() {
    println!("Vending machine");
    println!("Enter your choice: 1.Coffee  2.Tea");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let choice: i32 = input.trim().parse().expect("Invalid choice");

    if choice == 1 {
        println!("Enter sub-choice: 1.Black coffee  2.Cold coffee");

        let mut subchoice = String::new();
        io::stdin()
            .read_line(&mut subchoice)
            .expect("Failed to read line");
        let sub_choice: i32 = subchoice.trim().parse().expect("Invalid sub-choice");

        if sub_choice == 1 {
            println!("You ordered Black coffee");
        } 
        if sub_choice == 2{
          println!("You ordered Cold coffee");
        }
        if sub_choice > 2 {
            println!("Invalid sub-choice");
        }
    } else if choice == 2 {
        println!("Enter sub-choice: 1.Rose tea  2.Lemon Tea");
        let mut subchoice = String::new();
        io::stdin()
            .read_line(&mut subchoice)
            .expect("Failed to read line");
        let sub_choice_t: i32 = subchoice.trim().parse().expect("Invalid sub-choice");

        if sub_choice_t == 1 {
            println!("You ordered Rose Tea");
        } 
            if sub_choice_t == 2{
              println!("You ordered Lemon Tea");
            }
            if sub_choice_t > 2 {
              println!("Invalid sub-choice");
            }
    } else {
        println!("Invalid choice");
    }
}
