
#[cfg(test)]
mod tests {
    use crate::process_code;

    #[test]
    fn func_call_1() -> Result<(), ()> {
        let ast = process_code(vec!["source".into()], r#"

class TestClass {
    func init(this): string {
        abc();
    }
}

func abc(): string {
}
"#)?;
        crate::tir::build(vec![ast.into()]).unwrap();
        Ok(())
    }

    #[test]
    fn func_call_2() -> Result<(), ()> {
        let ast = process_code(vec!["source".into()], r#"

class TestClass {
    func init(this): string {
        this.abc();
        abc();
    }

    func abc(): string {
    }
}

func abc(): string {
}
"#)?;
        crate::tir::build(vec![ast.into()]).unwrap();
        Ok(())
    }

    #[test]
    fn func_call_3() -> Result<(), ()> {
        let ast = process_code(vec!["source".into()], r#"

class TestClass {
    func init(this): string {
        this.abc("hello");
    }

    func abc(a: string): string {
    }
}
"#)?;
        crate::tir::build(vec![ast.into()]).unwrap();

        let ast = process_code(vec!["source".into()], r#"

        class TestClass {
            func init(this): string {
                this.abc("hello", "world");
            }
            func abc(a: string, b: string): string {
            }
        }
        "#)?;
                crate::tir::build(vec![ast.into()]).unwrap();
        Ok(())
    }

    #[test]
    #[should_panic]
    fn func_call_4() {
        let ast = process_code(vec!["source".into()], r#"

class TestClass {
    func init(this): string {
        this.abc();
    }

    func abc(a: string): string {
    }
}
"#).unwrap();
        crate::tir::build(vec![ast.into()]).unwrap_err();
    }

    #[test]
    #[should_panic]
    fn func_call_5() {
        let ast = process_code(vec!["source".into()], r#"

class TestClass {
    func init(this): string {
        this.abc("hello");
    }

    func abc(): string {
    }
}
"#).unwrap();
        crate::tir::build(vec![ast.into()]).unwrap_err();
    }

    #[test]
    #[should_panic]
    fn func_call_6() {
        let ast = process_code(vec!["source".into()], r#"

class TestClass {
    func init(this): string {
        this.nope();
    }

    func abc(): string {
    }
}
"#).unwrap();
        crate::tir::build(vec![ast.into()]).unwrap_err();
    }

    #[test]
    fn func_call_7() {
        let ast = process_code(vec!["source".into()], r#"

interface ITest {
    func test(a: string): string;
    a: TestClass;
}

extend TestClass: ITest {
    func test(a: string): string {
        
    }
    a: TestClass;
}

class TestClass {
    func init(this): string {
        this.test("erhanbaris");
        this.a.test("baris");
        abc();
    }
}

func abc(): TestClass {
}

"#).unwrap();
        crate::tir::build(vec![ast.into()]).unwrap();
    }
}
