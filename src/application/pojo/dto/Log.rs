use std::fmt::Display;

pub struct Log {
    className: String,
    methodName: String,
    paramName: String,
    value: Box<dyn Display>,
}

impl Log {
    fn new(className: String, methodName: String, paramName: String, value: Box<dyn Display>) -> Self {
        Self { className, methodName, paramName, value }
    }

    pub fn of(className: &str, methodName: &str, paramName: &str, value: Box<dyn Display>) -> Log {
        Log::new(className.to_string(), methodName.to_string(), paramName.to_string(), value)
    }
}

impl Log {
    pub fn setClassName(&mut self, className: String) {
        self.className = className;
    }

    pub fn setMethodName(&mut self, methodName: String) {
        self.methodName = methodName;
    }

    pub fn setParamName(&mut self, paramName: String) {
        self.paramName = paramName;
    }

    pub fn setValue(&mut self, value: Box<dyn Display>) {
        self.value = value;
    }
}

impl Log {
    pub fn className(&self) -> &str {
        &self.className
    }

    pub fn methodName(&self) -> &str {
        &self.methodName
    }

    pub fn paramName(&self) -> &str {
        &self.paramName
    }

    pub fn value(&self) -> &Box<dyn Display> {
        &self.value
    }
}