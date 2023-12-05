use maud::{html, PreEscaped};

pub fn getPage() -> PreEscaped<String> {
    let result = html!(
        html {
            head{
                link rel="stylesheet" href="/static/style.css"{};
            }
            body {
                h1 { "Hello World!" }
            }
        }
    );
    return result;
}