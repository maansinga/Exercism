use clock::Clock;

fn main(){
//    let clock = Clock::new(10, 3).add_minutes(-30);
//    println!("Computer: {:?} Expected:{}", clock, "09:33");
//
//    let clock = Clock::new(0, 3).add_minutes(-4);
//    println!("Computer: {:?} Expected:{}", clock, "23:59");
//
//    let clock = Clock::new(23, 59).add_minutes(2);
//    println!("Computer: {:?} Expected:{}", clock, "00:01");
//
//    println!("");
//    let clock = Clock::new(10, 0).add_minutes(3);
//    println!("Computer: {:?} Expected:{}", clock, "10:03");

    println!("");
    let clock = Clock::new(2, -60).to_string();
    println!("Computer: {:?} Expected:{}", clock, "01:00");

//    println!("{} {}", (60 + -90 % 60) % 60, -90 / 60);
//    println!("{} {}", (60 + 90 % 60) % 60, 90 / 60);
//    println!("{}", ( ((-27 - 60) / 60)) as i32);
//    println!("{}", ( ((27 - 60) / 60) as u32) as i32);
}