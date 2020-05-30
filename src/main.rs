/*
 * Author: Aseem Lalfakawma
 * Website: https://github.com/alalfakawma
 * License: MIT
 */

extern crate ncurses;

use ncurses::*;

fn main() {
    let mut todos: Vec<Todo> = Vec::new();
    let mut cur_index: i32 = 0;
    let mut screen: SCREEN = SCREEN::MAIN; // Set the screen

    add_todo("Do Something", &mut todos); // Test
    add_todo("Do Something", &mut todos); // Test
    add_todo("Do Something", &mut todos); // Test

    initscr();

    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE); // Don't show the terminal cursor

    while cur_index != -1 {
        addstr("---TODO LIST---\n");

        if screen == SCREEN::MAIN {
            list_todos(&todos, cur_index);
        } else if screen == SCREEN::ADD {
            show_add_input();
        }
        refresh();
        // Listens for key
        listen_key(&mut cur_index, todos.len() as i32);
    }
    endwin();
}

enum SCREEN {
    MAIN,
    ADD
}

struct Todo {
    todo: String,
    done: bool
}

impl Todo {
    pub fn show(&self, i: usize, cur_index: i32) -> String {
        let done = if self.done { "[x] " } else { "[ ] " };
        let cursor = if i == cur_index as usize { "* " } else { "  " };

        return cursor.to_string() + &format!("#{} ", i + 1) + &done.to_string() + &self.todo + "\n";
    }
}

fn add_todo(todo: &str, todos: &mut Vec<Todo>) {
    todos.push(Todo { todo: todo.to_string(), done: false });
}

fn listen_key(cur_index: &mut i32, max: i32) {
    enum KEY {
        J = 106,
        K = 107,
        Q = 113
    }

    let k: i32 = getch();

    if k == KEY::J as i32 {
        // Down
        *cur_index += 1;
        if cur_index >= &mut (max - 1) {
            *cur_index = max - 1;
        }
    } else if k == KEY::K as i32 {
        // Up
        *cur_index -= 1;
        if cur_index <= &mut 0 {
            *cur_index = 0;
        }
    } else if k == KEY::Q as i32 {
        // Quit
        *cur_index = -1;
    }

    clear(); // Clear and refresh screen
}

fn show_add_input() {
    addstr("Enter Todo: ");
    getch();
}

fn list_todos(todos: &Vec<Todo>, cur_index: i32) {
    // Lists the todos
    for (i, todo) in todos.iter().enumerate() {
        addstr(&todo.show(i, cur_index));
    }
}
