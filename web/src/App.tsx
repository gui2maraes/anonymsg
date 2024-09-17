import "./App.css";
import KeyPanel from "./components/KeyPanel";
import MessageBoard from "./components/MessageBoard";
import MessageSender from "./components/MessageSender";
import Header from "./components/Header";

function App() {
  return (
    <>
      <div className="app">
        <Header />
        <KeyPanel />
        <MessageBoard />
        <MessageSender />
      </div>
    </>
  );
}

export default App;
