extern crate gcc;

fn main() {
    gcc::compile_library("libnsexec.a", &["src/nsexec.c"]);
}

