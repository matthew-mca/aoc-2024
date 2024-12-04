use std::fs;
use regex::Regex;

// I am going to use a 1D data structure to store the grid, since tbh I am not
// bothered to try and debug through a bunch of index combos to find a char.
struct WordSearch {
    grid: Vec<char>,
    width: usize,
    height: usize,
}

impl WordSearch {
    fn get(&self, row: usize, column: usize) -> Option<char> {
        if row >= self.height || column >= self.width {
            return None;
        }
        Some(self.grid[(self.width * row) + column])
    }

    fn get_row(&self, index: usize) -> String {
        let mut chars: Vec<char> = Vec::new();
        for i in 0..self.width {
            chars.push(self.get(index, i).unwrap())
        }
        let result: String = chars.into_iter().collect();
        result
    }

    fn get_column(&self, index: usize) -> String {
        let mut chars: Vec<char> = Vec::new();
        for i in 0..self.height {
            chars.push(self.get(i, index).unwrap())
        }
        let result: String = chars.into_iter().collect();
        result
    }

    fn get_all_diagonals(&self) -> Vec<String> {
        let mut diagonals: Vec<String> = Vec::new();
        // Down-right
        for i in 0..self.height {
            for j in 0..self.width {
                let chars = [
                    self.get(i,j),
                    self.get(i+1,j+1),
                    self.get(i+2,j+2),
                    self.get(i+3,j+3),
                ];
                if chars.iter().all(|x| !x.is_none()) {
                    diagonals.push(chars.into_iter().map(|x| x.unwrap()).collect())
                }
            }
        }
        // Down-left
        for i in 0..self.height {
            for j in 3..self.width {
                let chars = [
                    self.get(i,j),
                    self.get(i+1,j-1),
                    self.get(i+2,j-2),
                    self.get(i+3,j-3),
                ];
                if chars.iter().all(|x| !x.is_none()) {
                    diagonals.push(chars.into_iter().map(|x| x.unwrap()).collect())
                }
            }
        }
        // Up-right
        for i in 3..self.height {
            for j in 0..self.width {
                let chars = [
                    self.get(i,j),
                    self.get(i-1,j+1),
                    self.get(i-2,j+2),
                    self.get(i-3,j+3),
                ];
                if chars.iter().all(|x| !x.is_none()) {
                    diagonals.push(chars.into_iter().map(|x| x.unwrap()).collect())
                }
            }
        }
        // Up-left
        for i in 3..self.height {
            for j in 3..self.width {
                let chars = [
                    self.get(i,j),
                    self.get(i-1,j-1),
                    self.get(i-2,j-2),
                    self.get(i-3,j-3),
                ];
                if chars.iter().all(|x| !x.is_none()) {
                    diagonals.push(chars.into_iter().map(|x| x.unwrap()).collect())
                }
            }
        }
        diagonals
    }

    fn get_3_by_3_grids(&self) -> Vec<WordSearch> {
        let mut sub_grids: Vec<WordSearch> = Vec::new();
        for i in 0..self.height - 2 {
            for j in 0..self.width - 2 {
                let mut chars: Vec<char> = vec![];
                // I can't believe I manually wrote these out...
                // I know this is horrific, but damn am I already tired.
                chars.push(self.get(i,j).unwrap());
                chars.push(self.get(i,j+1).unwrap());
                chars.push(self.get(i,j+2).unwrap());
                chars.push(self.get(i+1,j).unwrap());
                chars.push(self.get(i+1,j+1).unwrap());
                chars.push(self.get(i+1,j+2).unwrap());
                chars.push(self.get(i+2,j).unwrap());
                chars.push(self.get(i+1,j+1).unwrap());
                chars.push(self.get(i+2,j+2).unwrap());

                sub_grids.push(WordSearch{
                    grid: chars,
                    height: 3,
                    width: 3,
                })
            }
        }
        sub_grids
    }

    fn is_xmas_cross(&self) -> bool {
        if self.width != 3 || self.height != 3 {
            return false;
        }

        let mut line_1: Vec<char> = vec![];
        for i in 0..3 {
            line_1.push(self.get(i, i).unwrap());
        }
        let line_1_str: String = line_1.into_iter().collect();
        if line_1_str != "MAS" && line_1_str != "SAM" {
            return false;
        }

        let mut line_2: Vec<char> = vec![];
        for i in 0..3 {
            line_2.push(self.get(2-i, i).unwrap());
        }
        let line_2_str: String = line_2.into_iter().collect();
        if line_2_str != "MAS" && line_2_str != "SAM" {
            return false;
        }
        true
    }
}

fn main() {
    let challenge_input = fs::read_to_string("input.txt").unwrap();
    let mut wordsearch_grid: Vec<char> = Vec::new();
    for char in challenge_input.chars() {
        if char == '\n' {
            continue;
        }
        wordsearch_grid.push(char);
    }
    let wordsearch_width = challenge_input.find("\n").unwrap();
    let wordsearch_height = wordsearch_grid.len() / wordsearch_width;
    let elf_wordsearch = WordSearch {
        grid: wordsearch_grid,
        width: wordsearch_width,
        height: wordsearch_height,
    };

    let mut xmas_word_count = 0;
    let forward_xmas = Regex::new("XMAS").unwrap();
    let backward_xmas = Regex::new("SAMX").unwrap();

    // Find left and right instances of XMAS
    for i in 0..elf_wordsearch.height {
        let current_row = elf_wordsearch.get_row(i);
        xmas_word_count += forward_xmas.find_iter(current_row.as_str()).into_iter().count();
        xmas_word_count += backward_xmas.find_iter(current_row.as_str()).into_iter().count();
    }

    // Find up and down instances of XMAS
    for i in 0..elf_wordsearch.width {
        let current_row = elf_wordsearch.get_column(i);
        xmas_word_count += forward_xmas.find_iter(current_row.as_str()).into_iter().count();
        xmas_word_count += backward_xmas.find_iter(current_row.as_str()).into_iter().count();
    }

    // We only need to search once here since get_all_diagonals() includes diagonals in every
    // direction
    for str in elf_wordsearch.get_all_diagonals() {
        xmas_word_count += forward_xmas.find_iter(str.as_str()).into_iter().count();
    }

    println!("{}", xmas_word_count);

    let mut xmas_cross_count = 0;

    let all_sub_grids: Vec<WordSearch> = elf_wordsearch.get_3_by_3_grids();
    for wordsearch in all_sub_grids {
        if wordsearch.is_xmas_cross(){
            xmas_cross_count += 1;
        }
    }

    println!("{}", xmas_cross_count);
}
