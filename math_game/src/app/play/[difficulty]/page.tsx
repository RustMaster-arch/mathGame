import QuestionDisplayer from "@/components/QuestionDisplayer";

interface Questions {
  question: string,
  answers: string[],
}

async function questions(difficulty: string): Promise<Questions[]> {
  const response = await fetch(`http://localhost:8080/${difficulty}`, {
    cache: 'no-store',
    method: "GET",
  });

  return response.json();
}

const page = async ({ params }: {params: {difficulty: string}}) => {
  const res = await questions(params.difficulty);
  console.log(res);

  return (
    <>
      <div className="some flex flex-col h-full">
          <QuestionDisplayer questions={res} difficulty={params.difficulty}/>
      </div>
    </>
  )
}

export default page
