fn main() {
    let input =
"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    let lines = input.lines();
    let mut matrix = vec![vec![0; lines[0].len()]; lines.len()];
    println!("width {} height {}", lines[0].len(), lines.len());

    println!("{}", input);
}
