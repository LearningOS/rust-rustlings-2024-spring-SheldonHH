#[derive(Debug)]
enum Message {
    // 定义不同的消息类型，每种类型可能携带不同的数据
    Move { x: i32, y: i32 },        // 消息类型为Move，包含两个整数类型的数据x和y
    Echo(String),                   // 消息类型为Echo，包含一个字符串
    ChangeColor(i32, i32, i32),     // 消息类型为ChangeColor，包含三个整数类型的数据，代表颜色的RGB值
    Quit,                           // 消息类型为Quit，无数据载荷
}

impl Message {
    fn call(&self) {
        println!("{:?}", self);
    }
}

fn main() {
    // 创建一个包含各种消息的数组
    let messages = [
        Message::Move { x: 10, y: 30 },
        Message::Echo(String::from("hello world")),
        Message::ChangeColor(200, 255, 255),
        Message::Quit,
    ];

    // 遍历消息数组，并打印每个消息
    for message in &messages {
        message.call();
    }
}
