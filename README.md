# Rust

Log for Learning Basic Rust Syntax :

24/12/2022 :-

1. ```rust
   fn main(){
      println!("Hello, World!")
   }
   ```
2. " ! means that you're calling a macro instead of a normal function and that macros don't always follow the same rules as functions."
3. **Cargo**
4. ```powershell
   cargo new hello_cargo <# without git if git already exist #>
   cargo new --vsc=git hello cargo <# with git eventhough git is already exist #>
   ```

29/12/2022:-

Ownership model

* Each value in Rust has a variable that's called its owner.
* There can only be one owner at a time.
* When the owner goes out of scope, the value will be dropped.

Offline Document

`rustup doc`
