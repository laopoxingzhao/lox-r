use std::{fs::File, io::Write};

pub fn define_ast(output_dir: &str, base_name: &str, types: &[&str]) {
    let path = std::path::Path::new(output_dir).join(format!("{}.rs", base_name));
    let mut file = std::fs::File::create(path).unwrap();

    writeln!(file, "pub struct {} {{", base_name).unwrap();
    for &ty in types {
        let s = ty.split("|").collect::<Vec<_>>();
        // 枚举类型
        //首字母大写
        
        let  enum_type_num = s[0].trim().to_uppercase();
       
        writeln!(file, "    {}: ({}),", enum_type_num, s[0]).unwrap();
    }
    writeln!(file, "}}").unwrap();

    for &ty in types {
        let s = ty.split("|").collect::<Vec<_>>();
        // 枚举类型
        let enum_type_num = s[0].trim().to_uppercase();
        let fields = s[1].split(",").collect::<Vec<_>>();
        define_type(&mut file, &enum_type_num, &fields);
    }


}

pub fn define_type(file: &mut File, type_name: &str, fields: &[&str]) {
    writeln!(file, "pub struct {} {{", type_name).unwrap();
    for field in fields {
        writeln!(file, "    {},", field).unwrap();
    }
    writeln!(file, "}}").unwrap();
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_define_ast() {
        define_ast(
            "./src",
            "Expr",
            &[
        "Binary|left_expr:Box<Expr>,token:Token,right_expr:Box<Expr>",
                "Grouping|",
                "Literal|",
                "Unary|",
            ],
        );
    }
}
