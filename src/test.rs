use failure::Error;

pub type TestCallback = Fn(&Test);

pub struct Test {
    pub name: String,
    callback: Box<TestCallback>,
}

impl Test {
    pub fn new<S, F: Fn(&Test) + 'static>(name: S, callback: F) -> Test where S: Into<String> {
        Test {
            name: name.into(),
            callback: Box::new(callback),
        }
    }

    pub fn assert(&self, value: bool) {
        if value != true {
            println!("failed {:?}", value);
        }
    }

    pub fn end(&self) {}

    // https://doc.rust-lang.org/std/panic/fn.catch_unwind.html
    pub fn run(&self) -> Result<(), Error> {
        let callback = &self.callback;
        callback(self);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::Test;

    #[test]
    fn new_test() {
        let t = Test::new("example", |t| {
            t.assert(true);
            t.end();
        });

        let res = t.run();
        assert!(res.is_ok());
    }
}
