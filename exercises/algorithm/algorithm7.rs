/*
	stack
	This question requires you to use a stack to achieve a bracket match
*/

// 定义泛型结构体`Stack`，用于存储元素
#[derive(Debug)]
struct Stack<T> {
    size: usize, // 栈当前的大小
    data: Vec<T>, // 使用Vec来存储栈中的元素
}

impl<T> Stack<T> {
    // 新建一个空栈
    fn new() -> Self {
        Self {
            size: 0,
            data: Vec::new(), // Rust中的Vec类似于C++的std::vector
        }
    }

    // 检查栈是否为空
    fn is_empty(&self) -> bool {
        0 == self.size
    }

    // 获取栈的长度
    fn len(&self) -> usize {
        self.size
    }

    // 清空栈
    fn clear(&mut self) {
        self.size = 0;
        self.data.clear(); // Vec的clear方法移除所有元素但不回收内存
    }

    // 向栈中添加一个元素
    fn push(&mut self, val: T) {
        self.data.push(val); // Vec的push方法在末尾添加一个元素
        self.size += 1;
    }

    // 从栈中移除最后一个元素并返回它
    fn pop(&mut self) -> Option<T> {
        if self.size > 0 {
            self.size -= 1;
            self.data.pop() // Vec的pop方法移除并返回最后一个元素
        } else {
            None
        }
    }

    // 查看栈顶元素而不移除它
    fn peek(&self) -> Option<&T> {
        if 0 == self.size {
            None
        } else {
            self.data.get(self.size - 1) // Vec的get方法返回指定位置的引用
        }
    }

    // 查看栈顶元素的可变引用而不移除它
    fn peek_mut(&mut self) -> Option<&mut T> {
        if 0 == self.size {
            None
        } else {
            self.data.get_mut(self.size - 1) // Vec的get_mut方法返回指定位置的可变引用
        }
    }
	fn into_iter(self) -> IntoIter<T> {
		IntoIter(self)
	}
	fn iter(&self) -> Iter<T> {
		let mut iterator = Iter { 
			stack: Vec::new() 
		};
		for item in self.data.iter() {
			iterator.stack.push(item);
		}
		iterator
	}
	fn iter_mut(&mut self) -> IterMut<T> {
		let mut iterator = IterMut { 
			stack: Vec::new() 
		};
		for item in self.data.iter_mut() {
			iterator.stack.push(item);
		}
		iterator
	}
}
struct IntoIter<T>(Stack<T>);
impl<T: Clone> Iterator for IntoIter<T> {
	type Item = T;
	fn next(&mut self) -> Option<Self::Item> {
		if !self.0.is_empty() {
			self.0.size -= 1;self.0.data.pop()
		} 
		else {
			None
		}
	}
}
struct Iter<'a, T: 'a> {
	stack: Vec<&'a T>,
}
impl<'a, T> Iterator for Iter<'a, T> {
	type Item = &'a T;
	fn next(&mut self) -> Option<Self::Item> {
		self.stack.pop()
	}
}
struct IterMut<'a, T: 'a> {
	stack: Vec<&'a mut T>,
}
impl<'a, T> Iterator for IterMut<'a, T> {
	type Item = &'a mut T;
	fn next(&mut self) -> Option<Self::Item> {
		self.stack.pop()
	}
}

fn bracket_match(bracket: &str) -> bool {
    let mut stack = Stack::new();
    for c in bracket.chars() {
        match c {
            '(' | '{' | '[' => stack.push(c),
            ')' => if stack.pop() != Some('(') { return false; },
            '}' => if stack.pop() != Some('{') { return false; },
            ']' => if stack.pop() != Some('[') { return false; },
            _ => (),
        }
    }
    stack.is_empty()
}



#[cfg(test)]
mod tests {
	use super::*;
	
	#[test]
	fn bracket_matching_1(){
		let s = "(2+3){func}[abc]";
		assert_eq!(bracket_match(s),true);
	}
	#[test]
	fn bracket_matching_2(){
		let s = "(2+3)*(3-1";
		assert_eq!(bracket_match(s),false);
	}
	#[test]
	fn bracket_matching_3(){
		let s = "{{([])}}";
		assert_eq!(bracket_match(s),true);
	}
	#[test]
	fn bracket_matching_4(){
		let s = "{{(}[)]}";
		assert_eq!(bracket_match(s),false);
	}
	#[test]
	fn bracket_matching_5(){
		let s = "[[[]]]]]]]]]";
		assert_eq!(bracket_match(s),false);
	}
	#[test]
	fn bracket_matching_6(){
		let s = "";
		assert_eq!(bracket_match(s),true);
	}
}