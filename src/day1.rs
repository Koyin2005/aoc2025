/*
 Dial through 0 to 99
*/

pub fn password() -> usize {
    #[derive(Debug, Clone, Copy)]
    pub enum Action {
        Left(isize),
        Right(isize),
    }

    fn read_file() -> String {
        std::fs::read_to_string("src/day_1_input.txt").expect("It should be")
    }

    fn actions() -> Vec<Action> {
        let txt = read_file();
        txt.lines()
            .filter_map(|line| {
                let line = line.trim();
                match line.split_at_checked(1)? {
                    ("L", rest) => rest.parse::<isize>().ok().map(Action::Left),
                    ("R", rest) => rest.parse::<isize>().ok().map(Action::Right),
                    _ => None,
                }
            })
            .collect()
    }
    let actions = actions();
    let mut dial: isize = 50;
    let mut password = 0;
    for action in actions {
        match action {
            Action::Left(mut offset) => {
                while offset >= 100 {
                    password += 1;
                    offset -= 100;
                }
                dial = if dial - offset < 0 {
                    let new_dial = 100 + (dial - offset);
                    if dial != 0 {
                        //println!("0 from left rollover");
                        password += 1;
                    }
                    new_dial
                } else {
                    let new_dial = dial - offset;
                    if new_dial == 0 {
                        //println!("0 from left");
                        password += 1;
                    }
                    new_dial
                };
            }
            Action::Right(mut offset) => {
                while offset >= 100 {
                    password += 1;
                    offset -= 100;
                }
                dial = if dial + offset >= 100 {
                    let new_dial = (dial + offset) - 100;

                    if dial != 0 {
                        //println!("0 from right rollover");
                        password += 1;
                    }
                    new_dial
                } else {
                    dial + offset
                };
            }
        }
        //println!("{dial} {action:?}");
        assert!(dial >= 0 && dial <= 99, "{dial}");
    }
    password
}
