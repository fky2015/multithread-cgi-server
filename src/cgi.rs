use std::collections::HashMap;
use std::process::Command;

#[cfg(test)]
mod cgi_tests {
    use crate::cgi::{cgi_caller, cgi_command, kv_to_env};
    use std::collections::HashMap;


    #[test]
    fn command_test() {
        let path = "ls".to_string();

        let result = cgi_command(&path, &"123".to_string());
        assert_eq!("Cargo.lock\nCargo.toml\nsrc\ntarget\n".to_string(), result);
    }

    #[test]
    fn kv_test() {
        let mut key_values = HashMap::new();
        key_values.insert("value1".to_string(), "123".to_string());
        key_values.insert("value2".to_string(), "234".to_string());
        let re = kv_to_env(key_values);
        assert!(("value2=234&value1=123" == re ||
            "value2=234&value1=123" == re));
    }
}

fn cgi_command(
    path: &String,
    env: &String,
) -> String {
    let output = Command::new(path).output();
    String::from_utf8(output.unwrap().stdout).unwrap()
}

fn kv_to_env(
    key_values: HashMap<String, String>,
) -> String {
    let mut re: String = "".to_string();
    for (key, value) in &key_values {
        re.push('&');
        re.push_str(key.as_str());
        re.push('=');
        re.push_str(value.as_str());
    }
    re[1..].parse().unwrap()
}

fn cgi_caller(
    path: &String,
    key_values: HashMap<String, String>,
) -> String {
    let mut env: String = "".to_string();
    for (key, value) in &key_values {
        env.push('&');
        env.push_str(key.as_str());
        env.push('=');
        env.push_str(value.as_str());
    }
    env[1..].parse().unwrap()
}

