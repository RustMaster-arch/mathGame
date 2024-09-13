use super::structs::Question;

impl Question {
    fn add_question(&mut self, question: &str) {
        self.question = question.to_string();
    }

    fn add_answers(&mut self, answers: Vec<&str>) {
        answers.iter().for_each(|answer| self.answers.push(answer.to_string()));
    }

    fn add_correct_answer(&mut self, correct: &str) {
        self.correct = correct.to_string();
    }
}

pub fn easy() -> Vec<Question> {
    let mut q1 = Question { ..Default::default()};
    let mut q2 = Question { ..Default::default()};
    let mut q3 = Question { ..Default::default()};
    let mut q4 = Question { ..Default::default()};
    let mut q5 = Question { ..Default::default()};

    q1.add_question("What is 7 + 5?");
    q1.add_answers(vec!["10", "11", "12", "13"]);
    q1.add_correct_answer("12");

    q2.add_question("What is 9 - 4?");
    q2.add_answers(vec!["2", "5", "6", "7"]);
    q2.add_correct_answer("5");

    q3.add_question("What is 6 * 3?");
    q3.add_answers(vec!["9", "12", "15", "18"]);
    q3.add_correct_answer("18");

    q4.add_question("What is 20 ÷ 4?");
    q4.add_answers(vec!["4", "5", "6", "7"]);
    q4.add_correct_answer("5");

    q5.add_question("What is 2^3?");
    q5.add_answers(vec!["6", "7", "8", "9"]);
    q5.add_correct_answer("8");

    vec![q1, q2, q3, q4, q5]
}

pub fn meddium() -> Vec<Question> {
    let mut q1 = Question { ..Default::default()};
    let mut q2 = Question { ..Default::default()};
    let mut q3 = Question { ..Default::default()};
    let mut q4 = Question { ..Default::default()};
    let mut q5 = Question { ..Default::default()};

    q1.add_question("What is the greatest common divisor (GCD) of 24 and 36?");
    q1.add_answers(vec!["6", "8", "12", "18"]);
    q1.add_correct_answer("12");

    q2.add_question("What is the value of x in the equation 2x + 5 = 17?");
    q2.add_answers(vec!["5", "6", "7", "8"]);
    q2.add_correct_answer("6");

    q3.add_question("What is the area of a triangle with a base of 10 and height of 5?");
    q3.add_answers(vec!["20", "25", "30", "35"]);
    q3.add_correct_answer("25");

    q4.add_question("Solve for y: 3y - 4 = 2y + 6.");
    q4.add_answers(vec!["-2", "2", "4", "10"]);
    q4.add_correct_answer("10");

    q5.add_question("What is the square root of 169?");
    q5.add_answers(vec!["11", "12", "13", "14"]);
    q5.add_correct_answer("13");

    vec![q1, q2, q3, q4, q5]
}

pub fn hard() -> Vec<Question> {
    let mut q1 = Question { ..Default::default()};
    let mut q2 = Question { ..Default::default()};
    let mut q3 = Question { ..Default::default()};
    let mut q4 = Question { ..Default::default()};
    let mut q5 = Question { ..Default::default()};

    q1.add_question("What is the solution to the quadratic equation x^2 - 5x + 6 = 0?");
    q1.add_answers(vec!["x = 2, x = 3", "x = -2, x = -3", "x = 1, x = 5", "x = 0, x = 6"]);
    q1.add_correct_answer("x = 2, x = 3");

    q2.add_question("What is the derivative of f(x) = 3x^3 - 4x^2 + 2x - 1?");
    q2.add_answers(vec!["9x^2 - 8x + 2", "6x^2 - 8x + 2", "9x^2 - 4x + 1", "3x^2 - 4x + 2"]);
    q2.add_correct_answer("9x^2 - 8x + 2");

    q3.add_question("What is the value of the determinant of the matrix [[3, 8], [4, 6]]?");
    q3.add_answers(vec!["-14", "-10", "12", "14"]);
    q3.add_correct_answer("-14");

    q4.add_question("Evaluate the limit: lim (x -> 0) (sin(x)/x).");
    q4.add_answers(vec!["0", "1", "∞", "Undefined"]);
    q4.add_correct_answer("1");

    q5.add_question("If log2(x) = 5, what is the value of x?");
    q5.add_answers(vec!["10", "16", "32", "64"]);
    q5.add_correct_answer("32");

    vec![q1, q2, q3, q4, q5]
}

pub fn very_hard() -> Vec<Question> {
    let mut q1 = Question { ..Default::default()};
    let mut q2 = Question { ..Default::default()};
    let mut q3 = Question { ..Default::default()};
    let mut q4 = Question { ..Default::default()};
    let mut q5 = Question { ..Default::default()};

    q1.add_question("What is the solution to the differential equation dy/dx = y^2 with initial condition y(0) = 1?");
    q1.add_answers(vec!["y = -1/(x + 1)", "y = 1/(x + 1)", "y = e^x", "y = ln(x)"]);
    q1.add_correct_answer("y = 1/(x + 1)");

    q2.add_question("Evaluate the improper integral: ∫ (1/√x) dx from 1 to ∞.");
    q2.add_answers(vec!["2", "∞", "1", "Diverges"]);
    q2.add_correct_answer("Diverges");

    q3.add_question("What is the Taylor series expansion of e^x around x = 0 up to the 4th degree?");
    q3.add_answers(vec!["1 + x + x^2/2 + x^3/6 + x^4/24", "1 + x + x^2 + x^3 + x^4", "x + x^2 + x^3/3 + x^4/24", "1 + x + x^2/2 + x^3/3 + x^4/12"]);
    q3.add_correct_answer("1 + x + x^2/2 + x^3/6 + x^4/24");

    q4.add_question("Solve the system of equations using matrix inversion: 2x + 3y = 5 and 4x - y = 3.");
    q4.add_answers(vec!["x = 2, y = -1", "x = 1, y = 1", "x = 0, y = 1", "x = -1, y = 2"]);
    q4.add_correct_answer("x = 1, y = 1");

    q5.add_question("What is the value of ζ(2), the Riemann zeta function at s = 2?");
    q5.add_answers(vec!["π^2 / 6", "π / 2", "π^2 / 4", "π^2"]);
    q5.add_correct_answer("π^2 / 6");

    vec![q1, q2, q3, q4, q5]
}
