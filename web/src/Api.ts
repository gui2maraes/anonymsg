type RegisterSchema = {
  // key name to be registered
  name: string;
  // public key PEM string
  key: string;
};
type PublishSchema = {
  // name of the message recipient
  recipient: string;
  // base64 encoded string
  content: string;
};
type GetMessagesSchema = {
  // name of the messages recipient
  recipient: string;
  // maximum number of messages to get
  limit?: number;
};
type MessageSchema = {
  // base64 encrypted content of the message
  content: string;
  // date and time message was sent
  sent_at: Date;
};
type NameSearchSchema = {
  name: string;
};
type NameSearchResultSchema = string[];
class RegisterInfo {
  readonly name: string;
  readonly key: string;
  constructor(name: string, publicKey: CryptoKey) {
    this.name = name;
  }
}
