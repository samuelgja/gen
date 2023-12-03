pub const TEMPLATE_FILE_CONTENT: &str = r#"
## Quick start:

 1. Remove this part section.
 2. Edit, save & that's it!
 
### Using variables:
 1. Basic -> __var__
 2. Basic with custom naming -> __var___some
 4. Variables also supports case type prefix -> __var___kebab__, __var___pascal__, __var___camel__, __var___snake__ or __var___my_name_snake
 5. For dropdowns selections -> #select
 6. It's same as __var__, expect that template folder contain config json which should contains array of available values.
 7. Same variable rules is applied for template paths as well.

#### Example rust:
```rust
   pub struct __var__;

   fn main () {
        let __var___snake__ = __var__ {};
   }
   
```

#### Example go:
```go
   type __var__ struct {}

   func main () {
        __var___camel__ := __var__{}
   }
   
```

#### Example typescript:
```typescript
   class __var__ {
        constructor () {}
   }
   
   const __var___camel__ = new __var__();
   
```

#### Example python:
```python
   class __var__:
        def __init__(self):
            pass
   
   __var___snake__ = __var__()
   
```

#### Example java:
```java
   public class __var__ {
        public __var__ () {}
   }
   
   var __var___pascal__ = new __var__();
   
```

#### Example php:
```php
   class __var__ {
        public function __construct() {}
   }
   
   $__var___snake__ = new __var__();
   
```
etc...
"#;
