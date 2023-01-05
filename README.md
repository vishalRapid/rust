## Few of the userful commands

- Run project using `cargo `
  `cargo run `

- Build project using cargo
  `cargo build`

- create New project with cargo
  `cargo new <project name>`

- Check project is good for build or not
  `cargo check` (much faster , it doesn't create executables and easy to check if your project is still compiling)

`Instead of saving the result of the build in the same directory as our code, Cargo stores it in the target/debug directory.`

- Build project for release
  `cargo build --release` (build the project in optimal format, This command will create an executable in target/release)
