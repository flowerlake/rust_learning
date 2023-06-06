## Learning Timeline

### 20230606
- crate lib
libs/rary 目录下是一个完整的cargo项目，编译得到library.rlib,
rary2.rs 是一个单文件，使用`rustc .\rary2.rs --crate-type=lib`编译得到library2.rlib
在main.rs加入一个test_other_lib函数用于测试调用第三方二进制依赖包的函数等。
然后使用--extern参数进行编译即可。
```shell
rustc .\main.rs --extern rary2=library2.rlib --extern rary=library.rlib
```
-cfg
**注意**条件编译时，当前环境不满足条件编译的时候，里面的重复代码\错误代码不会被检测到

### 20230604
- match pattern-解构、谓语句、绑定
- enum struct
- mod、cargo.toml的使用
```
// mod enum_test 会查找名为 `enum_test.rs` 或 `enum_test/mod.rs` 的文件，模块的两种方式
mod enum_test;
```
通过查看rustdesk的源码，它的main.rs文件中没有引入任何的mod，所有的mod引用全都放在了lib.rs文件中,这一点还不知道如何实现的

- rust结构体，方法，函数
rust中调用方法使用`.`，调用函数使用`::`，方法是需要有self字段的fn，函数是没有self字段的任意fn。