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
    let mut screen: i8 = SCREEN::MAIN as i8; // Set the screen

    let bw: WINDOW = initscr();

    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE); // Don't show the terminal cursor
    keypad(bw, true);

    while cur_index != -1 {
        addstr("---------------\n");
        addstr("---TODO LIST---\n");
        addstr("---------------\n");
        addstr("a: Add, d: Delete, x: Done/Undone, j: DOWN, k: UP, q: Quit\n\n");

        if todos.len() == 0 {
            addstr("--- **NOTHING TODO** ---\n");
        }

        if screen == SCREEN::MAIN as i8 {
            list_todos(&todos, cur_index);
            // Listens for key
            listen_key(&mut cur_index, todos.len() as i32, &mut screen, &mut todos);
        } else if screen == SCREEN::ADD as i8 {
            show_add_input(&mut todos, &mut screen, &bw);
        }
        refresh();
        clear();
    }
    endwin();
}

enum SCREEN {
    MAIN,
    ADD,
}

struct Todo {
    todo: String,
    done: bool,
}

impl Todo {
    pub fn show(&self, i: usize, cur_index: i32) -> String {
        let done = if self.done { "[x] " } else { "[ ] " };
        let cursor = if i == cur_index as usize { "* " } else { "  " };

        return cursor.to_string()
            + &format!("#{} ", i + 1)
            + &done.to_string()
            + &self.todo
            + "\n";
    }
}

fn add_todo(todo: &str, todos: &mut Vec<Todo>) {
    todos.push(Todo {
        todo: todo.to_string(),
        done: false,
    });
}

fn listen_key(cur_index: &mut i32, max: i32, screen: &mut i8, mut todos: &mut Vec<Todo>) {
    enum KEY {
        J = 106,
        K = 107,
        Q = 113,
        X = 120,
        A = 97,
        D = 100,
    }

    noecho();
    let k: i32 = getch();
    echo();

    if k == KEY::J as i32 {
        // Down
        *cur_index += 1;
        if cur_index >= &mut (max - 1) {
            if max != 0 {
                *cur_index = max - 1;
            }
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
        delete_todo(*cur_index, &mut todos);
    }
}

fn show_add_input(mut todos: &mut Vec<Todo>, screen: &mut i8, window: &WINDOW) {
    let mut todo: String = String::new();
    addstr("Enter Todo: ");

    curs_set(CURSOR_VISIBILITY::CURSOR_VISIBLE); // Show the terminal cursor
    let mut c: i32 = 97;
    while c != '\n' as i32 {
        noecho();
        c = getch();

        if c != '\n' as i32 {
            if c == 127 {
                if !todo.is_empty() {
                    mvdelch(getcury(*window), getcurx(*window) - 1);
                    todo.pop();
                }
            } else {
                todo.push(char::from(c as u8));
                addch(c as u64);
            }
        }
    }

    if !todo.is_empty() {
        add_todo(&todo, &mut todos);
    }

    *screen = SCREEN::MAIN as i8;

    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE); // Don't show the terminal cursor
}

fn list_todos(todos: &Vec<Todo>, cur_index: i32) {
    // Lists the todos
    for (i, todo) in todos.iter().enumerate() {
        addstr(&todo.show(i, cur_index));
    }
}

fn do_undo(cur_index: i32, todos: &mut Vec<Todo>) {
    todos[cur_index as usize].done = !todos[cur_index as usize].done;
}

fn delete_todo(cur_index: i32, todos: &mut Vec<Todo>) {
    todos.remove(cur_index as usize);
}
