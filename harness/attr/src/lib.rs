use debug::debug_enabled;
use proc_macro::{Delimiter, Group, TokenStream, TokenTree};
use std::fs::File;
use std::io;
use std::io::Read;
use std::path::Path;

mod debug;
mod module_ref;

#[proc_macro_attribute]
pub fn cogno_test(attr: TokenStream, item: TokenStream) -> TokenStream {
    if debug_enabled() {
        println!("cogno_test attr => {}", attr.to_string());
        println!("cogno_test => {}", item.to_string());
    }

    let mut spec_id = String::new();
    let mut header_src = String::new();
    let mut attr_iter = attr.into_iter();
    if let Some(TokenTree::Ident(id)) = attr_iter.next() {
        match id.to_string().as_str() {
            "spec" => {
                attr_iter.next();
                if let Some(TokenTree::Literal(id)) = attr_iter.next() {
                    spec_id = id.to_string();
                    header_src.push_str(
                        format!(
                            r#"
                        if !controller.lock().unwrap().is_spec_enabled({}) {{
                            cogno::tracing::event!(cogno::tracing::Level::INFO, "skipped");
                            return;
                        }}
                        "#,
                            spec_id
                        )
                        .as_str(),
                    );
                }
            }
            _ => {
                panic!("Unrecognised syntax in test attribute");
            }
        }
    }

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
                        ret.extend(to_token_stream("(controller: &mut std::sync::Arc<std::sync::Mutex<cogno::TestController>>)"));
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

        let mut new_body = TokenStream::new();

        let mut group_stream = group.stream().into_iter().peekable();
        while let Some(tt) = group_stream.peek() {
            match tt.to_string().as_str() {
                "should_eq" | "should_not_eq" | "must_eq" | "must_not_eq" | "may_eq" => {
                    new_body.extend(group_stream.next());

                    if group_stream.peek().is_some()
                        && group_stream.peek().unwrap().to_string() == "!"
                    {
                        new_body.extend(group_stream.next());
                        match group_stream.next() {
                            Some(TokenTree::Group(g)) => {
                                let mut new_group = TokenStream::new();
                                new_group.extend(to_token_stream("controller_thread_ref,"));
                                new_group.extend(g.stream());

                                new_body.extend(Some(TokenTree::from(Group::new(
                                    g.delimiter(),
                                    new_group,
                                ))));
                            }
                            _ => {
                                panic!("expected arguments after assertion macro");
                            }
                        }
                    } else {
                        panic!("identifier conflicts with an assertion macro")
                    }
                }
                _ => {
                    new_body.extend(group_stream.next());
                }
            }
        }

        let mut traced_header_src = String::new();
        traced_header_src.push_str(format!(r#"
        let span = cogno::tracing::span!(cogno::tracing::Level::INFO, "{}");
        let _enter = span.enter();
        cogno::tracing::event!(cogno::tracing::Level::INFO, "enter");
        {}
        "#, fn_name, header_src).as_str());

        let wrapped_body = to_token_stream(
            format!(
                r#"
            {}
            controller.lock().unwrap().register("{}", {});

    let controller_thread_ref = controller.clone();

    let result = std::thread::Builder::new()
    .name("{}".to_string())
    .spawn(move || {{
        std::panic::catch_unwind(move || {{
                {}
            }})
        }}).unwrap().join().unwrap();

        cogno::tracing::event!(cogno::tracing::Level::INFO, "exit");
        match result {{
            Ok(_) => {{
                controller.lock().unwrap().complete();
            }}
            _ => {{}}
        }};
        "#,
                traced_header_src,
                fn_name,
                spec_id,
                fn_name,
                new_body.to_string()
            )
            .as_str(),
        );

        ret.extend(Some(TokenTree::from(Group::new(
            group.delimiter(),
            wrapped_body,
        ))));
    }

    if debug_enabled() {
        println!("cogno_test transformed => {}", ret.to_string());
    }

    ret
}

#[proc_macro_attribute]
pub fn cogno_main(_: TokenStream, item: TokenStream) -> TokenStream {
    if debug_enabled() {
        println!("cogno_main => {}", item.to_string());
    }

    let manifest_path = option_env!("COGNO_MANIFEST");
    // TODO check up to date? should really always run with cargo cogno but could just run with cargo run
    if manifest_path.is_none() {
        panic!("Run with `cargo cogno`")
    }

    let manifest = load_manifest(manifest_path.unwrap()).unwrap();

    if debug_enabled() {
        println!("manifest => {:?}", manifest);
    }

    let mut ret = String::new();
    ret.push_str("fn main() {");

    ret.push_str(r#"
    if "true" == std::env::var("COGNO_TRACE").unwrap_or(String::from("false")).as_str() {
        let sub = cogno::tracing_subscriber::FmtSubscriber::new();
        cogno::tracing::subscriber::set_global_default(sub)
            .expect("setting tracing default failed");
    }
    let span = cogno::tracing::span!(cogno::tracing::Level::INFO, "cogno_main");
    let _enter = span.enter();
    cogno::tracing::event!(cogno::tracing::Level::INFO, "starting");

    let mut controller = std::sync::Arc::new(std::sync::Mutex::new(cogno::TestController::new().unwrap()));
    "#);

    ret.push_str(
        r#"
    let controller_panic_ref = controller.clone();
    std::panic::set_hook(Box::new(move |info| {
        cogno::tracing::event!(cogno::tracing::Level::INFO, "captured a panic - {}", info);
        let mut controller_handle = controller_panic_ref.lock().unwrap();
        controller_handle.set_panic_info(info.to_string());
    }));
    "#,
    );

    for module_ref in manifest {
        ret.push_str(format!("{}", module_ref.to_source()).as_str());
    }

    ret.push_str(r#"
    cogno::tracing::event!(cogno::tracing::Level::INFO, "finishing report");
    let finalize_result = controller.lock().unwrap().finalize();
    finalize_result.unwrap();
    cogno::tracing::event!(cogno::tracing::Level::INFO, "done");
    "#);
    ret.push_str("}");

    let ret = to_token_stream(ret.as_str());

    if debug_enabled() {
        println!("cogno_main transformed => {}", ret.to_string());
    }

    ret
}

fn to_token_stream(code: &str) -> TokenStream {
    code.parse().unwrap()
}

fn load_manifest<P: AsRef<Path>>(source: P) -> Result<Vec<module_ref::ModuleRef>, io::Error> {
    let mut content = String::new();
    File::open(source)?.read_to_string(&mut content)?;
    let module_refs = serde_json::from_str(content.as_str())?;
    Ok(module_refs)
}
