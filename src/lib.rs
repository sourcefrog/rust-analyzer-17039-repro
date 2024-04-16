//! Reproduction of rust-analyzer #17039.

pub struct Struct;
pub enum Enum {
    Variant(Struct),
}

pub fn f(args: &[Enum]) -> Option<(&Struct, &Struct)> {
    use itertools::Itertools;
    let arg_iter = args.iter();
    if let Some((Enum::Variant(thing), Enum::Variant(thing2))) = arg_iter.collect_tuple() {
        return Some((thing, thing2));
    }
    None
}
