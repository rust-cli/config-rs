
pub(crate) trait ExpandEnvVars {
    fn expand_env_vars(&self) -> String;
}

#[cfg(not(feature = "substitute_env"))]
impl ExpandEnvVars for str {
    fn expand_env_vars(&self) -> String {
        self.to_string()
    }
}

#[cfg(feature = "substitute_env")]
impl ExpandEnvVars for str {
    fn expand_env_vars(&self) -> String {
        let mut result = String::with_capacity(self.len());
        let mut chars = self.chars().peekable();

        while let Some(ch) = chars.next() {
            if ch == '$' && chars.peek() == Some(&'{') {
                chars.next(); // consume '{'

                // Find the closing '}'
                let mut var_expr = String::new();
                let mut found_closing = false;

                while let Some(ch) = chars.next() {
                    if ch == '}' {
                        found_closing = true;
                        break;
                    }
                    var_expr.push(ch);
                }

                if found_closing {
                    // Check for default value syntax: VAR:-default
                    if let Some(colon_pos) = var_expr.find(":-") {
                        let var_name = &var_expr[..colon_pos];
                        let default_value = &var_expr[colon_pos + 2..];

                        match std::env::var(var_name) {
                            Ok(value) if !value.is_empty() => result.push_str(&value),
                            _ => result.push_str(default_value),
                        }
                    } else {
                        // Simple variable reference: VAR
                        match std::env::var(&var_expr) {
                            Ok(value) => result.push_str(&value),
                            Err(_) => {
                                // Variable not found, leave the original syntax
                                result.push_str("${");
                                result.push_str(&var_expr);
                                result.push('}');
                            }
                        }
                    }
                } else {
                    // No closing '}' found, treat as literal text
                    result.push('$');
                    result.push('{');
                    result.push_str(&var_expr);
                }
            } else {
                result.push(ch);
            }
        }

        result
    }
}

#[cfg(all(test, feature = "substitute_env"))]
mod tests {
    use super::*;
    use temp_env;

    #[test]
    fn test_no_env_vars() {
        let input = "hello world";
        assert_eq!(input.expand_env_vars(), "hello world");
    }

    #[test]
    fn test_simple_env_var_exists() {
        temp_env::with_vars([("TEST_VAR", Some("test_value"))], || {
            let input = "prefix ${TEST_VAR} suffix";
            assert_eq!(input.expand_env_vars(), "prefix test_value suffix");
        });
    }

    #[test]
    fn test_simple_env_var_not_exists() {
        temp_env::with_vars([("NONEXISTENT_VAR", None::<&str>)], || {
            let input = "prefix ${NONEXISTENT_VAR} suffix";
            assert_eq!(input.expand_env_vars(), "prefix ${NONEXISTENT_VAR} suffix");
        });
    }

    #[test]
    fn test_nonexistent_with_default() {
        temp_env::with_vars([("NONEXISTENT_VAR", None::<&str>)], || {
            let input = "prefix ${NONEXISTENT_VAR:-43.7224985} suffix";
            assert_eq!(input.expand_env_vars(), "prefix 43.7224985 suffix");
        });
    }

    #[test]
    fn test_env_var_with_default_var_exists() {
        temp_env::with_vars([("TEST_VAR", Some("actual_value"))], || {
            let input = "prefix ${TEST_VAR:-default_value} suffix";
            assert_eq!(input.expand_env_vars(), "prefix actual_value suffix");
        });
    }

    #[test]
    fn test_env_var_with_default_var_not_exists() {
        temp_env::with_vars([("NONEXISTENT_VAR", None::<&str>)], || {
            let input = "prefix ${NONEXISTENT_VAR:-default_value} suffix";
            assert_eq!(input.expand_env_vars(), "prefix default_value suffix");
        });
    }

    #[test]
    fn test_env_var_with_default_var_empty() {
        temp_env::with_vars([("EMPTY_VAR", Some(""))], || {
            let input = "prefix ${EMPTY_VAR:-default_value} suffix";
            assert_eq!(input.expand_env_vars(), "prefix default_value suffix");
        });
    }

    #[test]
    fn test_multiple_env_vars() {
        temp_env::with_vars([("VAR1", Some("value1")), ("VAR2", Some("value2"))], || {
            let input = "${VAR1} and ${VAR2}";
            assert_eq!(input.expand_env_vars(), "value1 and value2");
        });
    }

    #[test]
    fn test_multiple_env_vars_mixed_existence() {
        temp_env::with_vars(
            [("EXISTS", Some("found")), ("MISSING", None::<&str>)],
            || {
                let input = "${EXISTS} and ${MISSING}";
                assert_eq!(input.expand_env_vars(), "found and ${MISSING}");
            },
        );
    }

    #[test]
    fn test_multiple_env_vars_with_defaults() {
        temp_env::with_vars([("VAR1", Some("actual1")), ("VAR2", None::<&str>)], || {
            let input = "${VAR1:-default1} and ${VAR2:-default2}";
            assert_eq!(input.expand_env_vars(), "actual1 and default2");
        });
    }

    // ... existing code ...

    #[test]
    fn test_variable_name_with_underscores() {
        temp_env::with_vars([("TEST_VAR_123", Some("underscore_value"))], || {
            let input = "${TEST_VAR_123}";
            assert_eq!(input.expand_env_vars(), "underscore_value");
        });
    }

    #[test]
    fn test_consecutive_variables() {
        temp_env::with_vars([("A", Some("a")), ("B", Some("b"))], || {
            let input = "${A}${B}";
            assert_eq!(input.expand_env_vars(), "ab");
        });
    }

    #[test]
    fn test_variable_at_start_and_end() {
        temp_env::with_vars([("START", Some("beginning")), ("END", Some("end"))], || {
            let input = "${START} middle ${END}";
            assert_eq!(input.expand_env_vars(), "beginning middle end");
        });
    }

    #[test]
    fn test_only_variable() {
        temp_env::with_vars([("ONLY_VAR", Some("only"))], || {
            let input = "${ONLY_VAR}";
            assert_eq!(input.expand_env_vars(), "only");
        });
    }

    #[test]
    fn test_complex_scenario() {
        temp_env::with_vars(
            [
                ("HOME", Some("/home/user")),
                ("USER", Some("testuser")),
                ("EDITOR", None::<&str>),
            ],
            || {
                let input =
                    "User: ${USER}, Home: ${HOME}, Editor: ${EDITOR:-vim}, Config: ${HOME}/.config";
                assert_eq!(
                    input.expand_env_vars(),
                    "User: testuser, Home: /home/user, Editor: vim, Config: /home/user/.config"
                );
            },
        );
    }

    // ... existing code ...

    #[test]
    fn test_colon_without_dash() {
        temp_env::with_vars([("TEST_VAR", Some("value"))], || {
            // This should not be treated as default syntax since it's missing the dash
            let input = "${TEST_VAR:not_default}";
            assert_eq!(input.expand_env_vars(), "${TEST_VAR:not_default}");
        });
    }

    #[test]
    fn test_multiple_colon_dash_in_default() {
        temp_env::with_vars([("MISSING", None::<&str>)], || {
            // Only the first :- should be treated as default syntax
            let input = "${MISSING:-default:-with:-colons}";
            assert_eq!(input.expand_env_vars(), "default:-with:-colons");
        });
    }

    #[test]
    fn test_env_var_with_numeric_value() {
        temp_env::with_vars([("PORT", Some("8080"))], || {
            let input = "Server running on port ${PORT}";
            assert_eq!(input.expand_env_vars(), "Server running on port 8080");
        });
    }

    #[test]
    fn test_boolean_like_env_var() {
        temp_env::with_vars([("DEBUG", Some("true"))], || {
            let input = "Debug mode: ${DEBUG:-false}";
            assert_eq!(input.expand_env_vars(), "Debug mode: true");
        });
    }
}
