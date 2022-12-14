use vk_method::Method;

pub struct ExecuteCompiler;

impl ExecuteCompiler {
    pub fn compile<Methods>(methods: Methods) -> String
    where
        Methods: IntoIterator<Item = Method>,
        <Methods as IntoIterator>::IntoIter: ExactSizeIterator,
    {
        let methods = methods.into_iter();

        let mut code = String::new();
        let method_count = methods.len();
        
        for (index, method) in methods.enumerate() {
            code.push_str(
                format!(
                    "var result{index} = API.{}({});",
                    method.name,
                    serde_json::to_string(&method.params).unwrap()
                )
                .as_str(),
            );
        }
    
        code.push_str("return [");
        for i in 0..method_count {
            code.push_str(format!("result{i},").as_str());
        }
        code.push_str("];");
    
        code
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use vk_method::Params;

    #[test]
    fn compile() {
        let methods = vec![
            Method::new(
                "users.get",
                Params::from([
                    ("user_id", 1.into()),
                    ("fields", vec!["photo50", "verified"].into())
                ])
            )
        ];

        let execute = ExecuteCompiler::compile(methods);

        assert_eq!(execute, "var result0 = API.users.get({\"user_id\":1,\"fields\":[\"photo50\",\"verified\"]});return [result0,];");
    }
}
