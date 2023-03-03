//! `time` macro. It can place on any function.
use proc_macro::TokenStream;

use quote::quote;
use syn::fold::Fold;
use syn::parse::{Parse, ParseStream};
use syn::token::Impl;
use syn::{
    parse_macro_input, Attribute, Block, Expr, ExprBlock, ImplItem, ImplItemMethod, ItemFn,
    ItemImpl, LitStr, Stmt, Type,
};

struct MetricName {
    struct_name: Option<String>,
    name: LitStr,
}

impl MetricName {
    fn remove_time_attribut(attributs: Vec<Attribute>) -> Vec<Attribute> {
        attributs
            .into_iter()
            .filter(|attr| {
                attr.path.segments.last().map(|i| i.ident.to_string()) != Some("time".to_string())
            })
            .collect()
    }

    fn block_from(&self, block: Block, function_name: String) -> Block {
        let mut statements: Vec<Stmt> = Vec::with_capacity(2);

        let metric_name = self.name.value();
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
        let macro_stmt: Stmt = syn::parse2(macro_stmt).expect("Can't parse token");
        statements.push(macro_stmt);

        statements.push(Stmt::Expr(Expr::Block(ExprBlock {
            attrs: vec![],
            label: None,
            block,
        })));

        Block {
            brace_token: Default::default(),
            stmts: statements,
        }
    }
}

impl Parse for MetricName {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name: LitStr = input.parse()?;

        Ok(Self {
            struct_name: None,
            name,
        })
    }
}

impl Fold for MetricName {
    fn fold_impl_item_method(&mut self, i: ImplItemMethod) -> ImplItemMethod {
        let mut result = i.clone();

        let block = i.block;
        let name = i.sig.ident.to_string();

        let new_attr = Self::remove_time_attribut(i.attrs);
        result.attrs = new_attr;

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
            if let ImplItem::Method(method) = item {
                new_items.push(ImplItem::Method(self.fold_impl_item_method(method)));
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
