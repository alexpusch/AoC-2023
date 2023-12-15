use regex::Regex;

pub fn solve() {
    let input = include_str!("./input.txt");

    dbg!(hash_all(input));

    let commands = parse(input);
    let mut machine = Machine::new();
    machine.run(commands);

    dbg!(machine.get_focusing_power());
}

#[derive(PartialEq, Debug)]
enum Op {
    Eq(u32),
    Dash,
}

#[derive(PartialEq, Debug)]
struct Command {
    pub label: String,
    pub op: Op,
}

impl Command {
    pub fn from_str(input: &str) -> Self {
        let command_regex = Regex::new(r"([a-zA-Z]+)(-|=\d)").unwrap();

        let parts = command_regex.captures(input).unwrap();

        let label = parts.get(1).unwrap().as_str().to_string();
        let op = parts.get(2).unwrap().as_str();

        let op = if op.starts_with("-") {
            Op::Dash
        } else {
            Op::Eq(op[1..2].parse().unwrap())
        };

        Self { label, op }
    }
}

#[derive(Clone, Debug, PartialEq)]
struct Lens {
    pub label: String,
    pub focal: u32,
}

#[derive(Default, Clone, Debug)]
struct LensBox {
    pub lenses: Vec<Lens>,
}

#[derive(Debug)]
struct Machine {
    pub boxes: Vec<LensBox>,
}

impl Machine {
    fn new() -> Self {
        Self {
            boxes: vec![LensBox::default(); 256],
        }
    }

    fn run(&mut self, commands: Vec<Command>) {
        for command in commands {
            let label = command.label.clone();

            match command.op {
                Op::Eq(focal) => {
                    let box_i = hash(&label) as usize;
                    let lens_box = self.boxes.get_mut(box_i).unwrap();

                    if let Some(existing_lens) =
                        lens_box.lenses.iter_mut().find(|l| l.label == label)
                    {
                        existing_lens.focal = focal;
                    } else {
                        lens_box.lenses.push(Lens { label, focal });
                    }
                }
                Op::Dash => {
                    let box_i = hash(&label) as usize;
                    let lens_box = self.boxes.get_mut(box_i).unwrap();
                    lens_box.lenses.retain(|l| l.label != label)
                }
            }
        }
    }

    fn get_focusing_power(&self) -> u32 {
        self.boxes
            .iter()
            .enumerate()
            .flat_map(|(i, b)| b.lenses.iter().enumerate().zip(std::iter::repeat(i)))
            .map(|((lens_i, lens), box_i)| (box_i as u32 + 1) * (lens_i as u32 + 1) * lens.focal)
            .sum()
    }
}

fn parse(input: &str) -> Vec<Command> {
    input.split(",").map(Command::from_str).collect()
}

fn hash_all(input: &str) -> u32 {
    input.split(',').map(hash).sum()
}

fn hash(input: &str) -> u32 {
    dbg!(input);
    let mut cur = 0;

    for char in input.chars() {
        let code = (char as u8) as u32;
        cur = (cur + code) * 17 % 256
    }

    cur
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn hash_works() {
        assert_eq!(hash("rn=1"), 30);
        assert_eq!(hash("cm-"), 253);
        assert_eq!(hash("qp=3"), 97);
        assert_eq!(hash("cm=2"), 47);
        assert_eq!(hash("qp-"), 14);
        assert_eq!(hash("pc=4"), 180);
        assert_eq!(hash("ot=9"), 9);
        assert_eq!(hash("ab=5"), 197);
        assert_eq!(hash("pc-"), 48);
        assert_eq!(hash("pc=6"), 214);
        assert_eq!(hash("ot=7"), 231);
    }

    #[test]
    fn hash_all_works() {
        assert_eq!(
            hash_all("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"),
            1320
        );
    }

    #[test]
    fn command_from_str_works() {
        assert_eq!(
            Command::from_str("rn=1"),
            Command {
                label: "rn".into(),
                op: Op::Eq(1)
            }
        );

        assert_eq!(
            Command::from_str("cm-"),
            Command {
                label: "cm".into(),
                op: Op::Dash
            }
        );
    }

    #[test]
    fn machine_run_works() {
        let commands = parse("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7");
        let mut machine = Machine::new();
        machine.run(commands);

        assert_eq!(
            machine.boxes[0].lenses,
            vec![
                Lens {
                    label: "rn".into(),
                    focal: 1,
                },
                Lens {
                    label: "cm".into(),
                    focal: 2,
                }
            ]
        );

        assert_eq!(
            machine.boxes[3].lenses,
            vec![
                Lens {
                    label: "ot".into(),
                    focal: 7,
                },
                Lens {
                    label: "ab".into(),
                    focal: 5,
                },
                Lens {
                    label: "pc".into(),
                    focal: 6,
                },
            ]
        );

        assert_eq!(machine.get_focusing_power(), 145);
    }
}
