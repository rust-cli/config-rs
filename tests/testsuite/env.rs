use serde::Deserialize;
use snapbox::{assert_data_eq, str};

use config::{Config, Environment, Source};

/// Reminder that tests using env variables need to use different env variable names, since
/// tests can be run in parallel

#[test]
fn test_default() {
    temp_env::with_var("A_B_C", Some("abc"), || {
        let environment = Environment::default();

        assert!(environment.collect().unwrap().contains_key("a_b_c"));
    });
}

#[test]
fn test_prefix_is_removed_from_key() {
    temp_env::with_var("B_A_C", Some("abc"), || {
        let environment = Environment::with_prefix("B");

        assert!(environment.collect().unwrap().contains_key("a_c"));
    });
}

#[test]
fn test_prefix_with_variant_forms_of_spelling() {
    temp_env::with_var("a_A_C", Some("abc"), || {
        let environment = Environment::with_prefix("a");

        assert!(environment.collect().unwrap().contains_key("a_c"));
    });

    temp_env::with_var("aB_A_C", Some("abc"), || {
        let environment = Environment::with_prefix("aB");

        assert!(environment.collect().unwrap().contains_key("a_c"));
    });

    temp_env::with_var("Ab_A_C", Some("abc"), || {
        let environment = Environment::with_prefix("ab");

        assert!(environment.collect().unwrap().contains_key("a_c"));
    });
}

#[test]
fn test_separator_behavior() {
    temp_env::with_var("C_B_A", Some("abc"), || {
        let environment = Environment::with_prefix("C").separator("_");

        assert!(environment.collect().unwrap().contains_key("b.a"));
    });
}

#[test]
fn test_empty_value_is_ignored() {
    temp_env::with_var("C_A_B", Some(""), || {
        let environment = Environment::default().ignore_empty(true);

        assert!(!environment.collect().unwrap().contains_key("c_a_b"));
    });
}

#[test]
fn test_keep_prefix() {
    temp_env::with_var("C_A_B", Some(""), || {
        // Do not keep the prefix
        let environment = Environment::with_prefix("C");

        assert!(environment.collect().unwrap().contains_key("a_b"));

        let environment = Environment::with_prefix("C").keep_prefix(false);

        assert!(environment.collect().unwrap().contains_key("a_b"));

        // Keep the prefix
        let environment = Environment::with_prefix("C").keep_prefix(true);

        assert!(environment.collect().unwrap().contains_key("c_a_b"));
    });
}

#[test]
fn test_custom_separator_behavior() {
    temp_env::with_var("C.B.A", Some("abc"), || {
        let environment = Environment::with_prefix("C").separator(".");

        assert!(environment.collect().unwrap().contains_key("b.a"));
    });
}

#[test]
fn test_custom_prefix_separator_behavior() {
    temp_env::with_var("C-B.A", Some("abc"), || {
        let environment = Environment::with_prefix("C")
            .separator(".")
            .prefix_separator("-");

        assert!(environment.collect().unwrap().contains_key("b.a"));
    });
}

#[test]
fn test_parse_int() {
    // using a struct in an enum here to make serde use `deserialize_any`
    #[derive(Deserialize, Debug)]
    #[serde(tag = "tag")]
    enum TestIntEnum {
        Int(TestInt),
    }

    #[derive(Deserialize, Debug)]
    struct TestInt {
        int_val: i32,
    }

    temp_env::with_var("INT_VAL", Some("42"), || {
        let environment = Environment::default().try_parsing(true);

        let config = Config::builder()
            .set_default("tag", "Int")
            .unwrap()
            .add_source(environment)
            .build()
            .unwrap();

        let config: TestIntEnum = config.try_deserialize().unwrap();

        assert!(matches!(config, TestIntEnum::Int(TestInt { int_val: 42 })));
    });
}

#[test]
fn test_parse_uint() {
    // using a struct in an enum here to make serde use `deserialize_any`
    #[derive(Deserialize, Debug)]
    #[serde(tag = "tag")]
    enum TestUintEnum {
        Uint(TestUint),
    }

    #[derive(Deserialize, Debug)]
    struct TestUint {
        int_val: u32,
    }

    temp_env::with_var("INT_VAL", Some("42"), || {
        let environment = Environment::default().try_parsing(true);

        let config = Config::builder()
            .set_default("tag", "Uint")
            .unwrap()
            .add_source(environment)
            .build()
            .unwrap();

        let config: TestUintEnum = config.try_deserialize().unwrap();

        assert!(matches!(
            config,
            TestUintEnum::Uint(TestUint { int_val: 42 })
        ));
    });
}

#[test]
fn test_parse_float() {
    // using a struct in an enum here to make serde use `deserialize_any`
    #[derive(Deserialize, Debug)]
    #[serde(tag = "tag")]
    enum TestFloatEnum {
        Float(TestFloat),
    }

    #[derive(Deserialize, Debug)]
    struct TestFloat {
        float_val: f64,
    }

    temp_env::with_var("FLOAT_VAL", Some("42.3"), || {
        let environment = Environment::default().try_parsing(true);

        let config = Config::builder()
            .set_default("tag", "Float")
            .unwrap()
            .add_source(environment)
            .build()
            .unwrap();

        let config: TestFloatEnum = config.try_deserialize().unwrap();

        // can't use `matches!` because of float value
        match config {
            TestFloatEnum::Float(TestFloat { float_val }) => {
                assert!(float_cmp::approx_eq!(f64, float_val, 42.3));
            }
        }
    });
}

#[test]
fn test_parse_bool() {
    // using a struct in an enum here to make serde use `deserialize_any`
    #[derive(Deserialize, Debug)]
    #[serde(tag = "tag")]
    enum TestBoolEnum {
        Bool(TestBool),
    }

    #[derive(Deserialize, Debug)]
    struct TestBool {
        bool_val: bool,
    }

    temp_env::with_var("BOOL_VAL", Some("true"), || {
        let environment = Environment::default().try_parsing(true);

        let config = Config::builder()
            .set_default("tag", "Bool")
            .unwrap()
            .add_source(environment)
            .build()
            .unwrap();

        let config: TestBoolEnum = config.try_deserialize().unwrap();

        assert!(matches!(
            config,
            TestBoolEnum::Bool(TestBool { bool_val: true })
        ));
    });
}

#[test]
#[should_panic(expected = "invalid type: string \"42\", expected i32")]
fn test_parse_off_int() {
    // using a struct in an enum here to make serde use `deserialize_any`
    #[derive(Deserialize, Debug)]
    #[serde(tag = "tag")]
    enum TestIntEnum {
        #[allow(dead_code)]
        Int(TestInt),
    }

    #[derive(Deserialize, Debug)]
    struct TestInt {
        #[allow(dead_code)]
        int_val_1: i32,
    }

    temp_env::with_var("INT_VAL_1", Some("42"), || {
        let environment = Environment::default().try_parsing(false);

        let config = Config::builder()
            .set_default("tag", "Int")
            .unwrap()
            .add_source(environment)
            .build()
            .unwrap();

        config.try_deserialize::<TestIntEnum>().unwrap();
    });
}

#[test]
#[should_panic(expected = "invalid type: string \"42.3\", expected f64")]
fn test_parse_off_float() {
    // using a struct in an enum here to make serde use `deserialize_any`
    #[derive(Deserialize, Debug)]
    #[serde(tag = "tag")]
    enum TestFloatEnum {
        #[allow(dead_code)]
        Float(TestFloat),
    }

    #[derive(Deserialize, Debug)]
    struct TestFloat {
        #[allow(dead_code)]
        float_val_1: f64,
    }

    temp_env::with_var("FLOAT_VAL_1", Some("42.3"), || {
        let environment = Environment::default().try_parsing(false);

        let config = Config::builder()
            .set_default("tag", "Float")
            .unwrap()
            .add_source(environment)
            .build()
            .unwrap();

        config.try_deserialize::<TestFloatEnum>().unwrap();
    });
}

#[test]
#[should_panic(expected = "invalid type: string \"true\", expected a boolean")]
fn test_parse_off_bool() {
    // using a struct in an enum here to make serde use `deserialize_any`
    #[derive(Deserialize, Debug)]
    #[serde(tag = "tag")]
    enum TestBoolEnum {
        #[allow(dead_code)]
        Bool(TestBool),
    }

    #[derive(Deserialize, Debug)]
    struct TestBool {
        #[allow(dead_code)]
        bool_val_1: bool,
    }

    temp_env::with_var("BOOL_VAL_1", Some("true"), || {
        let environment = Environment::default().try_parsing(false);

        let config = Config::builder()
            .set_default("tag", "Bool")
            .unwrap()
            .add_source(environment)
            .build()
            .unwrap();

        config.try_deserialize::<TestBoolEnum>().unwrap();
    });
}

#[test]
#[should_panic(expected = "invalid type: string \"not an int\", expected i32")]
fn test_parse_int_fail() {
    // using a struct in an enum here to make serde use `deserialize_any`
    #[derive(Deserialize, Debug)]
    #[serde(tag = "tag")]
    enum TestIntEnum {
        #[allow(dead_code)]
        Int(TestInt),
    }

    #[derive(Deserialize, Debug)]
    struct TestInt {
        #[allow(dead_code)]
        int_val_2: i32,
    }

    temp_env::with_var("INT_VAL_2", Some("not an int"), || {
        let environment = Environment::default().try_parsing(true);

        let config = Config::builder()
            .set_default("tag", "Int")
            .unwrap()
            .add_source(environment)
            .build()
            .unwrap();

        config.try_deserialize::<TestIntEnum>().unwrap();
    });
}

#[test]
#[should_panic(expected = "invalid type: string \"not a float\", expected f64")]
fn test_parse_float_fail() {
    // using a struct in an enum here to make serde use `deserialize_any`
    #[derive(Deserialize, Debug)]
    #[serde(tag = "tag")]
    enum TestFloatEnum {
        #[allow(dead_code)]
        Float(TestFloat),
    }

    #[derive(Deserialize, Debug)]
    struct TestFloat {
        #[allow(dead_code)]
        float_val_2: f64,
    }

    temp_env::with_var("FLOAT_VAL_2", Some("not a float"), || {
        let environment = Environment::default().try_parsing(true);

        let config = Config::builder()
            .set_default("tag", "Float")
            .unwrap()
            .add_source(environment)
            .build()
            .unwrap();

        config.try_deserialize::<TestFloatEnum>().unwrap();
    });
}

#[test]
#[should_panic(expected = "invalid type: string \"not a bool\", expected a boolean")]
fn test_parse_bool_fail() {
    // using a struct in an enum here to make serde use `deserialize_any`
    #[derive(Deserialize, Debug)]
    #[serde(tag = "tag")]
    enum TestBoolEnum {
        #[allow(dead_code)]
        Bool(TestBool),
    }

    #[derive(Deserialize, Debug)]
    struct TestBool {
        #[allow(dead_code)]
        bool_val_2: bool,
    }

    temp_env::with_var("BOOL_VAL_2", Some("not a bool"), || {
        let environment = Environment::default().try_parsing(true);

        let config = Config::builder()
            .set_default("tag", "Bool")
            .unwrap()
            .add_source(environment)
            .build()
            .unwrap();

        config.try_deserialize::<TestBoolEnum>().unwrap();
    });
}

#[test]
fn test_parse_string_and_list() {
    // using a struct in an enum here to make serde use `deserialize_any`
    #[derive(Deserialize, Debug)]
    #[serde(tag = "tag")]
    enum TestStringEnum {
        String(TestString),
    }

    #[derive(Deserialize, Debug)]
    struct TestString {
        string_val: String,
        string_list: Vec<String>,
    }

    temp_env::with_vars(
        vec![
            ("LIST_STRING_LIST", Some("test,string")),
            ("LIST_STRING_VAL", Some("test,string")),
        ],
        || {
            let environment = Environment::default()
                .prefix("LIST")
                .list_separator(",")
                .with_list_parse_key("string_list")
                .try_parsing(true);

            let config = Config::builder()
                .set_default("tag", "String")
                .unwrap()
                .add_source(environment)
                .build()
                .unwrap();

            let config: TestStringEnum = config.try_deserialize().unwrap();

            match config {
                TestStringEnum::String(TestString {
                    string_val,
                    string_list,
                }) => {
                    assert_eq!(String::from("test,string"), string_val);
                    assert_eq!(
                        vec![String::from("test"), String::from("string")],
                        string_list
                    );
                }
            }
        },
    );
}

#[test]
fn test_parse_string_and_list_ignore_list_parse_key_case() {
    // using a struct in an enum here to make serde use `deserialize_any`
    #[derive(Deserialize, Debug)]
    #[serde(tag = "tag")]
    #[allow(dead_code)]
    enum TestStringEnum {
        String(TestString),
    }

    #[derive(Deserialize, Debug)]
    #[allow(dead_code)]
    struct TestString {
        string_val: String,
        string_list: Vec<String>,
    }

    temp_env::with_vars(
        vec![
            ("LIST_STRING_LIST", Some("test,string")),
            ("LIST_STRING_VAL", Some("test,string")),
        ],
        || {
            let environment = Environment::default()
                .prefix("LIST")
                .list_separator(",")
                .with_list_parse_key("STRING_LIST")
                .try_parsing(true);

            let config = Config::builder()
                .set_default("tag", "String")
                .unwrap()
                .add_source(environment)
                .build()
                .unwrap();

            let res = config.try_deserialize::<TestStringEnum>();

            assert!(res.is_err());
            assert_data_eq!(
                res.unwrap_err().to_string(),
                str![[r#"invalid type: string "test,string", expected a sequence"#]]
            );
        },
    );
}

#[test]
#[cfg(feature = "convert-case")]
fn test_parse_nested_kebab() {
    use config::Case;

    #[derive(Deserialize, Debug)]
    #[serde(rename_all = "kebab-case")]
    struct TestConfig {
        single: String,
        plain: SimpleInner,
        value_with_multipart_name: String,
        inner_config: ComplexInner,
    }

    #[derive(Deserialize, Debug)]
    #[serde(rename_all = "kebab-case")]
    struct SimpleInner {
        val: String,
    }

    #[derive(Deserialize, Debug)]
    #[serde(rename_all = "kebab-case")]
    struct ComplexInner {
        another_multipart_name: String,
    }

    temp_env::with_vars(
        vec![
            ("PREFIX__SINGLE", Some("test")),
            ("PREFIX__PLAIN__VAL", Some("simple")),
            ("PREFIX__VALUE_WITH_MULTIPART_NAME", Some("value1")),
            (
                "PREFIX__INNER_CONFIG__ANOTHER_MULTIPART_NAME",
                Some("value2"),
            ),
        ],
        || {
            let environment = Environment::default()
                .prefix("PREFIX")
                .convert_case(Case::Kebab)
                .separator("__");

            let config = Config::builder().add_source(environment).build().unwrap();

            println!("{config:#?}");

            let config: TestConfig = config.try_deserialize().unwrap();

            assert_eq!(config.single, "test");
            assert_eq!(config.plain.val, "simple");
            assert_eq!(config.value_with_multipart_name, "value1");
            assert_eq!(config.inner_config.another_multipart_name, "value2");
        },
    );
}

#[test]
#[cfg(feature = "convert-case")]
fn test_parse_kebab_case_with_exclude_keys() {
    use config::Case;
    #[derive(Deserialize, Debug)]
    struct TestConfig {
        value_a: String,
        #[serde(rename = "value-b")]
        value_b: String,
    }

    temp_env::with_vars(
        vec![("VALUE_A", Some("value1")), ("VALUE_B", Some("value2"))],
        || {
            let environment =
                Environment::default().convert_case_exclude_keys(Case::Kebab, ["value_a"]);

            let config = Config::builder().add_source(environment).build().unwrap();

            let config: TestConfig = config.try_deserialize().unwrap();

            assert_eq!(config.value_a, "value1");
            assert_eq!(config.value_b, "value2");
        },
    );
}

#[test]
#[cfg(feature = "convert-case")]
fn test_parse_kebab_case_for_keys() {
    use config::Case;
    #[derive(Deserialize, Debug)]
    struct TestConfig {
        value_a: String,
        #[serde(rename = "value-b")]
        value_b: String,
    }

    temp_env::with_vars(
        vec![("VALUE_A", Some("value1")), ("VALUE_B", Some("value2"))],
        || {
            let environment =
                Environment::default().convert_case_for_keys(Case::Kebab, ["value_b"]);

            let config = Config::builder().add_source(environment).build().unwrap();

            let config: TestConfig = config.try_deserialize().unwrap();

            assert_eq!(config.value_a, "value1");
            assert_eq!(config.value_b, "value2");
        },
    );
}

#[test]
fn test_parse_string() {
    // using a struct in an enum here to make serde use `deserialize_any`
    #[derive(Deserialize, Debug)]
    #[serde(tag = "tag")]
    enum TestStringEnum {
        String(TestString),
    }

    #[derive(Deserialize, Debug)]
    struct TestString {
        string_val: String,
    }

    temp_env::with_var("STRING_VAL", Some("test string"), || {
        let environment = Environment::default().try_parsing(true);

        let config = Config::builder()
            .set_default("tag", "String")
            .unwrap()
            .add_source(environment)
            .build()
            .unwrap();

        let config: TestStringEnum = config.try_deserialize().unwrap();

        let test_string = String::from("test string");

        match config {
            TestStringEnum::String(TestString { string_val }) => {
                assert_eq!(test_string, string_val);
            }
        }
    });
}

#[test]
fn test_parse_string_list() {
    // using a struct in an enum here to make serde use `deserialize_any`
    #[derive(Deserialize, Debug)]
    #[serde(tag = "tag")]
    enum TestListEnum {
        StringList(TestList),
    }

    #[derive(Deserialize, Debug)]
    struct TestList {
        string_list: Vec<String>,
    }

    temp_env::with_var("STRING_LIST", Some("test string"), || {
        let environment = Environment::default().try_parsing(true).list_separator(" ");

        let config = Config::builder()
            .set_default("tag", "StringList")
            .unwrap()
            .add_source(environment)
            .build()
            .unwrap();

        let config: TestListEnum = config.try_deserialize().unwrap();

        let test_string = vec![String::from("test"), String::from("string")];

        match config {
            TestListEnum::StringList(TestList { string_list }) => {
                assert_eq!(test_string, string_list);
            }
        }
    });
}

#[test]
fn test_parse_off_string() {
    // using a struct in an enum here to make serde use `deserialize_any`
    #[derive(Deserialize, Debug)]
    #[serde(tag = "tag")]
    enum TestStringEnum {
        String(TestString),
    }

    #[derive(Deserialize, Debug)]
    struct TestString {
        string_val_1: String,
    }

    temp_env::with_var("STRING_VAL_1", Some("test string"), || {
        let environment = Environment::default().try_parsing(false);

        let config = Config::builder()
            .set_default("tag", "String")
            .unwrap()
            .add_source(environment)
            .build()
            .unwrap();

        let config: TestStringEnum = config.try_deserialize().unwrap();

        let test_string = String::from("test string");

        match config {
            TestStringEnum::String(TestString { string_val_1 }) => {
                assert_eq!(test_string, string_val_1);
            }
        }
    });
}

#[test]
fn test_parse_int_default() {
    #[derive(Deserialize, Debug)]
    struct TestInt {
        int_val: i32,
    }

    let environment = Environment::default().try_parsing(true);

    let config = Config::builder()
        .set_default("int_val", 42_i32)
        .unwrap()
        .add_source(environment)
        .build()
        .unwrap();

    let config: TestInt = config.try_deserialize().unwrap();
    assert_eq!(config.int_val, 42);
}

#[test]
fn test_parse_uint_default() {
    #[derive(Deserialize, Debug)]
    struct TestUint {
        int_val: u32,
    }

    let environment = Environment::default().try_parsing(true);

    let config = Config::builder()
        .set_default("int_val", 42_u32)
        .unwrap()
        .add_source(environment)
        .build()
        .unwrap();

    let config: TestUint = config.try_deserialize().unwrap();
    assert_eq!(config.int_val, 42);
}

#[cfg(any(unix, windows))]
#[cfg(test)]
mod unicode_tests {
    use std::ffi::OsString;

    use super::*;

    fn make_invalid_unicode_os_string() -> OsString {
        let string = {
            #[cfg(unix)]
            {
                use std::os::unix::ffi::OsStringExt;

                OsString::from_vec(vec![0xff])
            }
            #[cfg(windows)]
            {
                use std::os::windows::ffi::OsStringExt;

                OsString::from_wide(&[0xd800]) // unpaired high surrogate
            }
        };

        assert!(string.to_str().is_none());

        string
    }

    #[test]
    fn test_invalid_unicode_key_ignored() {
        temp_env::with_vars(
            vec![
                (make_invalid_unicode_os_string(), Some("abc")),
                ("A_B_C".into(), Some("abc")),
            ],
            || {
                let vars = Environment::default().collect().unwrap();

                assert!(vars.contains_key("a_b_c"));
            },
        );
    }

    #[test]
    fn test_invalid_unicode_value_filtered() {
        temp_env::with_vars(
            vec![
                ("invalid_value1", Some(make_invalid_unicode_os_string())),
                ("valid_value2", Some("valid".into())),
            ],
            || {
                let vars = Environment::with_prefix("valid")
                    .keep_prefix(true)
                    .collect()
                    .unwrap();

                assert!(!vars.contains_key("invalid_value1"));
                assert!(vars.contains_key("valid_value2"));
            },
        );
    }

    #[test]
    fn test_invalid_unicode_value_not_filtered() {
        temp_env::with_vars(
            vec![("invalid_value1", Some(make_invalid_unicode_os_string()))],
            || {
                let result = Environment::default().collect();

                #[cfg(unix)]
                let expected =
                    str![[r#"env variable "invalid_value1" contains non-Unicode data: "/xFF""#]];
                #[cfg(windows)]
                let expected = str![[
                    r#"env variable "invalid_value1" contains non-Unicode data: "/u{d800}""#
                ]];

                assert_data_eq!(result.unwrap_err().to_string(), expected);
            },
        );
    }
}
