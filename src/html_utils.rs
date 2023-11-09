use crate::button::ButtonHandler;
use crate::Button;

pub(super) fn create_page_html<'a, S: Send + Sync + 'static>(
    buttons: impl Iterator<Item = &'a Button<S>>,
) -> String {
    let buttons = create_buttons_html(buttons);

    format!(
        r#"<!DOCTYPE html>
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
</html>"#,
        buttons
    )
}

fn create_buttons_html<'a, S: Send + Sync + 'static>(
    buttons: impl Iterator<Item = &'a Button<S>>,
) -> String {
    buttons
        .enumerate()
        .map(|(id, b)| create_button_html(b, id))
        .collect()
}

fn create_button_html<S: Send + Sync + 'static>(button: &Button<S>, id: usize) -> String {
    match &button.handler {
        ButtonHandler::Basic(_) | ButtonHandler::WithState(_) => {
            format!(
                r#"<button onclick="showMessage({id}, null)">{}</button>"#,
                button.name
            )
        }
        ButtonHandler::WithExtraPrompts(_, extra_prompts)
        | ButtonHandler::WithBoth(_, extra_prompts) => {
            let questions_array = create_questions_array(&extra_prompts);
            format!(
                r#"<button onclick="showMessage({id}, {questions_array})">{}</button>"#,
                button.name
            )
        }
    }
}

fn create_questions_array(extra_prompts: &Vec<String>) -> String {
    let questions_array = extra_prompts
        .iter()
        .map(|question| sanitize_for_js_string(question))
        .map(|question| format!("'{question}'")) // put single quotes around each question
        .collect::<Vec<String>>() // separate each question with a comma
        .join(",");

    // surround with brackets
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
            _ => c.to_string(),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::button::ButtonResponse;
    use html_to_string_macro::html;

    fn basic_dummy() -> ButtonResponse {
        unimplemented!()
    }
    fn prompts_dummy(_: Vec<Option<String>>) -> ButtonResponse {
        unimplemented!()
    }

    #[test]
    fn create_button_test() {
        let button = create_button_html(
            &Button::<()>::create_basic_button("Count", Box::new(basic_dummy)),
            0,
        );
        assert_eq!(
            button,
            html!(<button onclick="showMessage(0, null)">"Count"</button>)
        );
    }

    #[test]
    fn create_buttons_test() {
        let count = Button::create_button_with_prompts(
            "Count",
            Box::new(prompts_dummy),
            vec!["How much do you want to add?".to_string()],
        );
        let ping = Button::create_basic_button("Ping", Box::new(basic_dummy));
        let greet = Button::create_button_with_prompts(
            "Greet",
            Box::new(prompts_dummy),
            vec!["Name?".to_string(), "Fav. Color?".to_string()],
        );

        let list: [Button<()>; 3] = [count, ping, greet];

        let buttons_html = create_buttons_html(list.iter());

        assert_eq!(
            buttons_html,
            html!(
                <button onclick="showMessage(0, ['How much do you want to add?'])">"Count"</button>
                <button onclick="showMessage(1, null)">"Ping"</button>
                <button onclick="showMessage(2, ['Name?','Fav. Color?'])">"Greet"</button>
            )
        );
    }
}
