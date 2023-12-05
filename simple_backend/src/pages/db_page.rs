use maud::{html, PreEscaped};

pub fn initial_page(tables: &Vec<String>) -> PreEscaped<String> {

    let result = html!(
        html {
            head{
                link rel="stylesheet" href="/static/style.css"{};
            }
            body {
                .flexcenter {
                    ol {
                        @for name in tables {
                            li { a href={ "/db/" (name) "/" } {(name)} }
                        }
                    }
                }
            }
        }
    );
    return result;
}

pub fn table_info_page(table_name: &String, table_info: &Vec<(String, String)>) -> PreEscaped<String> {
    let result = html!(
        html {
            head{
                link rel="stylesheet" href="/static/style.css"{};
            }
            body {
                div .flexcenter {
                    h1 { (table_name) }
                    table {
                        tr {
                            th { "column_name"}
                            th { "data_type"}
                        }
                        @for row in table_info {
                            tr {
                                td { (row.0) }
                                td { (row.1) }
                            }
                        }
                    }
                }
                div ."flexcenter" {
                    a href="/db" { "Back" }
                }

            }
        }
    );
    return result;
}