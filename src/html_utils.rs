use axum::handler::Handler;
use axum::Json;
use crate::Button;
use crate::button::ButtonInfo;

pub(super) fn create_page_html<'a, H, T, S>(buttons: impl Iterator<Item = &'a Button<H, T, S>>) -> String
where
    H: Handler<T, S, Json<ButtonInfo>>,
    T: 'static,
    S: Clone + Send + Sync + 'static
{
    let buttons = create_buttons_html(buttons);

    format!(r#"<!DOCTYPE html>
<html lang="en">
    <head>
        <title>BTNify</title>
        <meta charset="utf-8">
    </head>
    <body>
        {}
        <button onclick="showMessage(prompt('enter id'))">other</button>

        <script>
            async function showMessage(id) {{
                const data = await postData("/", {{ id }});
                alert(data.message);
            }}

            async function postData(url = "/", data = {{}}) {{
                const response = await fetch(url, {{
                    method: "POST",
                    headers: {{
                        "Content-Type": "application/json"
                    }},
                    body: JSON.stringify(data)
                }});
                return response.json();
            }}
        </script>
    </body>
</html>"#, buttons)
}

fn create_buttons_html<'a, H, T, S>(buttons: impl Iterator<Item = &'a Button<H, T, S>>) -> String
where
    H: Handler<T, S, Json<ButtonInfo>>,
    T: 'static,
    S: Clone + Send + Sync + 'static
{
    buttons
        .map(create_button_html)
        .collect()
}

fn create_button_html<H, T, S>(button: &Button<H, T, S>) -> String
where
    H: Handler<T, S, Json<ButtonInfo>>,
    T: 'static,
    S: Clone + Send + Sync + 'static
{
    format!(r#"<button onclick="showMessage('{}')">{}</button>"#, button.id, button.name)
}

#[cfg(test)]
mod tests {
    use crate::Button;
    use super::*;

    #[test]
    fn create_button_test() {
        let button = create_button_html(&Button::new("Count"));
        assert_eq!(button, r#"<button onclick="showMessage('count')">Count</button>"#);
    }

    #[test]
    fn create_buttons_test() {
        let count = Button::new("Count", ());
        let ping = Button::new("Ping", ());
        let greet = Button::new("Greet", ());

        let list = [count, ping, greet];

        let buttons_html = create_buttons_html(list.iter());

        // todo: make cleaner using raw string
        assert_eq!(buttons_html, "<button onclick=\"showMessage('count')\">Count</button>\
        <button onclick=\"showMessage('ping')\">Ping</button>\
        <button onclick=\"showMessage('greet')\">Greet</button>");
    }
}
