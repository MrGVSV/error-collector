use syn::Error;

#[derive(Default)]
pub struct ErrorCollector(Option<Error>);

impl ErrorCollector {
    pub fn new(error: Error) -> Self {
        Self(Some(error))
    }

    pub fn error(&self) -> Option<&Error> {
        self.0.as_ref()
    }

    pub fn error_mut(&mut self) -> Option<&mut Error> {
        self.0.as_mut()
    }

    pub fn take(&mut self) -> Option<Error> {
        self.0.take()
    }

    pub fn replace(&mut self, error: Error) -> Option<Error> {
        self.0.replace(error)
    }

    pub fn combine(&mut self, error: Error) {
        match self.0 {
            None => self.0 = Some(error),
            Some(ref mut prev) => prev.combine(error),
        }
    }

    pub fn try_throw(self) -> Result<(), Error> {
        match self.0 {
            None => Ok(()),
            Some(err) => Err(err),
        }
    }
}

#[cfg(test)]
mod tests {
    use syn::__private::Span;
    use super::*;

    #[test]
    fn should_combine() {
        let mut collector = ErrorCollector::new(Error::new(Span::call_site(), "First Error"));
        collector.combine(Error::new(Span::call_site(), "Second Error"));

        let expected = r#"compile_error ! { "First Error" } compile_error ! { "Second Error" }"#;
        let received = collector.try_throw().expect_err("expected error").to_compile_error().to_string();
        assert_eq!(expected, received);
    }

    #[test]
    fn should_replace() {
        let mut collector = ErrorCollector::new(Error::new(Span::call_site(), "First Error"));
        let existing = collector.replace(Error::new(Span::call_site(), "Second Error"));

        let expected = r#"compile_error ! { "First Error" }"#;
        let received = existing.expect("expected error").to_compile_error().to_string();
        assert_eq!(expected, received);

        let expected = r#"compile_error ! { "Second Error" }"#;
        let received = collector.try_throw().expect_err("expected error").to_compile_error().to_string();
        assert_eq!(expected, received);
    }

    #[test]
    fn should_take() {
        let mut collector = ErrorCollector::new(Error::new(Span::call_site(), "First Error"));
        let existing = collector.take();

        let expected = r#"compile_error ! { "First Error" }"#;
        let received = existing.expect("expected error").to_compile_error().to_string();
        assert_eq!(expected, received);
        assert!(collector.try_throw().is_ok());
    }
}
