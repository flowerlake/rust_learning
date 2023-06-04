## Learning Timeline
### 20230604
- match pattern-解构、谓语句、绑定
- enum struct
- mod、cargo.toml的使用
```rust
// mod enum_test 会查找名为 `enum_test.rs` 或 `enum_test/mod.rs` 的文件，模块的两种方式
mod enum_test;
```
通过查看rustdesk的源码，它的main.rs文件中没有引入任何的mod，所有的mod引用全都放在了lib.rs文件中,这一点还不知道如何实现的

- rust结构体，方法，函数
rust中调用方法使用`.`，调用函数使用`::`，方法是需要有self字段的fn，函数是没有self字段的任意fn。