use std::collections::HashMap;
use std::process::Command;
use std::env;

pub fn cgi_caller(
    path: &String,
    key_values: HashMap<String, String>,
) -> (String,bool) {
    let env_string = kv_to_env(key_values);
    cgi_command(&path,&env_string)
}
fn cgi_command(
    path: &String,
    env_string: &String,
) -> (String, bool) {
    let output = Command::new(path)
        .env("QUERY_STRING", env_string)
        .output()
        .expect("Failed to execute process");
    (String::from_utf8(output.stdout).unwrap(), output.status.success())
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

#[cfg(test)]
mod cgi_tests {
    use crate::cgi::{cgi_caller, cgi_command, kv_to_env};
    use std::collections::HashMap;


    #[test]
    fn calculator_test1() {
        let path = "./cgi-part/cgi-bin/calculator.py".to_string();
        let result = cgi_command(&path, &"value2=234&value1=123".to_string());
        assert_eq!(("Content-type:text/html\n\n<html>\n<head>\n<meta charset=\"utf-8\">\n<title>两数之和与之积</title>\n</head>\n<body>\n<h2>两数之和: 357</h2>\n<h2>两数之积: 28782</h2>\n</body>\n</html>\n".to_string()
                    ,true),
                   result);
    }

    #[test]
    fn calculator_test2() {
        let path = "./cgi-part/cgi-bin/calculator.py".to_string();
        let result = cgi_command(&path, &"xxx".to_string());
        assert_eq!(("".to_string()
                    ,false),
                   result);
    }

    #[test]
    fn kv_test() {
        let mut key_values = HashMap::new();
        key_values.insert("value1".to_string(), "123".to_string());
        key_values.insert("value2".to_string(), "234".to_string());
        let re = kv_to_env(key_values);
        println!("{}", re);
        assert!(("value2=234&value1=123" == re ||
            "value1=123&value2=234" == re));
    }

    #[test]
    fn cgi_test1(){
        let mut key_values = HashMap::new();
        key_values.insert("value1".to_string(), "123".to_string());
        key_values.insert("value2".to_string(), "234".to_string());
        let path = "./cgi-part/cgi-bin/calculator.py".to_string();
        assert_eq!(("Content-type:text/html\n\n<html>\n<head>\n<meta charset=\"utf-8\">\n<title>两数之和与之积</title>\n</head>\n<body>\n<h2>两数之和: 357</h2>\n<h2>两数之积: 28782</h2>\n</body>\n</html>\n".to_string(),
                    true) ,cgi_caller(&path,key_values));
    }

    #[test]
    fn cgi_test2(){
        let mut key_values = HashMap::new();
        key_values.insert("vae1".to_string(), "123".to_string());
        key_values.insert("value2".to_string(), "234".to_string());
        let path = "./cgi-part/cgi-bin/calculator.py".to_string();
        assert_eq!(("".to_string(),false) ,cgi_caller(&path,key_values));
    }

}

