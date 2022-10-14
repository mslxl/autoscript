# AutoScript

**本项目目前仍在施工**

`AutoScript` 是一个执行对应语言的解释器, 包含词法分析器，语法分析器，代码生成器和一个虚拟机。

`AutoScript` 也是一个静态类型的语言, 但是由于个人能力的限制，目前有些错误不能在编译期被发现。我会持续的学习并改进这一方面的能力。

如果不出意外的话，本项目未来可能会添加一个协程库和一些标准库。

创建这个项目的初衷有两点，一是尽可能的实现电脑上的重复工作的自动化（尽管这可能有重复造轮子的嫌疑），另一点是对个人学习成果的检验。

## 例子
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