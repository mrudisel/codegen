use std::io::{self, Write};
use std::process::{Command, Stdio};

use codegen::*;


fn format_code<S>(raw_code: S) -> io::Result<String>
where
    S: AsRef<str>
{
    let mut spawned = Command::new("rustfmt")
        .args(["--edition", "2021"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;

    let stdin = spawned.stdin.as_mut().expect("no stdin handle");
    stdin.write(raw_code.as_ref().as_bytes())?;

    drop(stdin);

    let output = spawned.wait_with_output()?;


    let formatted = String::from_utf8(output.stdout)
        .unwrap();

    Ok(formatted)
}


#[test]
fn verify_rustfmt_exists() {
    Command::new("rustfmt").arg("--help")
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .status()
        .expect("'rustfmt' must be in your $PATH to properly run tests");
}

#[test]
fn empty_scope() -> io::Result<()> {
    let scope = Scope::new();

    assert_eq!(scope.to_string(), "");
    Ok(())
}

#[test]
fn single_struct() -> io::Result<()> {
    let mut scope = Scope::new();

    scope
        .new_struct("Foo")
        .field("one", "usize")
        .field("two", "String");

    let expected = format_code(r#"
struct Foo {
    one: usize,
    two: String,
}"#)?;

    let generated = format_code(scope.to_string())?;

    assert_eq!(generated, expected);
    Ok(())
}

#[test]
fn struct_with_pushed_field() -> io::Result<()> {
    let mut scope = Scope::new();
    let mut struct_ = Struct::new("Foo");
    let field = Field::new_named("one", "usize");
    struct_.push_field(field);
    scope.push_struct(struct_);

    let expected = format_code(r#"
struct Foo {
    one: usize,
}"#)?;

    let generated = format_code(scope.to_string())?;

    assert_eq!(generated, expected);
    Ok(())
}


#[test]
fn single_struct_documented_field() -> io::Result<()> {
    let mut scope = Scope::new();

    let doc = vec!["Field's documentation", "Second line"];

    let mut struct_ = Struct::new("Foo");

    let mut field1 = Field::new_named("one", "usize");
    field1.push_docs(&doc);
    struct_.push_field(field1);

    let mut field2 = Field::new_named("two", "usize");
    field2.push_attr(r#"#[serde(rename = "bar")]"#);
    struct_.push_field(field2);

    let mut field3 = Field::new_named("three", "usize");
    field3.push_docs(doc).extend_attrs(vec![
        r#"#[serde(skip_serializing)]"#,
        r#"#[serde(skip_deserializing)]"#,
    ]);
    struct_.push_field(field3);

    scope.push_struct(struct_);

    let expected = format_code(r#"
struct Foo {
    /// Field's documentation
    /// Second line
    one: usize,
    #[serde(rename = "bar")]
    two: usize,
    /// Field's documentation
    /// Second line
    #[serde(skip_serializing)]
    #[serde(skip_deserializing)]
    three: usize,
}"#)?;

    let generated = format_code(scope.to_string())?;
    assert_eq!(generated, expected);

    Ok(())
}

#[test]
fn single_fn() -> io::Result<()> {
    let mut scope = Scope::new();
    scope
        .new_fn("my_fn")
        .set_vis(Vis::Pub)
        .arg("foo", Type::new("uint"))
        .ret(Type::new("uint"))
        .line("let res = foo + 1;")
        .line("res");

    let expected = format_code(r#"
pub fn my_fn(foo: uint) -> uint {
    let res = foo + 1;
    res
}"#)?;

    let generated = format_code(scope.to_string())?;

    assert_eq!(generated, expected);
    Ok(())
}

#[test]
fn documented_enum() -> io::Result<()> {
    let mut scope = Scope::new();

    let new_enum = scope.new_enum("AnEnum")
        .set_doc("An enum that has enum-level docs")
        .set_vis(Vis::PubCrate)
        .push_attr("#[serde(rename_all = \"camelCase\")]")
        .push_generic("T")
        .push_bound(Bound::new_with_bound("T", "Clone"))
        .derive_many(["Debug", "Clone", "PartialEq", "Eq", "serde::Deserialize"]);

    new_enum.new_variant("VariantA")
        .push_doc("Variant A docs")
        .push_doc("Some more docs")
        .named("inner", "T")
        .push_attr("#[serde(rename = \"aDifferentName\")]");

    new_enum.new_variant("VariantB")
        .push_doc("Variant B docs")
        .push_attr("#[serde(skip_serializing)]")
        .tuple("Option<T>");

    let expected = format_code(r#"
/// An enum that has enum-level docs
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) enum AnEnum<T>
where
    T: Clone,
{
    /// Variant A docs
    /// Some more docs
    #[serde(rename = "aDifferentName")]
    VariantA {
        inner: T,
    }
    ,
    /// Variant B docs
    #[serde(skip_serializing)]
    VariantB(Option<T>),
}"#)?;

    let generated = format_code(scope.to_string())?;

    assert_eq!(generated, expected);

    Ok(())
}

#[test]
fn empty_struct() -> io::Result<()> {
    let mut scope = Scope::new();

    scope.new_struct("Foo");

    let expected = format_code("struct Foo;")?;
    let generated = format_code(scope.to_string())?;

    assert_eq!(generated, expected);
    Ok(())
}

#[test]
fn two_structs() -> io::Result<()> {
    let mut scope = Scope::new();

    scope.new_struct("Foo")
        .field("one", "usize")
        .field("two", "String");

    scope.new_struct("Bar")
        .field("hello", "World");

    let expected = format_code(r#"
struct Foo {
    one: usize,
    two: String,
}

struct Bar {
    hello: World,
}"#)?;

    let generated = format_code(scope.to_string())?;

    assert_eq!(generated, expected);

    Ok(())
}

#[test]
fn struct_with_derive() -> io::Result<()> {
    let mut scope = Scope::new();

    scope.new_struct("Foo")
        .derive("Debug")
        .derive("Clone")
        .field("one", "usize")
        .field("two", "String");

    let expected = format_code(r#"
#[derive(Debug, Clone)]
struct Foo {
    one: usize,
    two: String,
}"#)?;

    let generated = format_code(scope.to_string())?;

    assert_eq!(generated, expected);
    Ok(())
}

#[test]
fn struct_with_repr() -> io::Result<()> {
    let mut scope = Scope::new();

    scope.new_struct("Foo")
        .repr("C")
        .field("one", "u8")
        .field("two", "u8");

    let expected = format_code(r#"
#[repr(C)]
struct Foo {
    one: u8,
    two: u8,
}"#)?;

    let generated = format_code(scope.to_string())?;

    assert_eq!(generated, expected);
    Ok(())
}

#[test]
fn struct_with_allow() -> io::Result<()> {
    let mut scope = Scope::new();

    scope.new_struct("Foo")
        .allow("dead_code")
        .field("one", "u8")
        .field("two", "u8");

    let expected = format_code(r#"
#[allow(dead_code)]
struct Foo {
    one: u8,
    two: u8,
}"#)?;

    let generated = format_code(scope.to_string())?;

    assert_eq!(generated, expected);
    Ok(())
}

#[test]
fn struct_with_generics_1() -> io::Result<()> {
    let mut scope = Scope::new();

    scope.new_struct("Foo")
        .extend_generics(["T", "U"])
        .field("one", "T")
        .field("two", "U");

    let expected = format_code(r#"
struct Foo<T, U> {
    one: T,
    two: U,
}"#)?;

    let generated = format_code(scope.to_string())?;

    assert_eq!(generated, expected);

    Ok(())
}

#[test]
fn struct_with_generics_2() -> io::Result<()> {
    let mut scope = Scope::new();

    scope.new_struct("Foo")
        .extend_generics(["T", "U"])
        .field("one", "T")
        .field("two", "U");

    let expected = format_code(r#"
struct Foo<T, U> {
    one: T,
    two: U,
}"#)?;

    let generated = format_code(scope.to_string())?;

    assert_eq!(generated, expected);
    Ok(())
}

#[test]
fn struct_with_generics_3() -> io::Result<()> {
    let mut scope = Scope::new();

    scope.new_struct("Foo")
        .extend_generics(["T: Win", "U"])
        .field("one", "T")
        .field("two", "U");

    let expected = format_code(r#"
struct Foo<T: Win, U> {
    one: T,
    two: U,
}"#)?;

    let generated = format_code(scope.to_string())?;

    assert_eq!(generated, expected);

    Ok(())
}

#[test]
fn struct_where_clause_1() -> io::Result<()> {
    let mut scope = Scope::new();

    scope
        .new_struct("Foo")
        .push_generic("T")
        .push_bound(Bound::new_with_bound("T", "Foo"))
        .field("one", "T");

    let expected = format_code(r#"
struct Foo<T>
where T: Foo,
{
    one: T,
}"#)?;

    let generated = format_code(scope.to_string())?;

    assert_eq!(generated, expected);

    Ok(())
}

#[test]
fn struct_where_clause_2() -> io::Result<()> {
    let mut scope = Scope::new();

    scope.new_struct("Foo")
        .extend_generics(["T", "U"])
        .extend_bounds([Bound::new_with_bound("T", "Foo"), Bound::new_with_bound("U", "Baz")])
        .field("one", "T")
        .field("two", "U");

    let expected = format_code(r#"
struct Foo<T, U>
where T: Foo,
      U: Baz,
{
    one: T,
    two: U,
}"#)?;

    let generated = format_code(scope.to_string())?;

    assert_eq!(generated, expected);
    Ok(())
}

#[test]
fn struct_doc() -> io::Result<()> {
    let mut scope = Scope::new();

    scope.new_struct("Foo")
        .push_doc(
            "Hello, this is a doc string\n\
              that continues on another line.",
        )
        .field("one", "T");

    let expected = format_code(r#"
/// Hello, this is a doc string
/// that continues on another line.
struct Foo {
    one: T,
}"#)?;

    let generated = format_code(scope.to_string())?;

    assert_eq!(generated, expected);
    Ok(())
}

#[test]
fn struct_in_mod() -> io::Result<()> {
    let mut scope = Scope::new();

    {
        let module = scope.new_module("foo");

        module.new_struct("Foo")
            .push_doc("Hello some docs")
            .derive("Debug")
            .extend_generics(["T", "U"])
            .push_bound(Bound::new_with_bound("T", "SomeBound"))
            .push_bound(Bound::new_with_bound("U", "SomeOtherBound"))
            .field("one", "T")
            .field("two", "U");
    }

    let expected = format_code(r#"
mod foo {
    /// Hello some docs
    #[derive(Debug)]
    struct Foo<T, U>
    where T: SomeBound,
          U: SomeOtherBound,
    {
        one: T,
        two: U,
    }
}"#)?;

    let generated = format_code(scope.to_string())?;

    assert_eq!(generated, expected);
    Ok(())
}

#[test]
fn struct_mod_import() -> io::Result<()> {
    let mut scope = Scope::new();

    scope.new_module("foo")
        .import("bar", "Bar")
        .new_struct("Foo")
        .field("bar", "Bar");

    let expected = format_code(r#"
mod foo {
    use bar::Bar;

    struct Foo {
        bar: Bar,
    }
}"#)?;

    let generated = format_code(scope.to_string())?;

    assert_eq!(generated, expected);
    Ok(())
}

#[test]
fn enum_with_repr() -> io::Result<()> {
    let mut scope = Scope::new();

    scope.new_enum("IpAddrKind")
        .repr("u8")
        .push_variant(Variant::new("V4"))
        .push_variant(Variant::new("V6"));

    let expected = format_code(r#"
#[repr(u8)]
enum IpAddrKind {
    V4,
    V6,
}"#)?;

    let generated = format_code(scope.to_string())?;

    assert_eq!(generated, expected);
    Ok(())
}

#[test]
fn enum_with_allow() -> io::Result<()> {
    let mut scope = Scope::new();

    scope.new_enum("IpAddrKind")
        .allow("dead_code")
        .push_variant(Variant::new("V4"))
        .push_variant(Variant::new("V6"));

    let expected = format_code(r#"
#[allow(dead_code)]
enum IpAddrKind {
    V4,
    V6,
}"#)?;

    let generated = format_code(scope.to_string())?;

    assert_eq!(generated, expected);
    Ok(())
}

#[test]
fn scoped_imports() -> io::Result<()> {
    let mut scope = Scope::new();
    scope.new_module("foo")
        .import("bar", "Bar")
        .import("bar", "baz::Baz")
        .import("bar::quux", "quuux::Quuuux")
        .new_struct("Foo")
        .field("bar", "Bar")
        .field("baz", "baz::Baz")
        .field("quuuux", "quuux::Quuuux");

    let expected = format_code(r#"
mod foo {
    use bar::{Bar, baz};
    use bar::quux::quuux;

    struct Foo {
        bar: Bar,
        baz: baz::Baz,
        quuuux: quuux::Quuuux,
    }
}"#)?;

    let generated = format_code(scope.to_string())?;

    assert_eq!(generated, expected);
    Ok(())
}

#[test]
fn module_mut() -> io::Result<()> {
    let mut scope = Scope::new();
    scope.new_module("foo").import("bar", "Bar");

    scope.get_module_mut("foo")
        .expect("module_mut")
        .new_struct("Foo")
        .field("bar", "Bar");

    let expected = format_code(r#"
mod foo {
    use bar::Bar;

    struct Foo {
        bar: Bar,
    }
}"#)?;

    let generated = format_code(scope.to_string())?;

    assert_eq!(generated, expected);
    Ok(())
}


#[test]
fn get_or_new_module() -> io::Result<()> {
    let mut scope = Scope::new();
    assert!(scope.get_module("foo").is_none());

    scope.get_or_new_module("foo").import("bar", "Bar");

    scope.get_or_new_module("foo")
        .new_struct("Foo")
        .field("bar", "Bar");

    let expected = format_code(r#"
mod foo {
    use bar::Bar;

    struct Foo {
        bar: Bar,
    }
}"#)?;

    let generated = format_code(scope.to_string())?;

    assert_eq!(generated, expected);
    Ok(())
}

#[test]
fn function_with_async() -> io::Result<()> {
    let mut scope = Scope::new();
    let trt = scope.new_trait("Foo");

    let f = trt.new_fn("pet_toby");
    f.set_async(true);
    f.line("println!(\"petting toby because he is a good boi\");");

    let expected = format_code(r#"
trait Foo {
    async fn pet_toby() {
        println!("petting toby because he is a good boi");
    }
}"#)?;

    let generated = format_code(scope.to_string())?;

    assert_eq!(generated, expected);
    Ok(())
}

#[test]
fn trait_with_macros() -> io::Result<()> {
    let mut scope = Scope::new();
    let trt = scope.new_trait("Foo");
    trt.push_attr("#[async_trait]");
    trt.push_attr("#[toby_is_cute]");

    let f = trt.new_fn("pet_toby");
    f.set_async(true);
    f.line("println!(\"petting toby because he is a good boi\");");

    let expected = format_code(r#"
#[async_trait]
#[toby_is_cute]
trait Foo {
    async fn pet_toby() {
        println!("petting toby because he is a good boi");
    }
}"#)?;

    let generated = format_code(scope.to_string())?;

    assert_eq!(generated, expected);
    Ok(())
}

#[test]
fn impl_with_macros() -> io::Result<()> {
    let mut scope = Scope::new();
    scope.new_struct("Bar");
    let imp = scope.new_impl("Bar");
    imp.impl_trait("Foo");
    imp.push_attr("#[async_trait]");
    imp.push_attr("#[toby_is_cute]");

    let f = imp.new_fn("pet_toby");
    f.set_async(true);
    f.line("println!(\"petting Toby many times because he is such a good boi\");");

    let expected = format_code(r#"
struct Bar;

#[async_trait]
#[toby_is_cute]
impl Foo for Bar {
    async fn pet_toby() {
        println!("petting Toby many times because he is such a good boi");
    }
}"#)?;

    let generated = format_code(scope.to_string())?;

    assert_eq!(generated, expected);
    Ok(())
}

#[test]
fn struct_with_multiple_allow() -> io::Result<()> {
    let mut scope = Scope::new();

    scope.new_struct("Foo")
        .allow("dead_code")
        .allow("clippy::all")
        .field("one", "u8")
        .field("two", "u8");

    let expected = format_code(r#"
#[allow(dead_code)]
#[allow(clippy::all)]
struct Foo {
    one: u8,
    two: u8,
}"#)?;

    let generated = format_code(scope.to_string())?;

    assert_eq!(generated, expected);
    Ok(())
}

#[test]
fn enum_with_multiple_allow() -> io::Result<()> {
    let mut scope = Scope::new();

    scope.new_enum("IpAddrKind")
        .allow("dead_code")
        .allow("clippy::all")
        .push_variant(Variant::new("V4"))
        .push_variant(Variant::new("V6"));

    let expected = format_code(r#"
#[allow(dead_code)]
#[allow(clippy::all)]
enum IpAddrKind {
    V4,
    V6,
}"#)?;

    let generated = format_code(scope.to_string())?;

    assert_eq!(generated, expected);
    Ok(())
}
