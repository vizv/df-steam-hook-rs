#[macro_use]
extern crate quote;
extern crate proc_macro;
extern crate syn;

use proc_macro::TokenStream;

struct Args {
  offset: usize,
  module: String,
  bypass: bool,
}

impl Args {
  pub fn parse(args: TokenStream) -> Self {
    let mut offset = 0;
    let mut module = String::from("");
    let mut bypass = false;
    for arg_pair in args.to_string().split(",") {
      let arg = arg_pair.split("=").collect::<Vec<&str>>();
      match arg[0].trim() {
        "offset" => offset = usize::from_str_radix(&arg[1].trim().replace("\"", ""), 16).unwrap(),
        "module" => module = String::from(arg[1].trim()),
        "bypass" => bypass = true,
        _ => (),
      }
    }
    Self { offset, module, bypass }
  }
}

#[proc_macro_attribute]
pub fn hook(args: TokenStream, input: TokenStream) -> TokenStream {
  let args = Args::parse(args);
  let item: syn::Item = syn::parse_macro_input!(input);
  if let syn::Item::Fn(function) = item {
    let syn::ItemFn {
      block,
      vis,
      sig:
        syn::Signature {
          ident,
          unsafety,
          constness,
          abi,
          output,
          inputs,
          ..
        },
      ..
    } = function;

    let attach_ident = format_ident!("attach_{}", ident);
    let handle_ident = format_ident!("handle_{}", ident);
    let enable_ident = format_ident!("enable_{}", ident);
    let disable_ident = format_ident!("disable_{}", ident);
    let inputs_unnamed = quote!(#inputs)
      .to_string()
      .split(",")
      .map(|arg| arg.split(":").collect::<Vec<&str>>()[1])
      .collect::<Vec<&str>>()
      .join(",");

    let mut attach = quote!(
      pub unsafe fn #attach_ident() -> Result<()> {
        let target = target();
        #handle_ident.initialize(target, #ident)?;
        #enable_ident()?;
        Ok(())
      }

      pub unsafe fn #enable_ident() -> Result<()> {
        #handle_ident.enable()?;
        Ok(())
      }

      pub unsafe fn #disable_ident() -> Result<()> {
        #handle_ident.disable()?;
        Ok(())
      }
    )
    .to_string();

    attach = match (args.offset, args.module) {
      (o, m) if m == "self" && o > 0 => attach.replace(
        "target()",
        format!("std::mem::transmute(OFFSETS.get({m:?}, {o}))").as_str(),
      ),
      (o, m) if o > 0 => attach.replace(
        "target()",
        format!("std::mem::transmute(OFFSETS.get({m}, {o}))").as_str(),
      ),
      (_, _) => attach.replace(
        "target()",
        format!(
          "std::mem::transmute(OFFSETS.get(offsets::FUNCTIONS.get(\"{ident}\").unwrap()))"
        )
        .as_str(),
      ),
    };

    if args.bypass {
      attach = quote!(
        pub unsafe fn #attach_ident() -> Result<()> {
          Ok(())
        }

        pub unsafe fn #enable_ident() -> Result<()> {
          Ok(())
        }

        pub unsafe fn #disable_ident() -> Result<()> {
          Ok(())
        }
      )
      .to_string();
    }

    let result = quote!(
      static_detour! { static #handle_ident: unsafe #abi fn() #output; }
      #vis #unsafety #constness fn #ident(#inputs) #output #block
    );

    return format!(
      "{}\n{}",
      attach.to_string(),
      result
        .to_string()
        .replace("original!", format!("handle_{}.call", ident.to_string()).as_str())
        .replace("fn()", format!("fn({})", inputs_unnamed).as_str())
    )
    .parse()
    .unwrap();
  } else {
    panic!("Fatal error in hook macro")
  }
}
