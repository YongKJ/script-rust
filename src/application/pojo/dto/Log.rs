use std::any::Any;

pub struct Log {
    className: String,
    methodName: String,
    paramName: String,
    value: Box<dyn Any>,
}

impl Log {
    fn new(className: String, methodName: String, paramName: String, value: Box<dyn Any>) -> Self {
        Self { className, methodName, paramName, value }
    }

    pub fn of(className: &str, methodName: &str, paramName: &str, value: Box<dyn Any>) -> Log {
        Log::new(String::from(className), String::from(methodName), String::from(paramName), value)
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

    pub fn setValue(&mut self, value: Box<dyn Any>) {
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

    pub fn value(&self) -> &Box<dyn Any> {
        &self.value
    }
}