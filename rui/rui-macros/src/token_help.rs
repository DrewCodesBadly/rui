use darling::FromMeta;
use kdl::KdlEntry;
use proc_macro2::Span;
use quote::ToTokens;
use quote::quote;
use syn::{Expr, Ident, Type};

use crate::UnimplementedError;

pub enum VariableType {
    SynType(Type),
    SynIdent(Ident),
}

impl From<&String> for VariableType {
    fn from(s: &String) -> Self {
        if let Ok(t) = Type::from_string(s) {
            Self::SynType(t)
        } else {
            Self::SynIdent(Ident::new(s, Span::call_site()))
        }
    }
}

pub struct VariableInfo {
    pub var_type: VariableType,
    pub name: Ident,
    pub default: Expr,
}

impl ToTokens for VariableInfo {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let name = &self.name;
        match &self.var_type {
            VariableType::SynType(t) => {
                tokens.extend(quote!(#name: #t,));
            }
            VariableType::SynIdent(i) => {
                tokens.extend(quote!(#name: #i,));
            }
        }
    }
}

pub struct VariableDefaultInfo {
    pub name: Ident,
    pub default: Expr,
}

impl ToTokens for VariableDefaultInfo {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let name = &self.name;
        let def = &self.default;
        tokens.extend(quote!(#name: #def,));
    }
}

pub fn entry_to_expression(entry: &KdlEntry) -> miette::Result<Option<Expr>> {
    if entry.ty().filter(|i| i.to_string() == "expr").is_some() {
        // TODO: parse expression for errors, if at all possible
        if let Some(s) = entry.value().as_string() {
            Ok(Some(
                Expr::from_string(&s).map_err(|_| UnimplementedError {})?,
            ))
        } else {
            Ok(None)
        }
    } else {
        Ok(match entry.value() {
            // All of these should be infallible.
            kdl::KdlValue::String(s) => {
                Some(Expr::from_string(&format!("String::from({})", s)).unwrap())
            }
            kdl::KdlValue::Integer(i) => Some(Expr::from_string(&format!("{}", i)).unwrap()),
            kdl::KdlValue::Float(f) => Some(Expr::from_string(&format!("{}", f)).unwrap()),
            // How is this fallible?
            kdl::KdlValue::Bool(b) => Some(Expr::from_bool(*b).unwrap()),
            kdl::KdlValue::Null => None,
        })
    }
}
