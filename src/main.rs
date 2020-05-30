/*
 * Author: Aseem Lalfakawma
 * Website: https://github.com/alalfakawma
 * License: MIT
 */

extern crate ncurses;

// use std::io::stdin;
// use std::io::prelude::*;
use ncurses::*;

fn main() {
    let mut todos: Vec<Todo> = Vec::new();
    let mut cur_index: i16 = 0;

    add_todo("Do Something", &mut todos); // Test

    while cur_index != -1 {
        initscr();
        // Lists the todos
        for (i, todo) in todos.iter().enumerate() {
            // println!("#{0} {1}", (i + 1), todo.show());
            printw(&todo.show(i));
        }
        refresh();
        // Listens for key
        listen_key(&mut cur_index, todos.len() as i32);
        // print!("\x1B[2J");
        endwin();
    }
}

struct Todo {
    todo: String,
    done: bool
}

impl Todo {
    pub fn show(&self, i: usize) -> String {
        let done = if self.done { "[x] ".to_string() } else { "[ ] ".to_string() };

        return format!("#{} ", i + 1) + &done + &self.todo;
    }
}

fn add_todo(todo: &str, todos: &mut Vec<Todo>) {
    todos.push(Todo { todo: todo.to_string(), done: false });
}

fn listen_key(cur_index: &mut i16, max: i32) {
    enum KEY {
        J = 106,
        K = 107,
        Q = 113
    }

    let k: i32 = getch();

    if k == KEY::J as i32 {
        // Down
        *cur_index += 1;
    } else if k == KEY::K as i32 {
        // Up
        *cur_index -= 1;
    } else if k == KEY::Q as i32 {
        // Quit
        *cur_index = -1;
    }
}
