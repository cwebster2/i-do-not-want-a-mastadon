use rand::Rng;

fn roll_d6() -> i32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(1..=6)
}

fn test_nirvana() {
    let mut rolls = [0; 3];
    for roll in rolls.iter_mut() {
        *roll = roll_d6();
    }
    if rolls[0] == rolls[1] && rolls[0] == rolls[2] {
        println!("You finally decipher what the Mastadon was trying to tell you.  You live the rest of your life in a state of blissful enlightenment, in harmony with your new friend.  You are also a vegan now.");
        std::process::exit(0);
    }
}

fn main() {
    println!("I do not want a mastadon!");
    let mut confusion = 0;
    let mut damages = 0;
    let mut thinking = 0;
    loop {
        match roll_d6() {
            1..=3 => {
                // A day with your mastadon
                print!("A day with your mastadon: ");
                match roll_d6() {
                    1 => {
                        println!("The mastadon tells you a secret about your family it could not possibly know.");
                        confusion += 1;
                        test_nirvana();
                    },
                    2 => {
                        println!("Your mastadon decentralizes all over the carpet.");
                        damages += 1;
                    },
                    3 => {
                        println!("You take the mastadon on a walk in your gated neighborhood.");
                        thinking += 1;
                    },
                    4 => {
                        println!("The mastadon federates violently through a window, which now needs replacing.");
                        confusion += 1;
                        test_nirvana();
                        damages += 1;
                    },
                    5 => {
                        println!("The mastadon won't move unless you know the password.  You do not know the password.");
                        confusion += 2;
                        test_nirvana();
                    },
                    6 => {
                        println!("The mastadon bashes a hole in your roof, and now claims to have a much better view of other mastadons.");
                        damages += 2;
                    },
                    _ => { panic!("d6 rolled an impossible result") }
                }
            },
            4..=5 => {
                // An evening at home
                print!("An evening at home: ");
                match roll_d6() {
                    1 => {
                        println!("The mastadon trumpets ERROR at you.  You ponder your mistake.");
                        thinking += 1;
                    },
                    2 => {
                        println!("It wears so many hats. How will you wash all these hats?");
                        confusion += 1;
                        test_nirvana();
                    },
                    3 => {
                        println!("It reads moral philosophy and particle physics.  All day.  It  will not let you read with it.");
                        confusion += 1;
                        test_nirvana();
                    },
                    4 => {
                        println!("Your friends tell you how wonderful their mastadons are.  You burn with envy.");
                        confusion += 1;
                        test_nirvana();
                        thinking += 1;
                    },
                    5 => {
                        println!("Before the mastadon will sleep, you must clean its many whistles and gears.");
                        confusion += 1;
                        test_nirvana();
                        thinking += 1;
                    },
                    6 => {
                        println!("During the night, your mastadon holds not one, not two, but a third party.  There is a platform.  And servers.");
                        damages += 1;
                    },
                    _ => { panic!("d6 rolled an impossible result") }
                }
            },
            6 => {
                // Anger
                print!("You anger your mastadon with your questions, and it sits on you: ");
                match roll_d6() {
                    6 => {
                        println!("You never emerge.");
                        std::process::exit(4);
                    }
                    _ => {
                        println!("You emerge, but you are changed.");
                    }
                }
            },
            _ => { panic!("d6 rolled an impossible result") }
        }
        if confusion >= 10 {
            println!("You finally lose your temper with the wretched creature and confront it.  The argument is brief, because you have no idea what it is trying to tell you, and eventually it crushes you to death with its trunk");
            std::process::exit(1);
        }
        if damages >= 10 {
            println!("You lose all your money and your livelihood is destroyed, reduced to gigantic footrpints in the ashes. The mastadon abandons you in search of someone else to inconvienence.");
            std::process::exit(2);
        }
        if thinking >= 10 {
            println!("You decide that mastadons are not for you.  Your slip away into the night with the last of your remaining savings faking your death.  Perhaps you'll build a gigantic pillowfort.  Or collect tumblers.  Something quiet.");
            std::process::exit(3);
        }
    }
}
