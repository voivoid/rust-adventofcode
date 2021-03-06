use crate::utils::parsing::{parse_decimal, parse_ws};
use nom::{bytes::complete::tag, character::complete::char, combinator::map, ToUsize};

type Coord = usize;
type Brigthness = u8;

const GRID_SIDE: Coord = 1000;
type Grid = Vec<Brigthness>;

type NomResult<'a, T> = nom::IResult<&'a str, T>;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Command {
    On,
    Off,
    Toggle,
}

#[derive(Debug, PartialEq, Eq)]
struct Instruction {
    cmd: Command,
    left: Coord,
    top: Coord,
    right: Coord,
    bottom: Coord,
}

fn parse_command(input: &str) -> NomResult<Command> {
    let on = map(tag("turn on"), |_| Command::On);
    let off = map(tag("turn off"), |_| Command::Off);
    let toggle = map(tag("toggle"), |_| Command::Toggle);

    nom::branch::alt((on, off, toggle))(input)
}

fn parse_coords(input: &str) -> NomResult<(Coord, Coord)> {
    let (input, c1) = parse_decimal::<Coord>(input)?;
    let (input, _) = char(',')(input)?;
    let (input, c2) = parse_decimal::<Coord>(input)?;

    Ok((input, (c1, c2)))
}

fn parse_instruction(input: &str) -> Instruction {
    fn parse_instruction_impl(input: &str) -> NomResult<Instruction> {
        let (input, cmd) = parse_command(input)?;
        let (input, coords1) = parse_ws(parse_coords)(input)?;
        let (input, _) = nom::bytes::complete::tag("through")(input)?;
        let (input, coords2) = parse_ws(parse_coords)(input)?;

        Ok((
            input,
            Instruction {
                cmd: cmd,
                left: coords1.0,
                top: coords1.1,
                right: coords2.0,
                bottom: coords2.1,
            },
        ))
    }

    match nom::combinator::all_consuming(parse_instruction_impl)(input) {
        Ok((_, instructions)) => instructions,
        Err(e) => panic!(format!("Failed to parse instructions: {:?}", e)),
    }
}

type ChangeBrightness = fn(Brigthness) -> Brigthness;
type ChangeBrightnessFactory = fn(Command) -> ChangeBrightness;

fn change_brightness_factory_a(cmd: Command) -> ChangeBrightness {
    match cmd {
        Command::On => |_| 1,
        Command::Off => |_| 0,
        Command::Toggle => |brightness| (brightness + 1) % 2,
    }
}

fn change_brightness_factory_b(cmd: Command) -> ChangeBrightness {
    match cmd {
        Command::On => |brightness| brightness + 1,
        Command::Off => |brightness| if brightness != 0 { brightness - 1 } else { 0 },
        Command::Toggle => |brightness| brightness + 2,
    }
}

fn get_grid_brightness(grid: &mut Grid, x: Coord, y: Coord) -> &mut Brigthness {
    grid.get_mut(y * GRID_SIDE + x).unwrap()
}

fn apply_instruction(
    mut grid: Grid,
    instruction: Instruction,
    change_brightness_factory: ChangeBrightnessFactory,
) -> Grid {
    let change_brightness = change_brightness_factory(instruction.cmd);
    for y in instruction.top..instruction.bottom + 1 {
        for x in instruction.left..instruction.right + 1 {
            let brightness = get_grid_brightness(&mut grid, x, y);
            *brightness = change_brightness(*brightness);
        }
    }

    grid
}

fn make_initial_grid() -> Grid {
    let mut grid = std::vec::Vec::new();
    grid.resize(GRID_SIDE * GRID_SIDE, 0);
    grid
}

fn solve(
    input: impl std::io::BufRead,
    change_brightness_factory: ChangeBrightnessFactory,
) -> usize {
    let instructions = input
        .lines()
        .map(|line| parse_instruction(line.unwrap().as_str()));

    let final_grid = instructions.fold(make_initial_grid(), |grid, instruction| {
        apply_instruction(grid, instruction, change_brightness_factory)
    });

    final_grid
        .iter()
        .map(|brightness| brightness.to_usize())
        .sum()
}

pub fn solve_a(input: impl std::io::BufRead) -> usize {
    solve(input, change_brightness_factory_a)
}

pub fn solve_b(input: impl std::io::BufRead) -> usize {
    solve(input, change_brightness_factory_b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_parsing() {
        assert_eq!(Ok(("", Command::On)), parse_command("turn on"));
        assert_eq!(Ok(("", Command::Off)), parse_command("turn off"));
        assert_eq!(Ok(("", Command::Toggle)), parse_command("toggle"));

        assert_eq!(
            Instruction {
                cmd: Command::On,
                left: 0,
                top: 0,
                right: 999,
                bottom: 999
            },
            parse_instruction("turn on 0,0 through 999,999")
        );
    }

    #[test]
    fn check_a() {
        assert_eq!(4, solve_a(&b"turn on 0,0 through 1,1"[..]));
        assert_eq!(
            100 - 4,
            solve_a(&b"turn on 0,0 through 9,9\nturn off 4,4 through 5,5"[..])
        );
        assert_eq!(10, solve_a(&b"toggle 0,0 through 9,0"[..]));
        assert_eq!(
            0,
            solve_a(&b"toggle 0,0 through 9,0\ntoggle 0,0 through 9,0"[..])
        );
    }

    #[test]
    fn check_b() {
        assert_eq!(1, solve_b(&b"turn on 0,0 through 0,0"[..]));
        assert_eq!(20, solve_b(&b"toggle 0,0 through 9,0"[..]));
    }
}
