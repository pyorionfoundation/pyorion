use proc_macro::TokenStream;
use quote::quote;
use syn::{
    FnArg, ItemFn, Pat, Stmt, Type, parse_macro_input, parse_quote, punctuated::Punctuated,
    token::Comma,
};

fn is_option_type(ty: &Type) -> bool {
    if let Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.last() {
            return segment.ident == "Option";
        }
    }
    false
}

fn api_args(inputs: Punctuated<FnArg, Comma>) -> Option<Stmt> {
    let len = inputs.len();
    if len == 0 {
        return None;
    }

    let mut names = Punctuated::<Box<Pat>, Comma>::new();
    let mut types = Punctuated::<Box<Type>, Comma>::new();

    for arg in inputs {
        if let FnArg::Typed(typed) = arg {
            names.push(typed.pat.clone());
            types.push(typed.ty.clone());
        }
    }

    let has_option = types.iter().any(|ty| is_option_type(ty));

    if has_option {
        Some(parse_quote! {
            let (#names,) = req.args().optional::<(#types,)>(#len)?;
        })
    } else {
        Some(parse_quote! {
            let (#names,) = req.args().get::<(#types,)>()?;
        })
    }
}

#[proc_macro_attribute]
pub fn api(_: TokenStream, raw: TokenStream) -> TokenStream {
    let f = parse_macro_input!(raw as ItemFn);
    let name = f.sig.ident;
    let output = f.sig.output;
    let body = f.block.stmts;
    let args_stmt = api_args(f.sig.inputs);
    let expanded = quote! {
        fn #name(
            app: std::sync::Arc<crate::core::App>,
            req: crate::api_manager::ApiRequest,
            target: &crate::utils::FrameWindowTarget,
            flow: &mut tao::event_loop::ControlFlow,
        ) #output {
            #args_stmt
            #(#body)*
        }
    };
    expanded.into()
}
