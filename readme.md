## Todo List (ncurses)
##### Author: [Aseem Lalfakawma](https://github.com/alalfakawma)

#### Screenshot
![Todo List screenshot](https://i.imgur.com/UQO5t6U.png)

#### Note
This app creates a `.todos.json` file in the folder that you run it to persist the data. It also will automatically add the `.todos.json` file to `.gitignore` file in case it exists.

#### Keybindings
- _a_ - Add
- _x_ - Done/Undone
- _e, enter_ - Edit
- _d_ - Delete
- _j, k, arrow up, arrow down_ - Up or down
- _q_ - Quit

#### Install
Make sure you have Cargo installed, if you install the rust package, I think it should come with it. Mine did (on Manjaro system btw).
Run the following code to install the package as a binary

```bash
cargo install todo_list
```

#### Clone and check it out

```bash
git clone https://github.com/alalfakawma/todo_list_rust.git

cd todo_list_rust/

cargo run
```
