pub trait DisplayGrade {
    fn display(&self) -> String;
}

// 为浮点数成绩实现 DisplayGrade 特性
impl DisplayGrade for f32 {
    fn display(&self) -> String {
        self.to_string()
    }
}

// 为字符串成绩实现 DisplayGrade 特性
impl DisplayGrade for &str {
    fn display(&self) -> String {
        self.to_string()
    }
}

// 泛型化的 ReportCard 结构体
pub struct ReportCard<T: DisplayGrade> {
    pub grade: T,
    pub student_name: String,
    pub student_age: u8,
}

impl<T: DisplayGrade> ReportCard<T> {
    pub fn print(&self) -> String {
        format!("{} ({}) - achieved a grade of {}",
            &self.student_name, &self.student_age, self.grade.display())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_numeric_report_card() {
        let report_card = ReportCard {
            grade: 2.1,
            student_name: "Tom Wriggle".to_string(),
            student_age: 12,
        };
        assert_eq!(
            report_card.print(),
            "Tom Wriggle (12) - achieved a grade of 2.1"
        );
    }

    #[test]
    fn generate_alphabetic_report_card() {
        let report_card = ReportCard {
            grade: "A+",
            student_name: "Gary Plotter".to_string(),
            student_age: 11,
        };
        assert_eq!(
            report_card.print(),
            "Gary Plotter (11) - achieved a grade of A+"
        );
    }
}
