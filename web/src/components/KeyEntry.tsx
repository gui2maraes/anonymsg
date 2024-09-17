import "./KeyEntry.css";
export class PrivateKey {
  name: string;
  pem: string;
  registered: boolean;
  constructor();
}
// export type PrivateKey = {
//   name: string;
//   content: string;
//   registered: boolean;

// };
export function KeyEntry({
  privateKey,
  onClick,
  selected,
}: {
  privateKey: PrivateKey;
  onClick: (event: React.MouseEvent<HTMLElement>) => void;
  selected: boolean;
}) {
  if (!selected) {
    return (
      <div className="key-entry" onClick={onClick}>
        {privateKey.name}
      </div>
    );
  }
  return (
    <div className="key-entry">
      <div className="key-name" onClick={onClick}>
        {privateKey.name}
      </div>
      <pre className="key-content">{privateKey.content}</pre>
    </div>
  );
}
