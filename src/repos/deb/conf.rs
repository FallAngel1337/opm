// Place where the files will be temporary placed
pub const OUTDIR: &'static str = "./tmp/.install";

pub struct DebConfig<'a> {
    pub repos: Vec<&'a str>
}