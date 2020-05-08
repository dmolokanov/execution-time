use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, parse_quote, AttributeArgs, ImplItemMethod, Lit, NestedMeta};

#[proc_macro_attribute]
pub fn execution_time(args: TokenStream, function: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as AttributeArgs);

    let mut function = parse_macro_input!(function as ImplItemMethod);
    let function_name = function.sig.ident.to_string();

    assert!(
        args.len() <= 1,
        r#"only one function alias allowed. eg. #[execution_time("{}")]"#,
        function_name
    );

    let name = match args.first() {
        Some(NestedMeta::Lit(Lit::Str(literal))) => literal.value(),
        _ => function_name,
    };

    let block = function.block;
    function.block = parse_quote!({
        let now = std::time::Instant::now();

        let __res = #block;

        println!("{}: {}", #name, now.elapsed().as_micros());

        __res
    });

    TokenStream::from(quote!(#function))
}

// #[proc_macro_attribute]
// pub fn execution_time(args: TokenStream, function: TokenStream) -> TokenStream {
//     assert!(args.is_empty());

//     let mut inner_fn = parse_macro_input!(function as ImplItemMethod);
//     let mut outer_fn = inner_fn.clone();

//     inner_fn.vis = Visibility::Inherited;

//     let inner_fn_ident = format_ident!("__inner_{}", inner_fn.sig.ident);
//     inner_fn.sig.ident = inner_fn_ident.clone();

//     let mut has_self = false;
//     let args: Vec<_> = inner_fn
//         .sig
//         .inputs
//         .iter()
//         .map(|input| match input {
//             FnArg::Receiver(Receiver { self_token, .. }) => {
//                 has_self = true;
//                 quote!(#self_token)
//             }
//             FnArg::Typed(PatType { pat, .. }) => quote!(#pat),
//         })
//         .collect();

//     let name = outer_fn.sig.ident.to_string();

//     let inner_fn_call: Block = parse_quote! ({
//         let now = std::time::Instant::now();

//         let __res = Self::#inner_fn_ident(#(#args),*);

//         println!("{}: {}", #name, now.elapsed().as_micros());

//         __res
//     });

//     // let inner_fn_call = parse_quote!({
//     //     let __res = Self::#inner_fn_ident(#(#args),*);
//     //     __res
//     // });

//     outer_fn.block = inner_fn_call;

//     // outer_fn.block = Box::new(parse_quote!({
//     //     #inner_fn_ident();
//     // }));

//     // let token = quote!({
//     //     #inner_fn
//     //     #outer_fn
//     // });

//     let tokens = quote! {
//         #outer_fn
//         #inner_fn
//     };
//     // dbg!(&tokens);

//     // println!("{:#?}", token);
//     // let mut s = TokenStream::new();
//     // s.extend(token);

//     TokenStream::from(tokens)
// }
