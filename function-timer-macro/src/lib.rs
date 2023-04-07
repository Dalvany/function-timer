//! `time` macro. It can place on any function.
use proc_macro::TokenStream;
use proc_macro2::Span;

use quote::quote;
use syn::fold::Fold;
use syn::parse::{Parse, ParseStream};
use syn::spanned::Spanned;
use syn::token::Impl;
use syn::{
    parse_macro_input, Attribute, Block, Expr, ExprBlock, Ident, ImplItem, ImplItemFn, ItemFn,
    ItemImpl, LitStr, Meta, Stmt, Type,
};

mod custom_keywords {
    syn::custom_keyword!(disable);
}

enum Name {
    Literal(LitStr),
    Ident(Ident),
    Disable(custom_keywords::disable),
}

impl Name {
    fn disable(&self) -> bool {
        matches!(self, Name::Disable(_))
    }

    fn span(&self) -> Span {
        match self {
            Self::Literal(lit) => lit.span(),
            Self::Ident(ident) => ident.span(),
            Self::Disable(tok) => tok.span(),
        }
    }
}

struct MetricName {
    struct_name: Option<String>,
    name: Name,
}

impl MetricName {
    /// Find if there's any override time attribute.
    fn any_time_attribut(attributs: &[Attribute]) -> bool {
        attributs.iter().any(|attr| {
            if let Meta::Path(path) = &attr.meta {
                path.segments.last().map(|i| i.ident.to_string()) == Some("time".to_string())
            } else if let Meta::List(list) = &attr.meta {
                list.path.segments.last().map(|i| i.ident.to_string()) == Some("time".to_string())
            } else {
                false
            }
        })
    }

    fn block_from(&self, block: Block, function_name: String) -> Block {
        let metric_name = match &self.name {
            Name::Literal(lit) => quote!(#lit),
            Name::Ident(ident) => quote!(#ident),
            // Early return the block as it shouldn't change (disable)
            Name::Disable(_) => return block,
        };
        let st = self.struct_name.clone();

        let macro_stmt = if let Some(st) = st {
            quote!(
                let _guard = function_timer::FunctionTimer::new(#metric_name, Some(#st), #function_name);
            )
        } else {
            quote!(
                let _guard = function_timer::FunctionTimer::new(#metric_name, None, #function_name);
            )
        };
        let mut statements: Vec<Stmt> = Vec::with_capacity(2);

        let macro_stmt: Stmt = syn::parse2(macro_stmt).expect("Can't parse token");
        statements.push(macro_stmt);

        statements.push(Stmt::Expr(
            Expr::Block(ExprBlock {
                attrs: vec![],
                label: None,
                block,
            }),
            None,
        ));

        Block {
            brace_token: Default::default(),
            stmts: statements,
        }
    }
}

impl Parse for MetricName {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();
        let name =
            if lookahead.peek(custom_keywords::disable) {
                Name::Disable(input.parse::<custom_keywords::disable>()?)
            } else if lookahead.peek(LitStr) {
                Name::Literal(input.parse()?)
            } else {
                Name::Ident(input.parse().map_err(|error| {
                    syn::Error::new(error.span(), "Expected literal or identifier")
                })?)
            };

        Ok(Self {
            struct_name: None,
            name,
        })
    }
}

impl Fold for MetricName {
    fn fold_impl_item_fn(&mut self, i: ImplItemFn) -> ImplItemFn {
        // If there's a time attribut it override the impl one.
        // This also allow to handle disable
        if Self::any_time_attribut(&i.attrs) {
            return i;
        }

        let mut result = i.clone();
        let block = i.block;
        let name = i.sig.ident.to_string();
        let new_block = self.block_from(block, name);
        result.block = new_block;

        result
    }

    fn fold_item_fn(&mut self, i: ItemFn) -> ItemFn {
        let block = *i.block;
        let name = i.sig.ident.to_string();

        let new_block = self.block_from(block, name);

        ItemFn {
            attrs: i.attrs,
            vis: i.vis,
            sig: i.sig,
            block: Box::new(new_block),
        }
    }
    fn fold_item_impl(&mut self, i: ItemImpl) -> ItemImpl {
        let mut new_items: Vec<ImplItem> = Vec::with_capacity(i.items.len());
        let mut result = i.clone();
        if let Type::Path(p) = *i.self_ty {
            self.struct_name = p.path.segments.last().map(|p| p.ident.to_string());
        }
        for item in i.items {
            if let ImplItem::Fn(method) = item {
                new_items.push(ImplItem::Fn(self.fold_impl_item_fn(method)));
            } else {
                new_items.push(item);
            }
        }
        result.items = new_items;
        result
    }
}

enum ImplOrFn {
    Function(ItemFn),
    ImplStruct(ItemImpl),
}

impl ImplOrFn {
    fn is_impl(&self) -> bool {
        matches!(self, ImplOrFn::ImplStruct(_))
    }
}

impl Parse for ImplOrFn {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(Impl) {
            Ok(Self::ImplStruct(input.parse()?))
        } else {
            Ok(Self::Function(input.parse()?))
        }
    }
}

/// Macro that time a function and emit a histogram metric using `metrics` crate
/// ```norust
/// #[time("metric_name")]
/// ```
/// This macro can be on a function.
#[proc_macro_attribute]
pub fn time(attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut args = parse_macro_input!(attr as MetricName);
    let input = parse_macro_input!(item as ImplOrFn);

    if args.name.disable() && input.is_impl() {
        return syn::Error::new(args.name.span(), "You can't disable a whole impl block")
            .into_compile_error()
            .into();
    }

    match input {
        ImplOrFn::Function(item_fn) => {
            let output = args.fold_item_fn(item_fn);
            TokenStream::from(quote!(#output))
        }
        ImplOrFn::ImplStruct(impl_struct) => {
            let output = args.fold_item_impl(impl_struct);
            TokenStream::from(quote!(#output))
        }
    }
}
