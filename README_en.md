# AutoScript

**This project is still under development.**

`AutoScript` is a interpreter aims to execute `AutoScript` script file, including a lexer, parser, code generator and virtual machine.

`AutoScript` is a static type language. Due to my knowledge limited, code analysis maybe can't find errors in compile period. I'm still learning for implementing a better analysis.

If there is no exceptions, this project will add async-runtime coroutine and some standard lib. There are two main ideas of creating this project,
one is creating a script language for speeding up some work on information follow my mind, and another is the practice of learning *Principle of Compiler*

## What does it look like:

```
fn main(){
    val pi = 3.14159;
    val result = if pi > 30 {
        true;
    } elif pi < 30 {
        false;
    } else {
        false;
    }
    return;
}

fn add(a: int, b: int) -> int {
    return a + b;
}
```

