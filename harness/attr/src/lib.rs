use proc_macro::{Delimiter, Group, TokenStream, TokenTree};
use std::path::Path;

#[proc_macro_attribute]
pub fn cogno_test(_: TokenStream, item: TokenStream) -> TokenStream {
    println!("cogno_test => {}", item.to_string());

    let mut ret = TokenStream::new();

    let mut fn_found = false;
    let mut param_injected = false;
    let mut fn_name = String::new();
    for token in item {
        if !param_injected {
            if !fn_found {
                if token.to_string() == "fn" {
                    fn_found = true;
                }
                ret.extend(Some(token));
                continue;
            }

            match token {
                TokenTree::Group(g) => {
                    if g.delimiter() == Delimiter::Parenthesis {
                        ret.extend(to_token_stream("(recorder: &mut cogno::TestRecorder)"));
                        param_injected = true;
                    } else {
                        panic!("unexpected group after test function name");
                    }
                }
                TokenTree::Ident(i) => {
                    fn_name = i.to_string();
                    ret.extend(Some(TokenTree::Ident(i)));
                }
                other => {
                    ret.extend(Some(other));
                }
            }
            continue;
        }

        let group = match token {
            TokenTree::Group(g) => {
                if g.delimiter() == Delimiter::Brace {
                    g
                } else {
                    ret.extend(Some(TokenTree::Group(g)));
                    continue;
                }
            }
            other => {
                ret.extend(Some(other));
                continue;
            }
        };

        let mut new_body = to_token_stream(
            r#"println!("injected before");
            "#,
        );

        new_body.extend(to_token_stream(&format!(
            r#"recorder.register("{}");
            "#,
            fn_name
        )));

        new_body.extend(group.stream());

        new_body.extend(to_token_stream(
            r#"
            println!("injected after");
        "#,
        ));

        new_body.extend(to_token_stream(r#"recorder.complete();"#));

        ret.extend(Some(TokenTree::from(Group::new(
            group.delimiter(),
            new_body,
        ))));
    }

    ret
}

#[proc_macro_attribute]
pub fn cogno_main(_: TokenStream, item: TokenStream) -> TokenStream {
    println!("cogno_main => {}", item.to_string());

    let manifest_path = option_env!("COGNO_MANIFEST");
    // TODO check up to date? should really always run with cargo cogno but could just run with cargo run
    if manifest_path.is_none() {
        panic!("Run with `cargo cogno`")
    }

    let manifest = core::load(manifest_path.unwrap()).unwrap();

    println!("manifest {:?}", manifest);

    let mut ret = String::new();
    ret.push_str("use cogno::TestRecorder;\n");
    ret.push_str("fn main() {\n");

    ret.push_str("  let mut recorder = TestRecorder::new();");

    for fr in manifest {
        println!("{}", fr.to_source());
        ret.push_str(fr.to_source().as_str());
    }

    ret.push_str(r#"  println!("{:?}", recorder);"#);
    ret.push_str("\n}");

    println!("{}", ret.to_string());

    to_token_stream(ret.as_str())
}

fn to_token_stream(code: &str) -> TokenStream {
    code.parse().unwrap()
}
