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
                    You are a code completion engine.
                    Your role is to take a function signature in Rust and emit its body.
                    Do not response with Markdown or code blocks.
                    Output the plain contents of the function body.

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
                .trim_matches('{')
                .trim_matches('}')
                .trim()
                .to_owned(),
        ))
    } else {
        Ok(None)
    }
}
