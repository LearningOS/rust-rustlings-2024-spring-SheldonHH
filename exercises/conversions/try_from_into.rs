use std::convert::{TryFrom, TryInto};
// 在你提供的代码中，你需要完整实现 TryFrom trait，用于从不同类型的数据（如元组、数组和切片）安全转换到 Color 结构体。
// 这里的关键是要确保每个元素都在有效的颜色值范围内（0到255），并且对于切片还要检查长度是否正确。以下是针对元组、数组和切片的 
#[derive(Debug, PartialEq)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

#[derive(Debug, PartialEq)]
enum IntoColorError {
    BadLen,
    IntConversion,
}

// 实现元组转换
impl TryFrom<(i16, i16, i16)> for Color {
    type Error = IntoColorError;

    fn try_from(tuple: (i16, i16, i16)) -> Result<Self, Self::Error> {
        if tuple.0 < 0 || tuple.1 < 0 || tuple.2 < 0 || tuple.0 > 255 || tuple.1 > 255 || tuple.2 > 255 {
            Err(IntoColorError::IntConversion)
        } else {
            Ok(Color {
                red: tuple.0 as u8,
                green: tuple.1 as u8,
                blue: tuple.2 as u8,
            })
        }
    }
}

// 实现数组转换
impl TryFrom<[i16; 3]> for Color {
    type Error = IntoColorError;

    fn try_from(arr: [i16; 3]) -> Result<Self, Self::Error> {
        if arr[0] < 0 || arr[1] < 0 || arr[2] < 0 || arr[0] > 255 || arr[1] > 255 || arr[2] > 255 {
            Err(IntoColorError::IntConversion)
        } else {
            Ok(Color {
                red: arr[0] as u8,
                green: arr[1] as u8,
                blue: arr[2] as u8,
            })
        }
    }
}

// 实现切片转换
impl TryFrom<&[i16]> for Color {
    type Error = IntoColorError;

    fn try_from(slice: &[i16]) -> Result<Self, Self::Error> {
        if slice.len() != 3 {
            Err(IntoColorError::BadLen)
        } else if slice[0] < 0 || slice[1] < 0 || slice[2] < 0 || slice[0] > 255 || slice[1] > 255 || slice[2] > 255 {
            Err(IntoColorError::IntConversion)
        } else {
            Ok(Color {
                red: slice[0] as u8,
                green: slice[1] as u8,
                blue: slice[2] as u8,
            })
        }
    }
}

fn main() {
    let c1 = Color::try_from((183, 65, 14));
    println!("{:?}", c1);

    let c2: Result<Color, _> = [183, 65, 14].try_into();
    println!("{:?}", c2);

    let v = vec![183, 65, 14];
    let c3 = Color::try_from(&v[..]);
    println!("{:?}", c3);

    let c4: Result<Color, _> = (&v[..]).try_into();
    println!("{:?}", c4);
}
