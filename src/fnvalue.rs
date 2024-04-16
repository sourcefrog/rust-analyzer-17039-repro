// Copyright 2021-2024 Martin Pool

//! Mutations of replacing a function body with a value of a (hopefully) appropriate type.

use itertools::Itertools;
use syn::{AngleBracketedGenericArguments, GenericArgument, Ident, Path, PathArguments, Type};

#[allow(dead_code)]
/// Match known key-value maps that can be empty or constructed from pair of
/// recursively-generated values.
fn known_map(path: &Path) -> Option<(&Ident, &Type, &Type)> {
    let last = path.segments.last()?;
    if let PathArguments::AngleBracketed(AngleBracketedGenericArguments { args, .. }) =
        &last.arguments
    {
        if let Some((GenericArgument::Type(key_type), GenericArgument::Type(value_type))) =
            args.iter().collect_tuple()
        {
            return Some((&last.ident, key_type, value_type));
        }
    }
    None
}
