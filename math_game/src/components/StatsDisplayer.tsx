interface Props {
  correctAnswers: number,
}

const StatsDisplayer = (props: Props) => {
  return (
    <div>
      <h1 className="pt">You haved {props.correctAnswers} of {5 - props.correctAnswers}</h1>
    </div>
  )
}

export default StatsDisplayer
