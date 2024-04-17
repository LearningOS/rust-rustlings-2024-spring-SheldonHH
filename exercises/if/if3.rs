// if3.rs
//
// Execute `rustlings hint if3` or use the `hint` watch subcommand for a hint.
// 根据动物名称返回其栖息地的函数
pub fn animal_habitat(animal: &str) -> &'static str {
    // 使用整数代码标识每种动物
    let identifier = if animal == "crab" {
        1
    } else if animal == "gopher" {
        2
    } else if animal == "snake" {
        3
    } else {
        0 // 未知动物使用0表示
    };

    // DO NOT CHANGE THIS STATEMENT BELOW
    // 根据动物标识符返回对应的栖息地
    let habitat = if identifier == 1 {
        "Beach"
    } else if identifier == 2 {
        "Burrow"
    } else if identifier == 3 {
        "Desert"
    } else {
        "Unknown"
    };

    habitat
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gopher_lives_in_burrow() {
        assert_eq!(animal_habitat("gopher"), "Burrow")
    }

    #[test]
    fn snake_lives_in_desert() {
        assert_eq!(animal_habitat("snake"), "Desert")
    }

    #[test]
    fn crab_lives_on_beach() {
        assert_eq!(animal_habitat("crab"), "Beach")
    }

    #[test]
    fn unknown_animal() {
        assert_eq!(animal_habitat("dinosaur"), "Unknown")
    }
}
