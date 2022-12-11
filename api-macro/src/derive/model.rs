use darling::FromDeriveInput;
use proc_macro::TokenStream;
use syn::{Ident, Generics, Attribute};
use quote::quote;

/*

#[derive(Model)]
#[model(node_suffix = "u")]
pub struct User {
    pub id: Id<Self>,
    pub name: String,
}

// generates:
impl Model for User {
    const NODE_SUFFIX = "u";
    fn model_id(&self) -> &Id<Self> {
        &self.id
    }
}

impl From<Id<User>> for Id<Node> {
    fn from(i: Id<User>) -> Self {
        Id::<Node>::new_from_id(i.id, i.node_suffix)
    }
}

impl<'a> From<&'a Id<User>> for &'a Id<Node> {
    fn from(i: &'a Id<User>) -> Self {
        Id::<Node>::as_ref(i)
    }
}

 */

#[derive(FromDeriveInput)]
#[darling(attributes(model), forward_attrs(allow, doc, cfg))]
pub struct MetaModel {
    pub ident: Ident,
    pub generics: Generics,
    pub attrs: Vec<Attribute>,
    
    pub node_suffix: String,
}

impl MetaModel {
    pub fn expand(&self) -> TokenStream {
        let ident = &self.ident;
        let node_suffix = self.node_suffix.clone();

        let expanded = quote! {
            impl crate::model::Model for #ident {
                const NODE_SUFFIX: &'static str = #node_suffix;
                fn model_id(&self) -> &crate::types::id::Id<Self> {
                    &self.id
                }
            }

            impl ::std::convert::From<crate::types::id::Id<#ident>> for crate::types::id::Id<crate::schema::Node> {
                fn from(i: crate::types::id::Id<#ident>) -> Self {
                    crate::types::id::Id::<crate::schema::Node>::new_from_id(i.id, i.node_suffix)
                }
            }
            
            impl<'a> ::std::convert::From<&'a crate::types::id::Id<#ident>> for &'a crate::types::id::Id<crate::schema::Node> {
                fn from(i: &'a crate::types::id::Id<#ident>) -> Self {
                    crate::types::id::Id::<crate::schema::Node>::as_ref(i)
                }
            }
        };

        expanded.into()
    }
}
