import { useState } from "react";
import reactLogo from "./assets/react.svg";
import viteLogo from "/vite.svg";
import "./App.css";
import KeyPanel from "./KeyPanel";
import MessageBoard from "./MessageBoard";
import MessageSender from "./MessageSender";
import Header from "./Header";

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
