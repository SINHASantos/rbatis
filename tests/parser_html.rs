#[cfg(test)]
mod test {
    use rbatis_codegen::codegen::parser_html::parse_html;

    #[test]
    fn test_parse_line_feed() {
        let mut ig = vec![];
        let token = parse_html(
            r#"
       <!DOCTYPE html PUBLIC "-//W3C//DTD XHTML 1.1//EN" "https://raw.githubusercontent.com/rbatis/rbatis/master/rbatis-codegen/mybatis-3-mapper.dtd">
        <mapper>
            <select id="select_by_condition">
        `select * `
        ` from biz_activity`
         </select>
        </mapper>"#,
            "select_by_condition",
            &mut ig,
        );
        let code = token.to_string();
        println!("{}", token);
        assert!(!code.contains(r#"`"#));
    }

    #[test]
    fn test_parse_line_feed2() {
        let mut ig = vec![];
        let token = parse_html(
            r#"
       <!DOCTYPE html PUBLIC "-//W3C//DTD XHTML 1.1//EN" "https://raw.githubusercontent.com/rbatis/rbatis/master/rbatis-codegen/mybatis-3-mapper.dtd">
        <mapper>
            <select id="select_by_condition">
        `select * from biz_activity
         where`
         </select>
        </mapper>"#,
            "select_by_condition",
            &mut ig,
        );
        let code = token.to_string();
        println!("{}", token);
        assert!(!code.contains(r#"`"#));
    }

    #[test]
    fn test_parse_line_feed3() {
        let mut ig = vec![];
        let token = parse_html(
            r#"
       <!DOCTYPE html PUBLIC "-//W3C//DTD XHTML 1.1//EN" "https://raw.githubusercontent.com/rbatis/rbatis/master/rbatis-codegen/mybatis-3-mapper.dtd">
        <mapper>
            <select id="select_by_condition">
            `select * from biz_activity
         where`
        `id = 1
         and id = 2`
         </select>
        </mapper>"#,
            "select_by_condition",
            &mut ig,
        );
        let code = token.to_string();
        println!("{}", token);
        assert!(!code.contains(r#"`"#));
    }

}
