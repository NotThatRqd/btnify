use crate::Button;

pub(super) fn create_page_html<'a, S: 'a>(buttons: impl Iterator<Item = &'a Button<S>>) -> String {
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

fn create_buttons_html<'a, S: 'a>(buttons: impl Iterator<Item = &'a Button<S>>) -> String {
    buttons
        .map(create_button_html)
        .collect()
}

fn create_button_html<S>(button: &Button<S>) -> String {
    format!(r#"<button onclick="showMessage('{}')">{}</button>"#, button.id, button.name)
}

#[cfg(test)]
mod tests {
    use crate::button::ButtonResponse;
    use super::*;

    /// Dummy function that can be used as a button handler
    fn dummy(_: &()) -> ButtonResponse {
        todo!()
    }

    #[test]
    fn create_button_test() {
        let button = create_button_html(&Button::new("Count", dummy));
        assert_eq!(button, r#"<button onclick="showMessage('count')">Count</button>"#);
    }

    #[test]
    fn create_buttons_test() {
        let count = Button::new("Count", dummy);
        let ping = Button::new("Ping", dummy);
        let greet = Button::new("Greet", dummy);

        let list = [count, ping, greet];

        let buttons_html = create_buttons_html(list.iter());

        // todo: make cleaner using raw string
        assert_eq!(buttons_html, "<button onclick=\"showMessage('count')\">Count</button>\
        <button onclick=\"showMessage('ping')\">Ping</button>\
        <button onclick=\"showMessage('greet')\">Greet</button>");
    }
}
