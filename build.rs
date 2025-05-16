use quote::ToTokens;
use std::{env, fs};
use syn::{self, Expr, Item, Stmt, spanned::Spanned};

fn main() {
    // println!("cargo:rerun-if-changed=nope");
    println!("cargo:rerun-if-changed=main.rs");
    println!("cargo:rerun-if-changed=build.rs");

    let contents = fs::read_to_string("src/main.rs").unwrap();
    let ast = syn::parse_file(contents.as_str()).unwrap();

    for item in ast.items.iter() {
        if let Item::Fn(function) = item {
            if let Some(Stmt::Macro(macr)) = function.block.stmts.first() {
                let sprinkle_here = macr
                    .mac
                    .path
                    .segments
                    .first()
                    .and_then(|segment| Some(segment.ident.to_string()))
                    .and_then(|name| Some(name == "sprinkle_magic_here"))
                    .unwrap_or_default();

                if sprinkle_here {
                    let name = function.sig.ident.to_string();
                    let signature = function.sig.to_token_stream().to_string();

                    println!("sprinkling {} for signature {}", name, signature);
                    sprinkle(name.as_str(), signature.as_str(), macr.span().start().line);
                }
            }
        }
    }

    // I couldn't find another way to debug this script.
    // fs::read_to_string("non_existent_rs").unwrap();
}

// Generates a function body and puts it in the OUT_DIR with the same name as
// the fn name.
fn sprinkle(fn_name: &str, signature: &str, line: usize) {
    let code = complete_code(signature)
        .expect(
            format!(
                "Something went wrong while generating sprinkle for fn {}",
                fn_name
            )
            .as_str(),
        )
        .expect(format!("Could not generate sprinkle for fn {}", fn_name).as_str());
    println!("Generated {} for {}", code, fn_name);

    let out_dir = env::var("OUT_DIR").expect("Could not find OUT_DIR");
    let base_path = format!("{}/sprinkles", out_dir);

    fs::create_dir_all(base_path.clone()).expect("Could not create magic directory");
    println!("{}", format!("{}/{}", base_path, line));
    fs::write(format!("{}/{}", base_path, line), code).expect("Could not write code to magic file");
}

// The autocompleter that will be used.
// Ideally, this lives in its own crate.
use reqwest::{self, Method};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Part {
    text: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Content {
    parts: Vec<Part>,
}

#[derive(Debug, Serialize)]
struct Payload {
    contents: Vec<Content>,
}

#[derive(Debug, Deserialize)]
struct ResponseContent {
    parts: Vec<Part>,
}

#[derive(Debug, Deserialize)]
struct Candidate {
    content: ResponseContent,
}

#[derive(Debug, Deserialize)]
struct GeminiResult {
    candidates: Vec<Candidate>,
}

pub fn complete_code(signature: &str) -> reqwest::Result<Option<String>> {
    let client = reqwest::blocking::Client::new();

    let req = client.request(
        Method::POST,
        "https://generativelanguage.googleapis.com/v1beta/models/gemini-2.0-flash:generateContent?key=AIzaSyCD1eut94ge1BhrrTdfznGD8Ntp0yiLMe8",
    );

    let payload = Payload {
        contents: vec![Content {
            parts: vec![Part {
                text: format!(
                    r#"
                    You are a Rust code generation engine.
                    Your role is to take a function signature in Rust and generate its entire body.
                    If necessary, return appropriate values.
                    Use explicit return statements.
                    Output the plain contents of the function body.
                    Do not response with Markdown or code blocks.

                    The function signature is,
                    {}
                    "#,
                    signature,
                )
                .to_owned(),
            }],
        }],
    };

    let autocomplete = req
        .json(&payload)
        .send()?
        .json::<GeminiResult>()?
        .candidates
        .first()
        .and_then(|candidate| Some(&candidate.content))
        .and_then(|content| content.parts.first())
        .and_then(|part| Some(part.text.clone()));

    if let Some(code) = autocomplete {
        // The AI loves to add the code in braces.
        // But, this could also remove legitimate brances, oh well.
        Ok(Some(
            code.trim()
                .trim()
                .to_owned(),
        ))
    } else {
        Ok(None)
    }
}
