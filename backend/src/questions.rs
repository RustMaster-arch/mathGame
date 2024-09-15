use crate::structs::Question;

pub fn easy() -> Vec<Question> {
    vec![
        Question::new("What is 5 + 3?", vec!["6", "7", "8", "9"], 2),
        Question::new("What is 10 - 4?", vec!["5", "6", "7", "8"], 1),
        Question::new("What is 6 * 2?", vec!["10", "11", "12", "13"], 2),
        Question::new("What is 15 / 3?", vec!["3", "4", "5", "6"], 2),
        Question::new("What is 9 + 6?", vec!["13", "14", "15", "16"], 2),
    ]
}

pub fn medium() -> Vec<Question> {
    vec![
        Question::new("What is the value of 7 * 8?", vec!["48", "54", "56", "63"], 2),
        Question::new("What is the square root of 144?", vec!["10", "12", "14", "16"], 1),
        Question::new("What is 25% of 200?", vec!["25", "40", "50", "75"], 2),
        Question::new("What is the value of 9^2?", vec!["64", "72", "81", "99"], 2),
        Question::new("What is the value of 5^3?", vec!["100", "120", "125", "150"], 2),
    ]
}

pub fn hard() -> Vec<Question> {
    vec![
        Question::new("What is the derivative of x^3 + 5x^2 - 2x + 1?", vec!["3x^2 + 10x - 2", "x^2 + 2x - 2", "4x^2 + 10x - 2", "6x^2 + 10x + 2"], 0),
        Question::new("What is the value of the integral of e^x from 0 to 1?", vec!["e - 1", "1", "e", "0"], 0),
        Question::new("Solve the equation: 2^(x+1) = 16", vec!["x = 3", "x = 2", "x = 4", "x = 5"], 0),
        Question::new("What is the limit of (x^2 - 1)/(x - 1) as x approaches 1?", vec!["2", "0", "3", "1"], 3),
        Question::new("If f(x) = 2x^2 + 3x, what is f'(x)?", vec!["4x + 3", "2x + 3", "4x + 6", "x + 3"], 0),
    ]
}

pub fn very_hard() -> Vec<Question> {
    vec![
        Question::new("What is the integral of e^(2x)?", vec!["e^(2x) + C", "e^(2x)/2 + C", "2e^(2x) + C", "e^(2x)/4 + C"], 1),
        Question::new("What is the limit of (x^2 - 1)/(x - 1) as x approaches 1?", vec!["0", "1", "2", "Undefined"], 2),
        Question::new("What is the solution to the differential equation y' + y = 0?", vec!["y = e^x", "y = e^(-x)", "y = x^2", "y = -x"], 1),
        Question::new("What is the determinant of the matrix [[1, 2], [3, 4]]?", vec!["-2", "2", "4", "-4"], 0),
        Question::new("What is the derivative of ln(x)?", vec!["1/x", "x", "ln(x)/x", "x^(-1)"], 0),
    ]
}
