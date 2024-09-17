import "./KeyPanel.css";
import { KeyEntry, PrivateKey } from "./KeyEntry";
import { useState } from "react";

export default function KeyPanel() {
  const [keys, setKeys] = useState<PrivateKey[]>([]);
  const [selected, setSelected] = useState(-1);
  const toggleSelected = (i: number) => {
    console.log("clicked");
    if (selected == i) {
      setSelected(-1);
    } else {
      setSelected(i);
    }
  };
  const appendKey = () =>
    setKeys(keys.concat([{ name: "lalala", content: "lelele" }]));

  const keyList = keys.map((key: PrivateKey, i: number) => (
    <li>
      <KeyEntry
        privateKey={key}
        selected={i == selected}
        onClick={() => toggleSelected(i)}
      />
    </li>
  ));
  return (
    <div className="key-panel">
      <div className="title">
        <h2>Your Keys</h2>
      </div>
      <div>
        <button onClick={appendKey}>Generate Key Pair</button>
        <button>Import Key</button>
      </div>
      <div>
        <h3>Keys</h3>
        <ul className="key-list">{keyList}</ul>
      </div>
    </div>
  );
}
