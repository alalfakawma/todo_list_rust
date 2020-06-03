/*
 * Author: Aseem Lalfakawma
 * Website: https://github.com/alalfakawma
 * License: MIT
 */

extern crate ncurses;
extern crate toml;

use ncurses::*;
use toml::Value;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn main() {
    let mut todos: Vec<Todo> = Vec::new();
    let mut cur_index: i32 = 0;
    let mut screen: i8 = SCREEN::MAIN as i8; // Set the screen

    println!("{}", toml("version"));
    return;
    let bw: WINDOW = initscr();

    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE); // Don't show the terminal cursor
    keypad(bw, true);

    while cur_index != -1 {
        addstr("---------------\n");
        addstr("---TODO LIST---\n");
        addstr("---------------\n");
        addstr("a: Add, e: Edit, d: Delete, x: Done/Undone, j: DOWN, k: UP, q: Quit\n\n");

        if todos.is_empty() && screen == SCREEN::MAIN as i8 {
            addstr("--- **NOTHING TODO** ---\n");
        }

        if screen == SCREEN::MAIN as i8 {
            list_todos(&todos, cur_index);
            // Listens for key
            listen_key(&mut cur_index, todos.len() as i32, &mut screen, &mut todos);
        } else if screen == SCREEN::ADD as i8 {
            show_add_input(&mut todos, &mut screen, bw, -1);
        } else if screen == SCREEN::EDIT as i8 {
            show_add_input(&mut todos, &mut screen, bw, cur_index);
        }
        refresh();
        clear();
    }
    endwin();
}

enum SCREEN {
    MAIN,
    ADD,
    EDIT
}

struct Todo {
    todo: String,
    done: bool,
}

impl Todo {
    pub fn show(&self, i: usize, cur_index: i32) -> String {
        let done = if self.done { "[x] " } else { "[ ] " };
        let cursor = if i == cur_index as usize { "* " } else { "  " };

        cursor.to_string() + &format!("#{} ", i + 1) + &done.to_string() + &self.todo + "\n"
    }
}

fn add_todo(todo: &str, todos: &mut Vec<Todo>) {
    todos.push(Todo {
        todo: todo.to_string(),
        done: false,
    });
}

fn listen_key(mut cur_index: &mut i32, max: i32, screen: &mut i8, mut todos: &mut Vec<Todo>) {
    enum KEY {
        J = 106,
        K = 107,
        Q = 113,
        X = 120,
        A = 97,
        D = 100,
        E = 101
    }

    noecho();
    let k: i32 = getch();
    echo();

    if k == KEY::J as i32 {
        // Down
        *cur_index += 1;
        if cur_index >= &mut (max - 1) && max != 0 {
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
    } else if k == KEY::A as i32 {
        // Add
        *screen = SCREEN::ADD as i8;
    } else if k == KEY::X as i32 {
        // Do/Undo
        do_undo(*cur_index, &mut todos);
    } else if k == KEY::D as i32 {
        delete_todo(&mut cur_index, &mut todos);
    } else if k == KEY::E as i32 {
        if !todos.is_empty() {
            *screen = SCREEN::EDIT as i8;
        }
    }
}

fn show_add_input(mut todos: &mut Vec<Todo>, screen: &mut i8, window: WINDOW, mut cur_index: i32) {
    let mut todo: String = if cur_index >= 0 { (*todos[cur_index as usize].todo).into() } else { String::new() };
    // let mut todo: String = String::new();
    addstr("Enter Todo: ");

    if cur_index >= 0 {
        addstr(&todo);
    }

    curs_set(CURSOR_VISIBILITY::CURSOR_VISIBLE); // Show the terminal cursor
    let mut c: i32 = 97;
    while c != '\n' as i32 {
        noecho();
        c = getch();

        if c != '\n' as i32 {
            if c == 127 {
                if !todo.is_empty() {
                    mvdelch(getcury(window), getcurx(window) - 1);
                    todo.pop();
                }
            } else {
                todo.push(char::from(c as u8));
                addch((c as u32).into());
            }
        }
    }

    if !todo.is_empty() && cur_index == -1 {
        add_todo(&todo, &mut todos);
    } else if !todo.is_empty() && cur_index >= 0 {
        update_todo(&todo, &mut todos, cur_index);
    } else if todo.is_empty() && cur_index >= 0 {
        delete_todo(&mut cur_index, &mut todos);
    }

    *screen = SCREEN::MAIN as i8;

    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE); // Don't show the terminal cursor
}

fn list_todos(todos: &[Todo], cur_index: i32) {
    // Lists the todos
    for (i, todo) in todos.iter().enumerate() {
        addstr(&todo.show(i, cur_index));
    }
}

fn do_undo(cur_index: i32, todos: &mut Vec<Todo>) {
    todos[cur_index as usize].done = !todos[cur_index as usize].done;
}

fn delete_todo(cur_index: &mut i32, todos: &mut Vec<Todo>) {
    let len = todos.len() as i32;
    todos.remove(*cur_index as usize);
    if *cur_index == len - 1 {
        if (*cur_index - 1) <= 0 {
            *cur_index = 0;
        } else {
            *cur_index -= 1;
        }
    }
}

fn update_todo(todo: &str, todos: &mut Vec<Todo>, cur_index: i32) {
    todos[cur_index as usize].todo = todo.into();
}

fn open_file() -> String {
    let file = File::open("Cargo.toml").unwrap();
    let mut reader = BufReader::new(file);
    let mut contents = String::new();
    reader.read_to_string(&mut contents);

    contents;
}

fn toml(key: &str) -> String {
    let file_contents = open_file();
    let mut res: String = String::new();

    for k in file_contents.split('\n') {
        let value = k.parse::<Value>().unwrap();
        if value.as_table().unwrap().contains_key(key) {
            res = value[key].as_str().unwrap().into();
        }
    }

    res
}
