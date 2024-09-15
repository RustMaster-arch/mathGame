"use client";

import { useState } from "react";

interface Question {
  question: string,
  answers: string[],
}

interface Props {
  questions: Question[],
  difficulty: string,
}

const accents = [
  "bg-[#00A8E8]", 
  "bg-[#9A57D3]",
  "bg-[#4CAF50]",
  "bg-[#FF8C00]",
];

async function correct(difficulty: string, answerIndex: number, questionIndex: number): Promise<boolean> {
  const response = await fetch(`http://localhost:8080/correct?question_index=${questionIndex}&difficulty=${difficulty}&answer_index=${answerIndex}`)

  return response.json()
}

const QuestionDisplayer = (props: Props) => {
  const [index, setIndex] = useState(0);
  const [isCorrect, setIsCorrect] = useState<boolean>();

  const indexHandler = () => {
    if (index === props.questions.length - 1) {
      setIsCorrect(false);
      setIndex(0);
      console.log("index: 0");
      return;
    }
    setIsCorrect(false);
    setIndex(index + 1);
    console.log("index: " + index + 1);
  }
  
  const correctHandler = async (difficulty: string, questionIndex: number, answerIndex: number) => {
    const res = await correct(difficulty, answerIndex, questionIndex);
    if (res === true) {
      setTimeout(() => indexHandler(), 2000)
      console.log("correct!")
      setIsCorrect(true)
      return
    }

    setTimeout(() => indexHandler(), 2000)
    console.log("incorrect!")
    setIsCorrect(false)
    return
  }

  const accentColor = accents[index % accents.length];

  return (
    <div className="min-h-screen p-4">
      <h3 className={`pt text-2xl border rounded  p-2 bc`}>
        {props.questions[index].question}
      </h3>
      {props.questions[index].answers.map((answer, answerIndex) => (
        <button
          key={answerIndex}
          className={`text-black pt m-3 w-full rounded ${accentColor} ${isCorrect ? "border-4 border-green-500" : ""}`}
          onClick={async () => await correctHandler(props.difficulty, index, answerIndex)}
        >
          {answer}
        </button>
      ))}
    </div>
  );
}

export default QuestionDisplayer;
