pub const TEMPLATE_FILE_CONTENT: &str = r#"
## Quick start:

 1. Remove this part section.
 2. Edit, save & that's it!
 
### Using variables:
 1. Basic -> #var
 2. Basic with custom naming -> #var_some
 4. Variables also supports case type prefix -> #var_kebab, #var_pascal, #var_camel, #var_snake or #var_my_name_snake
 5. For dropdowns selections -> #select
 6. It's same as #var, expect that template folder contain config json which should contains array of available values.
 7. Same variable rules is applied for template paths as well.

#### Example rust:
```rust
   pub struct #var;

   fn main () {
        let #var_snake = #var {};
   }
   
```

#### Example go:
```go
   type #var struct {}

   func main () {
        #var_camel := #var{}
   }
   
```

#### Example typescript:
```typescript
   class #var {
        constructor () {}
   }
   
   const #var_camel = new #var();
   
```

#### Example python:
```python
   class #var:
        def __init__(self):
            pass
   
   #var_snake = #var()
   
```

#### Example java:
```java
   public class #var {
        public #var () {}
   }
   
   var #var_pascal = new #var();
   
```

#### Example php:
```php
   class #var {
        public function __construct() {}
   }
   
   $#var_snake = new #var();
   
```
etc...
"#;
