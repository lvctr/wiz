use crate::format::Formatter;
use crate::ml_decl::MLDecl;
use crate::ml_node::MLNode;
use std::fmt;
use std::fmt::Write;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct MLFile {
    pub name: String,
    pub body: Vec<MLDecl>,
}

impl ToString for MLFile {
    fn to_string(&self) -> String {
        let mut buf = String::new();
        let mut formatter = Formatter::new(&mut buf);
        let _ = self.fmt(&mut formatter);
        buf
    }
}

impl MLNode for MLFile {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        for decl in self.body.iter() {
            decl.fmt(f)?;
            f.write_char('\n')?;
        }
        fmt::Result::Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::expr::{MLExpr, MLName};
    use crate::ml_decl::{MLArgDef, MLDecl, MLField, MLFun, MLFunBody, MLStruct};
    use crate::ml_file::MLFile;
    use crate::ml_type::{MLPrimitiveType, MLType, MLValueType};
    use crate::statement::{MLReturn, MLStmt};

    #[test]
    fn test_ml_file_to_string_empty() {
        let ml_file = MLFile {
            name: "test".to_string(),
            body: vec![],
        };
        assert_eq!(ml_file.to_string(), String::new());
    }

    #[test]
    fn test_ml_file_to_string_struct_no_fields() {
        let ml_file = MLFile {
            name: "test".to_string(),
            body: vec![MLDecl::Struct(MLStruct {
                name: "T".to_string(),
                fields: vec![],
            })],
        };
        assert_eq!(ml_file.to_string(), String::from("struct T {\n};\n"));
    }

    #[test]
    fn test_ml_file_to_string_struct() {
        let ml_file = MLFile {
            name: "test".to_string(),
            body: vec![MLDecl::Struct(MLStruct {
                name: "T".to_string(),
                fields: vec![MLField {
                    name: "i".to_string(),
                    type_: MLValueType::Primitive(MLPrimitiveType::Int64),
                }],
            })],
        };
        assert_eq!(
            ml_file.to_string(),
            String::from("struct T {\n    i:i64,\n};\n")
        );
    }

    #[test]
    fn test_ml_file_to_string_function_no_body() {
        let ml_file = MLFile {
            name: "test".to_string(),
            body: vec![MLDecl::Fun(MLFun {
                modifiers: vec![],
                name: "a".to_string(),
                arg_defs: vec![MLArgDef {
                    name: "b".to_string(),
                    type_: MLValueType::Primitive(MLPrimitiveType::Int64),
                }],
                return_type: MLValueType::Primitive(MLPrimitiveType::Unit),
                body: None,
            })],
        };
        assert_eq!(ml_file.to_string(), String::from("fun a(b:i64):unit;\n"))
    }

    #[test]
    fn test_ml_file_to_string_function_empty_body() {
        let ml_file = MLFile {
            name: "test".to_string(),
            body: vec![MLDecl::Fun(MLFun {
                modifiers: vec![],
                name: "a".to_string(),
                arg_defs: vec![
                    MLArgDef {
                        name: "b".to_string(),
                        type_: MLValueType::Primitive(MLPrimitiveType::Int64),
                    },
                    MLArgDef {
                        name: "c".to_string(),
                        type_: MLValueType::Primitive(MLPrimitiveType::Int64),
                    },
                ],
                return_type: MLValueType::Primitive(MLPrimitiveType::Unit),
                body: Some(MLFunBody { body: vec![] }),
            })],
        };
        assert_eq!(
            ml_file.to_string(),
            String::from("fun a(b:i64, c:i64):unit {\n};\n")
        )
    }

    #[test]
    fn test_ml_file_to_string_function() {
        let ml_file = MLFile {
            name: "test".to_string(),
            body: vec![MLDecl::Fun(MLFun {
                modifiers: vec![],
                name: "a".to_string(),
                arg_defs: vec![MLArgDef {
                    name: "b".to_string(),
                    type_: MLValueType::Primitive(MLPrimitiveType::Int64),
                }],
                return_type: MLValueType::Primitive(MLPrimitiveType::Int64),
                body: Some(MLFunBody {
                    body: vec![MLStmt::Expr(MLExpr::Return(MLReturn {
                        value: Some(Box::new(MLExpr::Name(MLName {
                            name: "b".to_string(),
                            type_: MLType::Value(MLValueType::Primitive(MLPrimitiveType::Int64)),
                        }))),
                    }))],
                }),
            })],
        };
        assert_eq!(
            ml_file.to_string(),
            String::from("fun a(b:i64):i64 {\n    return b;\n};\n")
        )
    }
}
