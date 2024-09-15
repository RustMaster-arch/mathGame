"use client";

import { useState } from "react";

interface Question {
  question: string,
  answers: string[],
}

interface Props {
  questions: Question[],
}

const accents = [
  "bg-[#00A8E8]", 
  "bg-[#9A57D3]",
  "bg-[#4CAF50]",
  "bg-[#FF8C00]",
];

async function correct(difficulty: string): Promise<boolean> {
  const response = await fetch(`http://localhost:8080/${difficulty}`, {
    method: "GET",
  });

  return response.json();
}

const QuestionDisplayer = (props: Props) => {
  const [index, setIndex] = useState(0);

  const indexHandler = () => {
    if (index === props.questions.length - 1) {
      setIndex(0);
      return;
    }
    setIndex(index + 1);
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
          className={`text-black pt m-3 w-full rounded bc ${accentColor}`}
        >
          {answer}
        </button>
      ))}
      <div className="mt-4">
        <button 
          className="bg-blue-500 text-white p-2 rounded"
          onClick={indexHandler}
        >
          Next Question
        </button>
      </div>
    </div>
  );
}

export default QuestionDisplayer;

