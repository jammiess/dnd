use std::env::set_var;
use std::fs::File;
use std::io::{self, BufRead, Write};

use iced::button;
use iced::{executor, Application, Button, Column, Command, Element, Row, Settings, Text};

macro_rules! write_data {
    ($file:ident, $val:expr, $msg:literal) => {
        write!($file, "{}\n", $val).expect($msg);
    };
}

const STAT_FILE: &str = "character.txt";
const HEALTH: usize = 0;
const QI: usize = 1;
const SHELL: usize = 2;
type Stats = [u16; 3];

struct Character {
    name: String,
    curr_values: Stats,
    max_values: Stats,

    use_qi: button::State,
    reset_qi: button::State,

    dec_health: button::State,
    inc_health: button::State,
    reset_health: button::State,

    use_shell: button::State,
    reset_shell: button::State,

    save_button: button::State,
}

#[derive(Debug, Clone, Copy)]
pub enum AppMessage {
    UseQi,
    ResetQi,
    ResetHealth,
    IncHealth,
    DecHealth,
    UseShell,
    ResetShell,
    Save,
}

impl Character {
    fn save(&self) {
        let mut save_file = File::create(STAT_FILE).expect("Failed to open save file");
        write_data!(save_file, self.name, "Failed to write name");
        write_data!(
            save_file,
            self.max_values[HEALTH],
            "Failed to write max health"
        );
        write_data!(
            save_file,
            self.curr_values[HEALTH],
            "Failed to write curr health"
        );
        write_data!(save_file, self.max_values[QI], "Failed to write max qi");
        write_data!(save_file, self.curr_values[QI], "Failed to write curr qi");
        write_data!(
            save_file,
            self.max_values[SHELL],
            "Failed to write max shell"
        );
        write_data!(
            save_file,
            self.curr_values[SHELL],
            "Failed to write curr shell"
        );
    }
}

impl Application for Character {
    type Executor = executor::Default;
    type Message = AppMessage;
    type Flags = (Stats, Stats, String);

    fn new(flags: Self::Flags) -> (Self, Command<Self::Message>) {
        let app = Character {
            max_values: flags.0,
            curr_values: flags.1,
            name: flags.2,
            use_qi: button::State::new(),
            reset_qi: button::State::new(),
            dec_health: button::State::new(),
            inc_health: button::State::new(),
            reset_health: button::State::new(),
            use_shell: button::State::new(),
            reset_shell: button::State::new(),
            save_button: button::State::new(),
        };

        (app, Command::none())
    }

    fn title(&self) -> String {
        self.name.clone()
    }

    fn view(&mut self) -> Element<Self::Message> {
        Column::new()
            .push(Row::new().push(Text::new(self.name.clone()).size(50)))
            .push(
                Row::new()
                    .push(Text::new(format!("Health: {}", self.curr_values[HEALTH])).size(36))
                    .push(
                        Button::new(&mut self.reset_health, Text::new("Reset"))
                            .on_press(Self::Message::ResetHealth),
                    )
                    .push(
                        Button::new(&mut self.inc_health, Text::new("Increment"))
                            .on_press(Self::Message::IncHealth),
                    )
                    .push(
                        Button::new(&mut self.dec_health, Text::new("Decrement"))
                            .on_press(Self::Message::DecHealth),
                    ),
            )
            .push(
                Row::new()
                    .push(Text::new(format!("Qi: {}", self.curr_values[QI])).size(36))
                    .push(
                        Button::new(&mut self.reset_qi, Text::new("Reset"))
                            .on_press(Self::Message::ResetQi),
                    )
                    .push(
                        Button::new(&mut self.use_qi, Text::new("Use qi"))
                            .on_press(Self::Message::UseQi),
                    ),
            )
            .push(
                Row::new()
                    .push(Text::new(format!("Shell: {}", self.curr_values[SHELL])).size(36))
                    .push(
                        Button::new(&mut self.reset_shell, Text::new("Reset"))
                            .on_press(Self::Message::ResetShell),
                    )
                    .push(
                        Button::new(&mut self.use_shell, Text::new("Use"))
                            .on_press(Self::Message::UseShell),
                    ),
            )
            .push(Row::new().push(
                Button::new(&mut self.save_button, Text::new("Save")).on_press(Self::Message::Save),
            ))
            .into()
    }

    fn update(&mut self, m: Self::Message) -> Command<Self::Message> {
        match m {
            Self::Message::UseQi => self.curr_values[QI] = self.curr_values[QI].saturating_sub(1),
            Self::Message::ResetQi => self.curr_values[QI] = self.max_values[QI],
            Self::Message::DecHealth => {
                self.curr_values[HEALTH] = self.curr_values[HEALTH].saturating_sub(1)
            }
            Self::Message::IncHealth => {
                self.curr_values[HEALTH] = self.max_values[HEALTH].min(self.curr_values[HEALTH] + 1)
            }
            Self::Message::ResetHealth => self.curr_values[HEALTH] = self.max_values[HEALTH],
            Self::Message::UseShell => {
                self.curr_values[SHELL] = self.curr_values[SHELL].saturating_sub(1)
            }
            Self::Message::ResetShell => self.curr_values[SHELL] = self.max_values[SHELL],
            Self::Message::Save => self.save(),
        };

        Command::none()
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    set_var("FONTCONFIG_FILE", "/etc/fonts");

    let data = File::open(STAT_FILE)?;
    let mut l = io::BufReader::new(data).lines();
    let mut max: Stats = [0; 3];
    let mut curr: Stats = [0; 3];

    let character_name = l.next().expect("Missing character name")?;

    max[HEALTH] = l.next().expect("Missing max Health")?.parse::<u16>()?;
    curr[HEALTH] = l.next().expect("Missing curr Health")?.parse::<u16>()?;

    max[QI] = l.next().expect("Missing max Qi")?.parse::<u16>()?;
    curr[QI] = l.next().expect("Missing curr Qi")?.parse::<u16>()?;

    max[SHELL] = l
        .next()
        .expect("Missing max shell master")?
        .parse::<u16>()?;
    curr[SHELL] = l
        .next()
        .expect("Missing curr shell master")?
        .parse::<u16>()?;

    Character::run(Settings::with_flags((max, curr, character_name)));

    Ok(())
}
