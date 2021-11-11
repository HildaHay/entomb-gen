use rand::prelude::*;

static LOOKUPTABLE: [i32; 32] = [
    1, 1, 1, 2, 0, 0, 2, 2, 1, 1, 1, 1, 2, 0, 0, 0, 1, 1, 1, 2, 0, 0, 0, 0, 2, 0, 1, 2, 2, 0, 0, 0,
];

fn main() {
    let mut maze: [[bool; 32]; 32] = [[false; 32]; 32];

    for x in 0..31 {
        maze[x][0] = rand::random();
    }

    for y in 1..31 {
        for x in 0..31 {
            maze[x][y] = generate_square(&maze, x as i32, y as i32);
        }
    }

    for y in 0..31 {
        for x in 0..31 {
            if maze[x][y] {
                print!("█");
            } else {
                print!(" ");
            };
        }
        println!();
    }
}

fn generate_square(maze: &[[bool; 32]; 32], x: i32, y: i32) -> bool {
    let mut prev = [false; 5];

    let xx = x as usize;
    let yy = y as usize;

    if (x == 0) {
        prev[0] = true;
        prev[1] = false;
        prev[2] = rand::random();
    } else {
        if (x == 1) {
            prev[0] = false;
        } else {
            prev[0] = maze[xx - 2][yy];
        }
        prev[1] = maze[xx - 1][yy];
        prev[2] = maze[xx - 1][yy - 1];
    }

    prev[3] = maze[xx][yy - 1];

    if (x == 31) {
        prev[4] = rand::random();
    } else {
        prev[4] = maze[xx + 1][yy - 1];
    }

    let mut lookup = 0;
    if prev[0] {
        lookup += 16
    };
    if prev[1] {
        lookup += 8
    };
    if prev[2] {
        lookup += 4
    };
    if prev[3] {
        lookup += 2
    };
    if prev[4] {
        lookup += 1
    };

    if(LOOKUPTABLE[lookup] == 0) {
        false
    } else if (LOOKUPTABLE[lookup] == 1) {
        true
    } else {
        rand::random()
    }
}
