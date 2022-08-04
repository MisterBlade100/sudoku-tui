use rand::prelude::SliceRandom;
use std::collections::BTreeSet;
use ndarray::prelude::*;
use itertools::Itertools;
use rand::Rng;

enum Event<I> {
    Input(I),
    Tick,
}

fn main() {
    /*
    enable_raw_mode().expect("can run in raw mode");

    let (tx, rx) = mpsc::channel();
        let tick_rate = Duration::from_millis(200);
        thread::spawn(move || {
            let mut last_tick = Instant::now();
            loop {
                let timeout = tick_rate
                    .checked_sub(last_tick.elapsed())
                    .unwrap_or_else(|| Duration::from_secs(0));
    
                if event::poll(timeout).expect("poll works") {
                    if let CEvent::Key(key) = event::read().expect("can read events") {
                        tx.send(Event::Input(key)).expect("can send events");
                    }
                }
    
                if last_tick.elapsed() >= tick_rate {
                    if let Ok(_) = tx.send(Event::Tick) {
                        last_tick = Instant::now();
                    }
                }
            }
        });

        */

    let newgame = Sudoku::gengame(Dificulty::Hard);
    println!("{:?}\n\n{:?}", newgame.game, newgame.solution);
}

struct Sudoku {
    solution: Array2::<u8>,
    game: Array2::<u8>,
}

enum Dificulty {
    Easy,
    Medium,
    Hard,
}

impl Sudoku {
fn gengame(dificulty: Dificulty) -> Sudoku {

    fn gen_solution() -> Option<Array2<u8>> {
        let mut grid = Array2::<u8>::zeros((9, 9));
    
        for r in 0..=8 {
        for c in 0..=8 {
            let mut exeptions: Vec<u8> = Vec::new();
    
            let row_exeptions = grid.row(r).clone();
            let row_exeptions = row_exeptions.iter().unique();
    
            let column_exeptions = grid.column(c).clone();
            let column_exeptions = column_exeptions.iter().unique();
    
            exeptions.extend(row_exeptions);
            exeptions.extend(column_exeptions);
    
            let y0: i32;
            let y: i32;
            let x0: i32;
            let x: i32;
    
            match (r, c) {
                (0..=2, 0..=2) => {y0 = 0; y = 3; x0 = 0; x = 3;}
                (0..=2, 3..=5) => {y0 = 0; y = 3; x0 = 3; x = 6;}
                (0..=2, 6..=8) => {y0 = 0; y = 3; x0 = 6; x = 9;}
    
                (3..=5, 0..=2) => {y0 = 3; y = 6; x0 = 0; x = 3;}
                (3..=5, 3..=5) => {y0 = 3; y = 6; x0 = 3; x = 6;}
                (3..=5, 6..=8) => {y0 = 3; y = 6; x0 = 6; x = 9;}
    
                (6..=8, 0..=2) => {y0 = 6; y = 9; x0 = 0; x = 3;}
                (6..=8, 3..=5) => {y0 = 6; y = 9; x0 = 3; x = 6;}
                (6..=8, 6..=8) => {y0 = 6; y = 9; x0 = 6; x = 9;}
                (_, _) => panic!("Couldn't identify the group"),
            }
    
            let group_exeptions = grid.slice(s![y0..y, x0..x])
                .iter()
                .copied()
                .collect::<Array1<_>>()
                .clone();
    
            let group_exeptions = group_exeptions.iter().unique();
            exeptions.extend(group_exeptions);
    
            let mut base: Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];

            let exeptions = BTreeSet::from_iter(exeptions);
            base.retain(|e| !exeptions.contains(e));
    
            grid[[r, c]] = *base.choose(&mut rand::thread_rng())?
    
        }}
        Some(grid)
    }

    let solution: Array2::<u8>;
    loop {
        match gen_solution() {
            Some(grid) => {solution = grid; break;}
            None => continue,
        };
    }

    let mut game = solution.clone();

    let missing_squares: u8;
    match dificulty {
        Dificulty::Easy => missing_squares = 45,
        Dificulty::Medium => missing_squares = 54,
        Dificulty::Hard => missing_squares = 62,
    }
    for _i in 0..=missing_squares {
        let r = rand::thread_rng().gen_range(0..=8);
        let c = rand::thread_rng().gen_range(0..=8);

        game[[r, c]] = 0;
    }

    Sudoku { solution: solution, game: game }
}

}
