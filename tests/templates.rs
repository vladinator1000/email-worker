#[cfg(test)]
mod tests {
    use email_worker::*;
    use pretty_assertions::assert_eq;

    const TEMPLATE_NAME: &'static str = "test";
    const EXPECTED_TEMPLATE: &'static str = include_str!("../templates/test.hbs");

    async fn fetch() {
        let template = fetch_template(TEMPLATE_NAME).unwrap();

        assert_eq!(template, EXPECTED_TEMPLATE);
    }

    async fn render_template() {
        let result = send(
            "a@a.a".to_string(),
            "a@a.a".to_string(),
            TEMPLATE_NAME.to_string(),
            "".to_string(),
        );

        assert_eq!(result, EXPECTED_TEMPLATE);
    }
}
