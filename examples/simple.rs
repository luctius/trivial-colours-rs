extern crate trivial_colours;

use trivial_colours::*;


fn main() {
    print!("{}", Reset);

    println!("{}Colour::Black", Colour::Black);
    println!("{}Colour::Red", Colour::Red);
    println!("{}Colour::Green", Colour::Green);
    println!("{}Colour::Yellow", Colour::Yellow);
    println!("{}Colour::Blue", Colour::Blue);
    println!("{}Colour::Magenta", Colour::Magenta);
    println!("{}Colour::Cyan", Colour::Cyan);
    println!("{}Colour::White", Colour::White);

    println!("{}BgColour::Black", BgColour::Black);
    println!("{}BgColour::Red", BgColour::Red);
    println!("{}BgColour::Green", BgColour::Green);
    println!("{}BgColour::Yellow", BgColour::Yellow);
    println!("{}BgColour::Blue", BgColour::Blue);
    println!("{}BgColour::Magenta", BgColour::Magenta);
    println!("{}BgColour::Cyan", BgColour::Cyan);
    println!("{}BgColour::White", BgColour::White);

    println!("{}{}Colour::White on BgColour::Black", Colour::White, BgColour::Black);
    println!("{}{}Colour::Cyan on BgColour::Red", Colour::Cyan, BgColour::Red);
    println!("{}{}Colour::Magenta on BgColour::Green", Colour::Magenta, BgColour::Green);
    println!("{}{}Colour::Blue on BgColour::Yellow", Colour::Blue, BgColour::Yellow);
    println!("{}{}Colour::Yellow on BgColour::Blue", Colour::Yellow, BgColour::Blue);
    println!("{}{}Colour::Green on BgColour::Magenta", Colour::Green, BgColour::Magenta);
    println!("{}{}Colour::Red on BgColour::Cyan", Colour::Red, BgColour::Cyan);
    println!("{}{}Colour::Black on BgColour::White", Colour::Black, BgColour::White);

    print!("{}", Reset);
}
