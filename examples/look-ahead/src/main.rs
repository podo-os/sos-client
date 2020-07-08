mod imp;
mod output;

fn main() {
    const ROOT: &str = "../data";

    let root = std::path::PathBuf::from(ROOT);

    let mut root = ::sos_client::SourceRoot::<crate::output::Output, _>::load(
        root,
        crate::imp::SimpleSourceImpl,
    )
    .unwrap();

    let output = root.find("hello.txt").unwrap();
    dbg!(output);

    let output = root.find("ghost.txt").unwrap();
    dbg!(output);

    root.save().unwrap();
}
