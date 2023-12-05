use maud::{html, PreEscaped};

pub fn get_page() -> PreEscaped<String> {
    let result = html!(
        html {
            head{
                link rel="stylesheet" href="/static/style.css"{};
            }
            body {
                .flexcenter {
                    h1 { "Hello!" }
                    a href="/db" { "Database Info" }
                }
            }
        }
    );
    return result;
}