import logo from '!file-loader!./kubernetes.svg'; // eslint-disable-line
import './App.css';

function App() {
  return (
    <div className="App">
      <header className="App-header">
        <img src={logo} className="App-logo" alt="logo" />
        <p>
          Hello, Kubernetes!
        </p>
      </header>
    </div>
  );
}

export default App;
