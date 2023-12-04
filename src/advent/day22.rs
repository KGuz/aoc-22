use super::*;
use itertools::Itertools;
use nom::{
    branch::alt,
    character::complete::{char, i32},
    multi::many1,
    IResult, Parser,
};
use pt::P2;
use std::collections::HashMap;

pub struct Day22 {
    /* --- Day 22: Monkey Map ---
    The monkeys take you on a surprisingly easy trail through the jungle.
    They're even going in roughly the right direction according to your
    handheld device's Grove Positioning System.

    As you walk, the monkeys explain that the grove is protected by a force
    field. To pass through the force field, you have to enter a password; doing
    so involves tracing a specific path on a strangely-shaped board.

    At least, you're pretty sure that's what you have to do; the elephants
    aren't exactly fluent in monkey.

    The monkeys give you notes that they took when they last saw the password
    entered (your puzzle input).

    For example:
            ...#
            .#..
            #...
            ....
    ...#.......#
    ........#...
    ..#....#....
    ..........#.
            ...#....
            .....#..
            .#......
            ......#.

    10R5L5R10L4R5L5

    The first half of the monkeys' notes is a map of the board. It is comprised
    of a set of open tiles (on which you can move, drawn .) and solid walls
    (tiles which you cannot enter, drawn #).

    The second half is a description of the path you must follow. It consists
    of alternating numbers and letters:

    - A number indicates the number of tiles to move in the direction you are
    facing. If you run into a wall, you stop moving forward and continue with
    the next instruction.
    - A letter indicates whether to turn 90 degrees clockwise (R) or
    counterclockwise (L). Turning happens in-place; it does not change your
    current tile.

    So, a path like 10R5 means "go forward 10 tiles, then turn clockwise 90
    degrees, then go forward 5 tiles".

    You begin the path in the leftmost open tile of the top row of tiles.
    Initially, you are facing to the right (from the perspective of how the map
    is drawn).

    If a movement instruction would take you off of the map, you wrap around to
    the other side of the board. In other words, if your next tile is off of
    the board, you should instead look in the direction opposite of your
    current facing as far as you can until you find the opposite edge of the
    board, then reappear there.

    For example, if you are at A and facing to the right, the tile in front of
    you is marked B; if you are at C and facing down, the tile in front of you
    is marked D:
            ...#
            .#..
            #...
            ....
    ...#.D.....#
    ........#...
    B.#....#...A
    .....C....#.
            ...#....
            .....#..
            .#......
            ......#.

    It is possible for the next tile (after wrapping around) to be a wall; this
    still counts as there being a wall in front of you, and so movement stops
    before you actually wrap to the other side of the board.

    By drawing the last facing you had with an arrow on each tile you visit,
    the full path taken by the above example looks like this:
            >>v#
            .#v.
            #.v.
            ..v.
    ...#...v..v#
    >>>v...>#.>>
    ..#v...#....
    ...>>>>v..#.
            ...#....
            .....#..
            .#......
            ......#.

    To finish providing the password to this strange input device, you need to
    determine numbers for your final row, column, and facing as your final
    position appears from the perspective of the original map. Rows start from
    1 at the top and count downward; columns start from 1 at the left and count
    rightward. (In the above example, row 1, column 1 refers to the empty space
    with no tile on it in the top-left corner.) Facing is 0 for right (>), 1
    for down (v), 2 for left (<), and 3 for up (^). The final password is
    the sum of 1000 times the row, 4 times the column, and the facing.

    In the above example, the final row is 6, the final column is 8, and the
    final facing is 0. So, the final password is 1000 * 6 + 4 * 8 + 0: 6032.

    Follow the path given in the monkeys' notes. What is the final password?

    --- Part Two ---
    As you reach the force field, you think you hear some Elves in the
    distance. Perhaps they've already arrived?

    You approach the strange input device, but it isn't quite what the monkeys
    drew in their notes. Instead, you are met with a large cube; each of its
    six faces is a square of 50x50 tiles.

    To be fair, the monkeys' map does have six 50x50 regions on it. If you were
    to carefully fold the map, you should be able to shape it into a cube!

    In the example above, the six (smaller, 4x4) faces of the cube are:
            1111
            1111
            1111
            1111
    222233334444
    222233334444
    222233334444
    222233334444
            55556666
            55556666
            55556666
            55556666

    You still start in the same position and with the same facing as before,
    but the wrapping rules are different. Now, if you would walk off the board,
    you instead proceed around the cube. From the perspective of the map, this
    can look a little strange. In the above example, if you are at A and move
    to the right, you would arrive at B facing down; if you are at C and move
    down, you would arrive at D facing up:
            ...#
            .#..
            #...
            ....
    ...#.......#
    ........#..A
    ..#....#....
    .D........#.
            ...#..B.
            .....#..
            .#......
            ..C...#.

    Walls still block your path, even if they are on a different face of the
    cube. If you are at E facing up, your movement is blocked by the wall
    marked by the arrow:
            ...#
            .#..
         -->#...
            ....
    ...#..E....#
    ........#...
    ..#....#....
    ..........#.
            ...#....
            .....#..
            .#......
            ......#.

    Using the same method of drawing the last facing you had with an arrow on
    each tile you visit, the full path taken by the above example now looks
    like this:
            >>v#
            .#v.
            #.v.
            ..v.
    ...#..^...v#
    .>>>>>^.#.>>
    .^#....#....
    .^........#.
            ...#..v.
            .....#v.
            .#v<<<<.
            ..v...#.

    The final password is still calculated from your final position and facing
    from the perspective of the map. In this example, the final row is 5, the
    final column is 7, and the final facing is 3, so the final password is 1000
    * 5 + 4 * 7 + 3 = 5031.

    Fold the map into a cube, then follow the path given in the monkeys' notes.
    What is the final password? */
}

impl Puzzle for Day22 {
    fn part_one(&self, data: &'static str) -> String {
        let lines = data.lines().filter(|l| !l.is_empty()).collect_vec();
        let (s, b) = lines.split_last().unwrap();

        let (steps, board) = (Steps::from(s), BoardMap::from(b));
        let (start_pos, start_dir) = start_params(&board);

        let (end_pos, end_dir) = follow_steps(steps, board, start_pos, start_dir, simple_wrap);
        password(end_pos, end_dir).to_string()
    }

    fn part_two(&self, data: &'static str) -> String {
        let lines = data.lines().filter(|l| !l.is_empty()).collect_vec();
        let (s, b) = lines.split_last().unwrap();

        let (steps, board) = (Steps::from(s), BoardMap::from(b));
        let (start_pos, start_dir) = start_params(&board);

        let (end_pos, end_dir) = follow_steps(steps, board, start_pos, start_dir, magic_wrap);
        // assert_eq!(password, 124302)
        password(end_pos, end_dir).to_string()
    }
}

type WrappingFn = fn(&HashMap<P2<i32>, Tile>, P2<i32>, P2<i32>, P2<i32>) -> (P2<i32>, P2<i32>);

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Tile {
    Space,
    Wall,
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '.' => Tile::Space,
            '#' => Tile::Wall,
            _ => Tile::Space,
        }
    }
}

struct BoardMap;
impl BoardMap {
    fn from(data: &[&str]) -> HashMap<P2<i32>, Tile> {
        let mut board = HashMap::new();
        for (y, line) in data.iter().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let (x, y) = (x as i32 + 1, y as i32 + 1);
                match c {
                    '.' => board.insert(P2 { x, y }, Tile::Space),
                    '#' => board.insert(P2 { x, y }, Tile::Wall),
                    _ => continue,
                };
            }
        }
        board
    }
}

#[derive(Debug)]
enum Step {
    Move(i32),
    TurnLeft,
    TurnRight,
}
struct Steps;
impl Steps {
    fn from(data: &str) -> Vec<Step> {
        let result = many1(alt((
            i32.map(Step::Move),
            alt((
                char('L').map(|_| Step::TurnLeft),
                char('R').map(|_| Step::TurnRight),
            )),
        )))(data) as IResult<&str, Vec<Step>>;
        result.unwrap().1
    }
}

fn start_params(board: &HashMap<P2<i32>, Tile>) -> (P2<i32>, P2<i32>) {
    let x_start = board
        .iter()
        .filter(|(pt, _)| pt.y == 1)
        .map(|(pt, _)| pt.x)
        .min()
        .unwrap();

    (P2::new(x_start, 1), P2::new(1, 0))
}

fn password(pos: P2<i32>, dir: P2<i32>) -> i32 {
    let (column, row) = pos.into();
    let facing = match dir.into() {
        (1, 0) => 0,
        (0, 1) => 1,
        (-1, 0) => 2,
        (0, -1) => 3,
        _ => unreachable!(),
    };
    1000 * row + 4 * column + facing
}

fn simple_wrap(
    board: &HashMap<P2<i32>, Tile>,
    pos: P2<i32>,
    dir: P2<i32>,
    bounds: P2<i32>,
) -> (P2<i32>, P2<i32>) {
    let mut wrapped_pos = match dir.into() {
        (-1, _) => P2::new(bounds.x, pos.y),
        (1, _) => P2::new(1, pos.y),
        (_, 1) => P2::new(pos.x, 1),
        (_, -1) => P2::new(pos.x, bounds.y),
        _ => unreachable!(),
    };

    while board.get(&wrapped_pos).is_none() {
        wrapped_pos += dir;
    }
    if *board.get(&wrapped_pos).unwrap() == Tile::Wall {
        return (pos, dir);
    }
    (wrapped_pos, dir)
}

fn segment(pos: P2<i32>) -> i32 {
    match pos.y {
        ..=50 => match pos.x {
            ..=100 => 1,
            101.. => 2,
        },
        51..=100 => 3,
        101..=150 => match pos.x {
            ..=50 => 4,
            51.. => 5,
        },
        151.. => 6,
    }
}

#[rustfmt::skip]
fn magic_wrap(
    board: &HashMap<P2<i32>, Tile>,
    pos: P2<i32>,
    dir: P2<i32>,
    _: P2<i32>,
) -> (P2<i32>, P2<i32>) {
    let (wrapped_pos, wrapped_dir) = match segment(pos) {
        1 => match dir.into() {
            (-1, _) => ((1, 151 - pos.y), (1, 0)), // 4
            (_, -1) => ((1, 100 + pos.x), (1, 0)), // 6
            _ => unreachable!(),
        },
        2 => match dir.into() {
            (_, -1) => ((pos.x - 100, 200), (0, -1)), // 6
            (_,  1) => ((100, pos.x - 50 ), (-1, 0)), // 3
            (1,  _) => ((100, 151 - pos.y), (-1, 0)), // 5
            _ => unreachable!(),
        },
        3 => match dir.into() {
            (-1, _) => ((pos.y - 50, 101), (0, 1)),   // 4
            ( 1, _) => ((pos.y + 50, 50 ),  (0, -1)), // 2
            _ => unreachable!(),
        },
        4 => match dir.into() {
            (-1, _) => ((51, pos.y - 100), (1, 0)), // 1
            (_, -1) => ((51, pos.x + 50 ), (1, 0)), // 3
            _ => unreachable!(),
        },
        5 => match dir.into() {
            (1, _) => ((150, 151 - pos.y), (-1, 0)), // 2
            (_, 1) => ((50,  pos.x + 100), (-1, 0)), // 6
            _ => unreachable!(),
        },
        6 => match dir.into() {
            (-1, _) => ((pos.y - 100, 1  ), (0,  1)), // 1
            ( 1, _) => ((pos.y - 100, 150), (0, -1)), // 5
            (_,  1) => ((pos.x + 100, 1  ), (0,  1)), // 2
            _ => unreachable!(),
        },
        _ => unreachable!(),
    };
    let (wrapped_pos, wrapped_dir) = (wrapped_pos.into(), wrapped_dir.into());

    if *board.get(&wrapped_pos).unwrap() == Tile::Wall {
        return (pos, dir);
    }
    (wrapped_pos, wrapped_dir)
}

fn follow_steps(
    steps: Vec<Step>,
    board: HashMap<P2<i32>, Tile>,
    mut pos: P2<i32>,
    mut dir: P2<i32>,
    wrapping_fn: WrappingFn,
) -> (P2<i32>, P2<i32>) {
    use {Step::*, Tile::*};

    let bounds = P2 {
        x: board.keys().map(|pt| pt.x).max().unwrap(),
        y: board.keys().map(|pt| pt.y).max().unwrap(),
    };

    for step in steps {
        match step {
            Move(mut val) => {
                while val > 0 {
                    if let Some(&next_tile) = board.get(&(pos + dir)) {
                        if next_tile == Wall {
                            break;
                        }
                        pos += dir
                    } else {
                        (pos, dir) = wrapping_fn(&board, pos, dir, bounds); // after changing dir - rotation change!
                    }
                    val -= 1;
                }
            }
            #[rustfmt::skip]
            TurnLeft => {
                dir = match dir.into() {
                    // should be dependent on previous dir state as well
                    (-1,  0) => P2::new( 0,  1),
                    ( 1,  0) => P2::new( 0, -1),
                    ( 0, -1) => P2::new(-1,  0),
                    ( 0,  1) => P2::new( 1,  0),
                    _ => unreachable!(),
                };
            }
            #[rustfmt::skip]
            TurnRight => {
                dir = match dir.into() {
                    (-1,  0) => P2::new( 0, -1),
                    ( 1,  0) => P2::new( 0,  1),
                    ( 0, -1) => P2::new( 1,  0),
                    ( 0,  1) => P2::new(-1,  0),
                    _ => unreachable!(),
                };
            }
        }
    }
    (pos, dir)
}
