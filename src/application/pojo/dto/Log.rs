use std::fmt::Display;

pub struct Log<T> {
    className: String,
    methodName: String,
    paramName: String,
    value: T,
}

impl<T> Log<T> {
    fn new(className: String, methodName: String, paramName: String, value: T) -> Self {
        Self { className, methodName, paramName, value }
    }

    pub fn of(className: &str, methodName: &str, paramName: &str, value: T) -> Log<T> {
        Log::new(className.to_string(), methodName.to_string(), paramName.to_string(), value)
    }
}

impl<T> Log<T> {
    pub fn setClassName(&mut self, className: String) {
        self.className = className;
    }

    pub fn setMethodName(&mut self, methodName: String) {
        self.methodName = methodName;
    }

    pub fn setParamName(&mut self, paramName: String) {
        self.paramName = paramName;
    }

    pub fn setValue(&mut self, value: T) {
        self.value = value;
    }
}

impl<T> Log<T> {
    pub fn className(&self) -> &str {
        &self.className
    }

    pub fn methodName(&self) -> &str {
        &self.methodName
    }

    pub fn paramName(&self) -> &str {
        &self.paramName
    }

    pub fn value(&self) -> &T {
        &self.value
    }
}