use ntree::models::{escape_mermaid_label, validate_mermaid};

#[test]
fn test_escape_single_quotes() {
    let input = "let x = 'hello';";
    let escaped = escape_mermaid_label(input);
    assert_eq!(escaped, "let x = &apos;hello&apos;;");
}

#[test]
fn test_escape_double_quotes() {
    let input = r#"let x = "hello";"#;
    let escaped = escape_mermaid_label(input);
    assert_eq!(escaped, "let x = &quot;hello&quot;;");
}

#[test]
fn test_escape_angle_brackets() {
    let input = "if x < 5 && y > 3";
    let escaped = escape_mermaid_label(input);
    assert_eq!(escaped, "if x &lt; 5 &amp;&amp; y &gt; 3");
}

#[test]
fn test_validate_mermaid_valid() {
    let valid = r#"graph TD
    0([ENTRY])
    1["let x = &quot;hello&quot;;"]
    2{"x &gt; 5"}
    0 --> 1
    1 --> 2"#;

    assert!(validate_mermaid(valid).is_ok());
}

#[test]
fn test_validate_mermaid_invalid_quotes() {
    let invalid = r#"graph TD
    0([ENTRY])
    1[let x = 'hello';]
    0 --> 1"#;

    let result = validate_mermaid(invalid);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Unescaped single quote"));
}