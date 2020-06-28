pub trait Compile {
    type Args;

    fn compile(self, args: Self::Args);
}
