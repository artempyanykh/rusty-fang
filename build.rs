use std::path::PathBuf;

fn main() {
    let dir: PathBuf = ["grammar", "src"].iter().collect();

    cc::Build::new()
        .include(&dir)
        .file(dir.join("parser.c"))
        .flag("-Wno-unused-but-set-variable") // this warning comes from tree-sitter generate code
        .compile("grammar");
}
