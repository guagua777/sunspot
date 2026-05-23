mod acir_field;
mod black_box_func;
mod expression;
mod opcodes;
mod shared;

fn main() {
    // 打印当前的路径
    println!("Current path: {:?}", std::env::current_dir());
    let target_dir = "../go/binaries/";

    // 这段代码初始化了一个日志订阅器（tracing subscriber），逐行解释：
    // tracing_subscriber::fmt() — 创建一个格式化日志订阅器，将日志输出为人类可读的文本格式。
    // .with_max_level(tracing::Level::TRACE) — 设置最大日志级别为 TRACE（最详细），会输出所有级别：TRACE > DEBUG > INFO > WARN > ERROR。
    // .with_target(false) — 不显示日志来源模块路径（默认会显示类似 testgen::main 这样的前缀）。
    // .with_writer(std::io::stdout) — 将日志输出到标准输出（stdout），而不是 stderr（tracing 默认是 stderr）。
    // .init() — 将这个订阅器注册为全局默认订阅器，之后代码中所有 tracing::info!、tracing::debug! 等宏的输出都会走这里。
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .with_target(false)
        .with_writer(std::io::stdout)
        .init();

    // check if the directory exists
    if !std::path::Path::new(target_dir).exists() {
        std::fs::create_dir_all(target_dir).expect("Failed to create directory");
    }

    black_box_func::generate_tests(target_dir);
    shared::generate_tests(target_dir);
    acir_field::generate_tests(target_dir);
    expression::generate_tests(target_dir);
    opcodes::generate_tests(target_dir);
}
