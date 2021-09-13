#![feature(string_remove_matches)]

use std::fmt::Debug;
use std::str::FromStr;
use std::io::{stdout, Write, stdin};
use std::collections::HashMap;
use std::fs::File;
use rand::thread_rng;
use rand::Rng;
pub fn compress<T: Copy + Debug>(board: Vec<Vec<T>>) -> String {
    let mut vec: Vec<T> = vec![];
    for i in 0..board.len() {
        for j in 0..board[i].len() {
            vec.push(board[i][j]);
        }
    }
    format!("{:?}", vec)
}
pub trait Reversed {
    fn reversed(&self) -> Self;
}
impl Reversed for Vec<String> {
    fn reversed(&self) -> Self {
        let mut reversed_vec = self.clone();
        reversed_vec.reverse();
        reversed_vec
    }
}
pub trait GameBoard {
    type Position;
    type Player;
    fn available_positions(&self) -> Vec<Self::Position>;
    fn check_winner(&mut self) -> Option<Self::Player>;
    fn give_reward(&mut self);
    fn reset(&mut self);
    fn update(&mut self, play: Self::Position) -> Result<(), ()>;
}

pub trait AiFunctions {
    type Position;
    type Board;
    type Player;
    fn reset(&mut self);
    fn choose_action(&mut self, available_positions: Vec<Self::Position>, current_board: Self::Board, symbol: Self::Player) -> Self::Position;
    fn feed_reward(&mut self, reward: f32);
}
pub fn input<T: FromStr>(text: &str) -> T where <T as FromStr>::Err: Debug{
    let mut input = String::new();
    print!("{}", text);
    stdout().flush();
    stdin().read_line(&mut input);
    input.remove_matches("\n");
    let mut x: T = input.parse().expect("Failed to Parse X");
    x
}

