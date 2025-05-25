import './game-grid.css';

export default function GameGrid({ gameRows }) {

    const handleBlockPress = (e) => {
        console.log(`You pressed on a ${e === 1 ? "mine" : "normal tile"}!`);
    }

    return (
        <div className="game-grid">

            {gameRows.map((row, i) => {
                return <div className="row" key={i}>
                    {row.map((number, j) => {
                        return <button onClick={() => handleBlockPress(number)} className="row-block" key={j}>
                            {number}
                        </button>
                    })}
                </div>
            })}

        </div>
    );
}