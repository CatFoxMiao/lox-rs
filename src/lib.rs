mod scanner;
mod token;
use crate::scanner::Scanner; // 导入具体类型

// 使用示例
fn example() {
    let scanner = Scanner::new("hello world".to_String());
}
