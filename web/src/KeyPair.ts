export class KeyPair {
  readonly name: string;
  readonly privateKey: CryptoKey;
  readonly publicKey: CryptoKey;

  constructor(name: string, keyPair: CryptoKeyPair) {
    this.name = name;
    this.privateKey = keyPair.privateKey;
    this.publicKey = keyPair.publicKey;
  }

  async register(): Promise<void> {
    const keyPem = await window.crypto.subtle.exportKey(
      "pkcs8",
      this.publicKey,
    );
    const body = { name: this.name, key: keyPem };
    const req = new Request("/api/register", {
      method: "POST",
      body: JSON.stringify(body),
    });
  }
}

export async function generateKeyPair(name: string): Promise<KeyPair> {
  const crypto = window.crypto;
  const keyPair = await crypto.subtle.generateKey(
    {
      name: "RSA-OAEP",
      modulusLength: 4096,
      publicExponent: new Uint8Array([0x01, 0x00, 0x01]),
      hash: "SHA-256",
    },
    true,
    ["encrypt", "decrypt"],
  );
  return new KeyPair(name, keyPair);
}
