"use client";

import { useEffect, useState } from "react";

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
  const response = await fetch(`http://localhost:8080/correct?question_index=${questionIndex}&difficulty=${difficulty}&answer_index=${answerIndex}`);
  return response.json();
}

const QuestionDisplayer = (props: Props) => {
  const [index, setIndex] = useState(0);
  const [selectedAnswerIndex, setSelectedAnswerIndex] = useState<number | null>(null);
  const [correctAnswers, setCorrectAnswers] = useState(0);
  const [isCorrect, setIsCorrect] = useState<boolean | null>(null);

  const indexHandler = () => {
    if (index === props.questions.length - 1) {
      setSelectedAnswerIndex(null);
      setIsCorrect(null);
      setIndex(0);
      return;
    }
    setSelectedAnswerIndex(null);
    setIsCorrect(null);
    setIndex(index + 1);
  }

  useEffect(() => {
    if (isCorrect) {
      console.log("correct answers now will be: " + (correctAnswers + 1));
      setCorrectAnswers(correctAnswers + 1);
      return
    }
  }, [isCorrect])
  
  const correctHandler = async (difficulty: string, questionIndex: number, answerIndex: number) => {
    const res = await correct(difficulty, answerIndex, questionIndex);
    setSelectedAnswerIndex(answerIndex); 
    if (res === true) {
      setIsCorrect(true);
      setTimeout(() => indexHandler(), 1000);
      return;
    }
    setIsCorrect(false);
    setTimeout(() => indexHandler(), 1000);
  }

  const accentColor = accents[index % accents.length];

  return (
    <div className="flex justify-center items-center flex-col w-full h-full">
      <div className="p-4 text-center w-full min-h-36 flex justify-center items-center">
        <h3 className={`pt text-2xl border rounded p-2 bc w-full`}>
          {props.questions[index].question}
        </h3>
      </div>

      <div className="grid grid-cols-2 gap-3 grow w-full">
        {props.questions[index].answers.map((answer, answerIndex) => (
          <button
            key={answerIndex}
            className={`text-black pt m-3 rounded ${accentColor} ${
            selectedAnswerIndex === answerIndex ? isCorrect ? "bg-gradient-to-r from-green-500 to-green-700 transition animate-pulse" :
            "bg-gradient-to-r from-red-500 to-red-700 transition animate-pulse" : ""}`}
            onClick={async () => await correctHandler(props.difficulty, index, answerIndex)}
          >
            {answer}
          </button>
        ))}
      </div>
    </div>
  );
}

export default QuestionDisplayer;

