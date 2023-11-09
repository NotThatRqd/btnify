use crate::Button;
use crate::button::ButtonHandler;

pub(super) fn create_page_html<'a, S: Send + Sync + 'static>(buttons: impl Iterator<Item = &'a Button<S>>) -> String {
    let buttons = create_buttons_html(buttons);

    format!(r#"<!DOCTYPE html>
<html lang="en">
    <head>
        <title>BTNify</title>
        <meta charset="utf-8">
    </head>
    <body>
        {}

        <script>
            async function showMessage(id, extra_questions = null) {{
                let extra_responses = [];
                if (extra_questions !== null) {{
                    for (const question of extra_questions) {{
                        let response = prompt(question);
                        extra_responses.push(response);
                    }}
                }}
                const data = await postData("/", {{ id, extra_responses }});
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

fn create_buttons_html<'a, S: Send + Sync + 'static>(buttons: impl Iterator<Item = &'a Button<S>>) -> String {
    buttons
        .enumerate()
        .map(|(id, b)| create_button_html(b, id))
        .collect()
}

fn create_button_html<S: Send + Sync + 'static>(button: &Button<S>, id: usize) -> String {
    match &button.handler {
        ButtonHandler::Basic(_) | ButtonHandler::WithState(_) => {
            format!(r#"<button onclick="showMessage({id}, null)">{}</button>"#, button.name)
        }
        ButtonHandler::WithExtraPrompts(_, extra_prompts) | ButtonHandler::WithBoth(_, extra_prompts) => {
            let questions_array = create_questions_array(&extra_prompts);
            format!(r#"<button onclick="showMessage({id}, {questions_array})">{}</button>"#, button.name)
        }
    }
}

fn create_questions_array(extra_prompts: &Vec<String>) -> String {
    let questions_array = extra_prompts
        .iter()

        .map(|question| sanitize_for_js_string(question))

        // put single quotes around each question
        .map(|question| format!("'{question}'"))

        // separate each question with a comma
        .collect::<Vec<String>>()
        .join(",");

    // surround with array brackets
    let questions_array = format!("[{questions_array}]");

    questions_array
}

fn sanitize_for_js_string(input: &str) -> String {
    input
        .chars()
        .map(|c| match c {
            '\\' => "\\\\".to_string(), // Escape backslash
            '\'' => "\\\'".to_string(), // Escape single quote
            '"' => "\\\"".to_string(),  // Escape double quote
            '\n' => "\\n".to_string(),  // Escape newline character
            '\r' => "\\r".to_string(),  // Escape carriage return character
            '\t' => "\\t".to_string(),  // Escape tab character
            _ => c.to_string()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::button::ButtonResponse;
    use super::*;

    /// Dummy function that can be used as a button handler
    fn dummy(_: &(), _: Option<Vec<Option<String>>>) -> ButtonResponse {
        unimplemented!()
    }

    #[test]
    fn create_button_test() {
        let button = create_button_html(&Button::new("Count", dummy, None), 0);
        assert_eq!(button, r#"<button onclick="showMessage(0, null)">Count</button>"#);
    }

    #[test]
    fn create_buttons_test() {
        let count = Button::new(
            "Count",
            dummy,
            Some(vec!["How much do you want to add?".to_string()])
        );
        let ping = Button::new("Ping", dummy, None);
        let greet = Button::new(
            "Greet",
            dummy,
            Some(vec!["Name?".to_string(), "Fav. Color?".to_string()])
        );

        let list = [count, ping, greet];

        let buttons_html = create_buttons_html(list.iter());

        // todo: make cleaner using raw string
        assert_eq!(buttons_html, "<button onclick=\"showMessage(0, ['How much do you want to add?'])\">Count</button>\
        <button onclick=\"showMessage(1, null)\">Ping</button>\
        <button onclick=\"showMessage(2, ['Name?','Fav. Color?'])\">Greet</button>");
    }
}
