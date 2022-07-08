import logo from '!file-loader!./kubernetes.svg'; // eslint-disable-line
import './App.css';
import { TodoWidget } from './features/todo';
import { useGetPictureOfTheDayQuery } from './api/pictureOfTheDay'

const PictureOfTheDay = () => {
  const { data, isLoading, isError } = useGetPictureOfTheDayQuery()

  if (isLoading) {
    return <div>Loading...</div>
  }

  if (isError) {
    return <div>Error!</div>
  }

  const { url, last_updated } = data

  return (
    <div>
      <img className="picture-of-the-day" src={url} />
      <p>Last updated at: {last_updated}</p>
    </div>
  )
};

function App() {
  return (
    <div className="App">
      <header className="App-header">
        <img src={logo} className="App-logo" alt="logo" />
        <p>
          Hello, Kubernetes!
        </p>
        <h1>Picture of the day:</h1>
        <PictureOfTheDay />
        <TodoWidget />
      </header>
    </div>
  );
}

export default App;
