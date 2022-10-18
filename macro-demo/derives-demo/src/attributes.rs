// my-derives/attributes.rs
use bae::FromAttributes;

#[derive(Default, FromAttributes)]
pub struct OicColumn {
    pub name: Option<syn::Lit>,
    pub age: Option<syn::Lit>,
    pub comment: Option<syn::Lit>,
}
