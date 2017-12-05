extern crate chakracore as js;

fn main() {
    let runtime = js::Runtime::new().unwrap();
    let context = js::Context::new(&runtime).unwrap();
    let guard = context.make_current().unwrap();

    let result = js::script::eval(&guard, "5 + 1").unwrap();
    assert_eq!(result.to_integer(&guard), 10);
}
