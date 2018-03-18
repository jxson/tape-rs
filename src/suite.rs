use test::Test;
use failure::Error;
use std::collections::VecDeque;

pub struct Suite {
    tests: VecDeque<Test>,
}

impl Suite {
    pub fn new() -> Suite {
        Suite {
            tests: VecDeque::new(),
        }
    }

    // pub fn add<S, F: Fn(&Test) + 'static> (&mut self, name: S, callback: F) where S: Into<String> {

    pub fn test<S, F: Fn(&Test) + 'static>() -> impl FnMut(S, F) where S: Into<String> {
        let mut suite = Suite::new();

        move |name, callback| {
            suite.add(name, callback);
        }
    }

    pub fn add<S, F: Fn(&Test) + 'static> (&mut self, name: S, callback: F) where S: Into<String> {
        let t = Test::new(name, callback);
        self.tests.push_back(t);
    }

    pub fn run(&mut self) -> Result<(), Error> {
        // iterate over tests and execute each.
        // report status to harness.
        // catch errors and report failures.

        // suite has multiple tests, tests have multiple asserts.

        // convert test runs into reports.
        for t in self.tests.iter() {
            let res = t.run();
            if res.is_err() {
                return res;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::Suite;

    #[test]
    fn simple_runner() {
        let mut test = Suite::test();
        test("using the higher order function ", |t| {
            t.assert(true);
            t.end();
        });
    }

    #[test]
    fn suite_add() {
        let mut suite = Suite::new();
        suite.add("example", |t| {
            t.assert(true);
            t.end();
        });
        let res = suite.run();
        assert!(res.is_ok());
    }
}
