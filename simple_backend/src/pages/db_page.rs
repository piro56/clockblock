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

pub fn initial_page(tables: &Vec<String>) -> PreEscaped<String> {

    let result = html!(
        html {
            head{
                link rel="stylesheet" href="/static/style.css"{};
            }
            body {
                ol {
                    @for name in tables {
                        li { a href={ "/db/" (name) "/" } {(name)} }
                    }
                }
            }
        }
    );
    return result;
}