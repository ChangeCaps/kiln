mod bindings;
mod entry_points;
mod path;
mod rust_type;
mod span;
mod texture;
mod vertex;
mod wgsl;

#[proc_macro_error::proc_macro_error]
#[proc_macro]
pub fn wgsl(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    wgsl::wgsl(input)
}

#[proc_macro_error::proc_macro_error]
#[proc_macro_derive(Vertex, attributes(vertex))]
pub fn derive_vertex(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    vertex::derive_vertex(input)
}
