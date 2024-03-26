fn display(board:[[i8; 3]; 3]) -> () {
    let board_lines = board.map(|line| {
                            line.map(|el| {
                                el.to_string()
                            }).join(" | ")
                    });
    // println!("{:?}", board_lines);
    println!("{}", board_lines.join("\n"));
}

fn main() {
    let board: [[i8; 3]; 3]= [[0; 3]; 3];

    display(board);
} 
