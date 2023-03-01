use proc_macro::TokenStream;

use quote::quote;
use syn::fold::Fold;
use syn::parse::{Parse, ParseStream};
use syn::{parse_macro_input, Block, Expr, ExprBlock, ItemFn, LitStr, Stmt};

struct MetricName {
    name: LitStr,
}

impl Parse for MetricName {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name: LitStr = input.parse()?;

        Ok(Self { name })
    }
}

impl Fold for MetricName {
    fn fold_item_fn(&mut self, i: ItemFn) -> ItemFn {
        let block = *i.block;

        let name = i.sig.ident.to_string();

        let mut statements: Vec<Stmt> = Vec::with_capacity(2);

        let metric_name = self.name.value();

        let macro_stmt = quote!(
            let _guard = function_timer::FunctionTimer::new(#metric_name.to_string(), #name.to_string());
        );
        let macro_stmt: Stmt = syn::parse2(macro_stmt).unwrap();
        statements.push(macro_stmt);

        statements.push(Stmt::Expr(Expr::Block(ExprBlock {
            attrs: vec![],
            label: None,
            block,
        })));

        let new_block = Block {
            brace_token: Default::default(),
            stmts: statements,
        };

        ItemFn {
            attrs: i.attrs,
            vis: i.vis,
            sig: i.sig,
            block: Box::new(new_block),
        }
    }
}

#[derive(Clone)]
enum ImplOrFn {
    Function(ItemFn),
}

impl Parse for ImplOrFn {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self::Function(input.parse()?))
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
    }
}
