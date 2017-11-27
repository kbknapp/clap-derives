// Copyright (c) 2017 `clap-derive` Authors

//! ## How to `derive(ClapApp)`
//!
//! First, let's look at an example:
//!
//! ```ignore
//! #[derive(ClapApp)]
//! #[clap(name = "myapp", about = "An example of clap_derive usage.")]
//! struct MyApp {
//!     #[clap(short = "d", long = "debug", help = "Activate debug mode")]
//!     debug: bool,
//!     #[clap(short = "s", long = "speed", help = "Set speed", default_value = "42")]
//!     speed: f64,
//!     #[clap(help = "Input file")]
//!     input: String,
//!     #[clap(help = "Output file, stdout if not present")]
//!     output: Option<String>,
//! }
//! ```
//!
//! So `derive(ClapApp)` tells Rust to generate a command line parser,
//! and the various `clap_derive` attributes are simply
//! used for additional parameters.
//!
//! First, define a struct, whatever its name.  This structure will
//! correspond to a `clap::App`.  Every method of `clap::App` in the
//! form of `fn function_name(self, &str)` can be use through attributes
//! placed on the struct. In our example above, the `about` attribute
//! will become an `.about("An example of ClapApp usage.")` call on the
//! generated `clap::App`. There are a few attributes that will default
//! if not specified:
//!
//!   - `name`: The binary name displayed in help messages. Defaults
//!      to the crate name given by Cargo.
//!   - `version`: Defaults to the crate version given by Cargo.
//!   - `author`: Defaults to the crate author name given by Cargo.
//!   - `about`: Defaults to the crate description given by Cargo.
//!
//! Then, each field of the struct not marked as a subcommand corresponds
//! to a `clap::Arg`. As with the struct attributes, every method of
//! `clap::Arg` in the form of `fn function_name(self, &str)` can be used
//! through specifying it as an attribute.
//! The `name` attribute can be used to customize the
//! `Arg::with_name()` call (defaults to the field name).
//! For functions that do not take a `&str` as argument, the attribute can be
//! called `function_name_raw`, e. g. `aliases_raw = "&[\"alias\"]"`.
//!
//! The type of the field gives the kind of argument:
//!
//! Type                 | Effect                               | Added method call to `clap::Arg`
//! ---------------------|--------------------------------------|--------------------------------------
//! `bool`               | `true` if present                    | `.takes_value(false).multiple(false)`
//! `u64`                | number of times the argument is used | `.takes_value(false).multiple(true)`
//! `Option<T: FromStr>` | optional argument                    | `.takes_value(true).multiple(false)`
//! `Vec<T: FromStr>`    | list of arguments                    | `.takes_value(true).multiple(true)`
//! `T: FromStr`         | required argument                    | `.takes_value(true).multiple(false).required(!has_default)`
//!
//! The `FromStr` trait is used to convert the argument to the given
//! type, and the `Arg::validator` method is set to a method using
//! `to_string()` (`FromStr::Err` must implement `std::fmt::Display`).
//! If you would like to use a custom string parser other than `FromStr`, see
//! the [same titled section](#custom-string-parsers) below.
//!
//! Thus, the `speed` argument is generated as:
//!
//! ```ignore
//! clap::Arg::with_name("speed")
//!     .takes_value(true)
//!     .multiple(false)
//!     .required(false)
//!     .validator(parse_validator::<f64>)
//!     .short("s")
//!     .long("speed")
//!     .help("Set speed")
//!     .default_value("42")
//! ```
//!
//! ## Help messages
//!
//! Help messages for the whole binary or individual arguments can be
//! specified using the `about` attribute on the struct/field, as we've
//! already seen. For convenience, they can also be specified using
//! doc comments. For example:
//!
//! ```ignore
//! #[derive(ClapApp)]
//! #[clap(name = "foo")]
//! /// The help message that will be displayed when passing `--help`.
//! struct Foo {
//!   ...
//!   #[clap(short = "b")]
//!   /// The description for the arg that will be displayed when passing `--help`.
//!   bar: String
//!   ...
//! }
//! ```
//!
//! ## Subcommands
//!
//! Some applications, especially large ones, split their functionality
//! through the use of "subcommands". Each of these act somewhat like a separate
//! command, but is part of the larger group.
//! One example is `git`, which has subcommands such as `add`, `commit`,
//! and `clone`, to mention just a few.
//!
//! `clap` has this functionality, and `clap` supports it through enums:
//!
//! ```ignore
//! #[derive(ClapApp)]
//! #[clap(name = "git", about = "the stupid content tracker")]
//! enum Git {
//!     #[clap(name = "add")]
//!     Add {
//!         #[clap(short = "i")]
//!         interactive: bool,
//!         #[clap(short = "p")]
//!         patch: bool,
//!         files: Vec<String>
//!     },
//!     #[clap(name = "fetch")]
//!     Fetch {
//!         #[clap(long = "dry-run")]
//!         dry_run: bool,
//!         #[clap(long = "all")]
//!         all: bool,
//!         repository: Option<String>
//!     },
//!     #[clap(name = "commit")]
//!     Commit {
//!         #[clap(short = "m")]
//!         message: Option<String>,
//!         #[clap(short = "a")]
//!         all: bool
//!     }
//! }
//! ```
//!
//! Using `derive(ClapApp)` on an enum instead of a struct will produce
//! a `clap::App` that only takes subcommands. So `git add`, `git fetch`,
//! and `git commit` would be commands allowed for the above example.
//!
//! `clap_derive` also provides support for applications where certain flags
//! need to apply to all subcommands, as well as nested subcommands:
//!
//! ```ignore
//! #[derive(ClapApp)]
//! #[clap(name = "make-cookie")]
//! struct MakeCookie {
//!     #[clap(name = "supervisor", default_value = "Puck", required = false, long = "supervisor")]
//!     supervising_faerie: String,
//!     #[clap(name = "tree")]
//!     /// The faerie tree this cookie is being made in.
//!     tree: Option<String>,
//!     #[clap(subcommand)]  // Note that we mark a field as a subcommand
//!     cmd: Command
//! }
//!
//! #[derive(ClapApp)]
//! enum Command {
//!     #[clap(name = "pound")]
//!     /// Pound acorns into flour for cookie dough.
//!     Pound {
//!         acorns: u32
//!     },
//!     #[clap(name = "sparkle")]
//!     /// Add magical sparkles -- the secret ingredient!
//!     Sparkle {
//!         #[clap(short = "m")]
//!         magicality: u64,
//!         #[clap(short = "c")]
//!         color: String
//!     },
//!     #[clap(name = "finish")]
//!     Finish {
//!         #[clap(short = "t")]
//!         time: u32,
//!         #[clap(subcommand)]  // Note that we mark a field as a subcommand
//!         type: FinishType
//!     }
//! }
//!
//! #[derive(ClapApp)]
//! enum FinishType {
//!     #[clap(name = "glaze")]
//!     Glaze {
//!         applications: u32
//!     },
//!     #[clap(name = "powder")]
//!     Powder {
//!         flavor: String,
//!         dips: u32
//!     }
//! }
//! ```
//!
//! Marking a field with `clap(subcommand)` will add the subcommands of the
//! designated enum to the current `clap::App`. The designated enum *must* also
//! be derived `ClapApp`. So the above example would take the following
//! commands:
//!
//! + `make-cookie pound 50`
//! + `make-cookie sparkle -mmm --color "green"`
//! + `make-cookie finish 130 glaze 3`
//!
//! ### Optional subcommands
//!
//! A nested subcommand can be marked optional:
//!
//! ```ignore
//! #[derive(ClapApp)]
//! #[clap(name = "foo")]
//! struct Foo {
//!     file: String,
//!     #[clap(subcommand)]
//!     cmd: Option<Command>
//! }
//!
//! #[derive(ClapApp)]
//! enum Command {
//!     Bar,
//!     Baz,
//!     Quux
//! }
//! ```
//!
//! ## Custom string parsers
//!
//! If the field type does not have a `FromStr` implementation, or you would
//! like to provide a custom parsing scheme other than `FromStr`, you may
//! provide a custom string parser using `parse(...)` like this:
//!
//! ```ignore
//! use std::num::ParseIntError;
//! use std::path::PathBuf;
//!
//! fn parse_hex(src: &str) -> Result<u32, ParseIntError> {
//!     u32::from_str_radix(src, 16)
//! }
//!
//! #[derive(ClapApp)]
//! struct HexReader {
//!     #[clap(short = "n", parse(try_from_str = "parse_hex"))]
//!     number: u32,
//!     #[clap(short = "o", parse(from_os_str))]
//!     output: PathBuf,
//! }
//! ```
//!
//! There are four kinds custom string parsers:
//!
//! | Kind              | Signature                             | Default                         |
//! |-------------------|---------------------------------------|---------------------------------|
//! | `from_str`        | `fn(&str) -> T`                       | `::std::convert::From::from`    |
//! | `try_from_str`    | `fn(&str) -> Result<T, E>`            | `::std::str::FromStr::from_str` |
//! | `from_os_str`     | `fn(&OsStr) -> T`                     | `::std::convert::From::from`    |
//! | `try_from_os_str` | `fn(&OsStr) -> Result<T, OsString>`   | (no default function)           |
//!
//! When supplying a custom string parser, `bool` and `u64` will not be treated
//! specially:
//!
//! Type        | Effect            | Added method call to `clap::Arg`
//! ------------|-------------------|--------------------------------------
//! `Option<T>` | optional argument | `.takes_value(true).multiple(false)`
//! `Vec<T>`    | list of arguments | `.takes_value(true).multiple(true)`
//! `T`         | required argument | `.takes_value(true).multiple(false).required(!has_default)`
//!
//! In the `try_from_*` variants, the function will run twice on valid input:
//! once to validate, and once to parse. Hence, make sure the function is
//! side-effect-free.


#![recursion_limit="256"]

extern crate proc_macro;
extern crate syn;
extern crate clap;
#[macro_use] extern crate quote;
#[macro_use] extern crate error_chain;

use errors::*;

mod arg_enum;
mod helpers;
mod errors;
mod clap_app;

/// Parses the inputted stream.
fn derive<F>(input: proc_macro::TokenStream, impl_fn: F) -> Result<proc_macro::TokenStream> 
    where F: Fn(&syn::DeriveInput) -> Result<quote::Tokens> {
    // Construct a string representation of the type definition
    let as_string = input.to_string();
    // Parse the string representation
    let ast = syn::parse_derive_input(&as_string)
        .map_err(|e| ErrorKind::ParseError(e))?;
    let generated_output = impl_fn(&ast)?;
    generated_output.parse()
        .map_err(|e| ErrorKind::ProcLexError(e))?
}

/// It is required to have this seperate and specificly defined.
#[proc_macro_derive(ArgEnum, attributes(case_sensitive))]
pub fn derive_arg_enum(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // @TODO @p4: unwrap->expect: panic! is good because this
    // all happens at compile time

    derive(input, arg_enum::impl_arg_enum).unwrap()
}

/// Generates the `ClapApp` impl.
#[proc_macro_derive(ClapApp, attributes(clap))]
pub fn clap_app(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // @TODO @p4: unwrap->expect: panic! is good because this
    // all happens at compile time

    derive(input, clap_app::impl_clap_app).unwrap()
}

#[derive(Copy, Clone, PartialEq)]
pub(crate) enum Ty {
    Bool,
    U64,
    Vec,
    Option,
    Other,
}

pub(crate) fn ty(t: &syn::Ty) -> Ty {
    if let syn::Ty::Path(None, syn::Path { segments: ref segs, .. }) = *t {
        match segs.last().unwrap().ident.as_ref() {
            "bool" => Ty::Bool,
            "u64" => Ty::U64,
            "Option" => Ty::Option,
            "Vec" => Ty::Vec,
            _ => Ty::Other,
        }
    } else {
        Ty::Other
    }
}

pub(crate) fn sub_type(t: &syn::Ty) -> Option<&syn::Ty> {
    let segs = match *t {
        syn::Ty::Path(None, syn::Path { ref segments, .. }) => segments,
        _ => return None,
    };
    match *segs.last().unwrap() {
        syn::PathSegment {
            parameters: syn::PathParameters::AngleBracketed(
                syn::AngleBracketedParameterData { ref types, .. }),
            ..
        } if !types.is_empty() => Some(&types[0]),
            _ => None,
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum AttrSource { Struct, Field, }

#[derive(Debug)]
pub(crate) enum Parser {
    /// Parse an option to using a `fn(&str) -> T` function. The function should never fail.
    FromStr,
    /// Parse an option to using a `fn(&str) -> Result<T, E>` function. The error will be
    /// converted to a string using `.to_string()`.
    TryFromStr,
    /// Parse an option to using a `fn(&OsStr) -> T` function. The function should never fail.
    FromOsStr,
    /// Parse an option to using a `fn(&OsStr) -> Result<T, OsString>` function.
    TryFromOsStr,
}

fn extract_attrs<'a>(attrs: &'a [syn::Attribute], attr_source: AttrSource) -> Box<Iterator<Item = (syn::Ident, syn::Lit)> + 'a> {
    let settings_attrs = attrs.iter()
        .filter_map(|attr| match attr.value {
            syn::MetaItem::List(ref i, ref v) if i.as_ref() == "clap" => Some(v),
            _ => None,
        }).flat_map(|v| v.iter().filter_map(|mi| match *mi {
            syn::NestedMetaItem::MetaItem(syn::MetaItem::NameValue(ref i, ref l)) =>
                Some((i.clone(), l.clone())),
            _ => None,
        }));

    let doc_comments: Vec<String> = attrs.iter()
        .filter_map(move |attr| {
            if let syn::Attribute {
                value: syn::MetaItem::NameValue(ref name, syn::Lit::Str(ref value, syn::StrStyle::Cooked)),
                is_sugared_doc: true,
                ..
            } = *attr {
                if name != "doc" { return None; }
                let text = value.trim_left_matches("//!")
                    .trim_left_matches("///")
                    .trim_left_matches("/*!")
                    .trim_left_matches("/**")
                    .trim();
                Some(text.into())
            } else {
                None
            }
        })
        .collect();

    let doc_comments = if doc_comments.is_empty() {
        None
    } else {
        // Clap's `App` has an `about` method to set a description,
        // it's `Field`s have a `help` method instead.
        if let AttrSource::Struct = attr_source {
            Some(("about".into(), doc_comments.join(" ").into()))
        } else {
            Some(("help".into(), doc_comments.join(" ").into()))
        }
    };

    Box::new(doc_comments.into_iter().chain(settings_attrs))
}

fn from_attr_or_env(attrs: &[(syn::Ident, syn::Lit)], key: &str, env: &str) -> syn::Lit {
    let default = std::env::var(env).unwrap_or("".into());
    attrs.iter()
        .filter(|&&(ref i, _)| i.as_ref() == key)
        .last()
        .map(|&(_, ref l)| l.clone())
        .unwrap_or_else(|| syn::Lit::Str(default, syn::StrStyle::Cooked))
}
