// modules1.rs
//
// Execute `rustlings hint modules1` or use the `hint` watch subcommand for a
// hint.

mod sausage_factory {
    // Don't let anybody outside of this module see this!
    fn get_secret_recipe() -> String {
        String::from("Ginger")
    }

    // 将 make_sausage 函数标记为公开
    pub fn make_sausage() {
        get_secret_recipe();  // 这个调用是模块内部的，所以没有问题
        println!("sausage!");
    }
}

fn main() {
    sausage_factory::make_sausage();  // 现在可以在模块外部调用这个函数
}
