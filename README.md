#### Gen
Code templates easy.


#### Why
In many languages, projects there are situations when you repeat yourself with writing a same code again and again. It's basically not wrong, but managing templates for various situations in **IDE** sometimes can be difficult.
1. **UI** - When writing new component, basically all the time it follows the same pattern - this pattern can be specific for `personal` or `company` project
2. **Data models** - Each project has unique data models, it can use **different databases**, or even different `ORM` libraries so template can vary for specific project.
3. **Boiler places** - Also there are boiler places in code, that can be generated, like `main` function in `C` or `C++` projects, or `package.json` in `NodeJS` projects. 

### What this fixes
IDE templates work basically on just file. So for using template in your favorite IDE, file need to be created first (**manually**) and then template can be used.
**Gen** single template do not consist of single file, but rather composition of multiple files belong to same template.

### How it works
Idea how it works is pretty simple, 
1. In root project directory there is `.gen` directory, which consist of folders - **templates** and config json file - `_.json`. 
2. Template is represented as normal file with path in template directory where `path` and `template` itself can contain variables.
3. Template variables - There are two types of variables - `__var__` and `__select__`.

*So basically there is 3 main things to remember how it works: `path` to template, template `content` itself and template `variables`.*

### Example
Structure how it will looks like in `.gen/my-template` directory for simple UI template:
1. UI Main component 
    - path: `src/components/__var__/__var__.tsx`
    - content: 
    ```tsx
        import React from 'react';
        interface __var__Props {

        }
        export const __var__: React.FC<__var__Props> = () => {
            return (
                <div>
                    __var__
                </div>
            );
        };
    ```
2. UI Main component test
    - path: `src/components/__var__/__var__.test.tsx`
    - content: 
    ```tsx
        import React from 'react';
        import { render } from '@testing-library/react';
        import { __var__ } from './__var__';

        describe('__var__', () => {
            it('should render successfully', () => {
                const { baseElement } = render(<__var__ />);
                expect(baseElement).toBeTruthy();
            });
        });
    ```

3. Updating index file
    - path: `src/components/index.ts` - if in filename (end path) do not contain any variable, it will be in append mode.
    - content: 
    ```tsx
        export * from './__var__kebab__/__var__kebab__'; // variable support also casing - __var__kebab__, __var__camel__, __var__snake__, __var__pascal__
    ```


After run `gen use` and select this template, it will ask for variables and after that it will create files in project directory with variables replaced with values respecting defined casing.


### Commands
1. `gen` - initialize project and create `.gen` directory in root file and global one in `~/.gen`
2. `gen new` - cli wizard for creating new template
3. `gen use` - cli wizard for using template / select template
4. `gen help` - print help
4. `gen edit` - edit template
5. `gen variables` - print all variables for template

Use help command for show all commands and options.

- `-g` flag which will save and use templates from `~/.gen` instead of local `.gen` directory


*Note: Editing template can be also proceed manually via editing template **file** or **change template file path** in your favorite IDE or text editor.*

### Install
TODO...