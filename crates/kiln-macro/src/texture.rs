use naga::StorageFormat;
use syn::parse_quote;

use crate::path::kiln_path;

pub fn rust_storage_format(format: &StorageFormat) -> syn::Type {
    let kiln_path = kiln_path();

    match format {
        StorageFormat::R8Unorm => parse_quote!(#kiln_path::Rgba8Unorm),
        _ => unreachable!(),
    }
}
