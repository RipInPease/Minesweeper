import './App.css';
import GameGrid from './components/game-grid';

export default function App() {

  const object = [
    [0, 1, 0, 0, 1, 1],
    [1, 0, 0, 1, 1, 0],
    [0, 1, 0, 1, 0, 1],
    [0, 1, 0, 1, 0, 1],
    [0, 1, 0, 1, 0, 1]
  ];

  return (
    <div className='app'>
      <div className="app-content">
        <GameGrid gameRows={object} />
      </div>
    </div>
  )
}