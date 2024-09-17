import { PublicJwk, PublicJwkAlgEnum, PublicJwkKtyEnum, PublicJwkEEnum, PublicJwkUseEnum } from "./oapi";
import { Base64 } from "js-base64";

export class KeyPair {
  readonly alias: string;
  readonly publicKey: CryptoKey;
  private readonly privateKey: CryptoKey;

  constructor(alias: string, keyPair: CryptoKeyPair) {
    this.alias = alias;
    this.privateKey = keyPair.privateKey;
    this.publicKey = keyPair.publicKey;
  }

  async publicJwk(): Promise<PublicJwk> {
    const exported = await window.crypto.subtle.exportKey("jwk", this.publicKey);
    assertJwkProp(exported.alg === PublicJwkAlgEnum.RsaOaep256);
    assertJwkProp(exported.kty === PublicJwkKtyEnum.Rsa);
    assertJwkProp(exported.e === PublicJwkEEnum.Aqab || exported.e === PublicJwkEEnum.Aqab2);
    assertJwkProp(exported.n !== undefined);
    if (!exported.use) {
      exported.use = PublicJwkUseEnum.Enc;
    }
    assertJwkProp(exported.use === PublicJwkUseEnum.Enc);

    return {
      n: exported.n,
      e: exported.e,
      alg: exported.alg,
      kty: exported.kty,
      use: exported.use,
    }

  }

  async encrypt(content: string): Promise<ArrayBuffer> {
    const enc = new TextEncoder();
    const buffer = enc.encode(content);
    const encrypted = await window.crypto.subtle.encrypt({name: "RSA-OAEP"}, this.publicKey, buffer);
    return encrypted;
  }
  async decrypt(content: ArrayBuffer): Promise<string> {
    const decrypted = await window.crypto.subtle.decrypt({name: "RSA-OAEP"}, this.privateKey, content);
    const dec = new TextDecoder();
    return dec.decode(decrypted);
  }
}

function assertJwkProp(condition: boolean, msg?: string): asserts condition {
  if (!condition) {
    throw new Error(msg);
  }
}

export async function generateKeyPair(alias: string): Promise<KeyPair> {
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
  return new KeyPair(alias, keyPair);
}

export class EncryptedContent {
  readonly base64: string;
  constructor(base64: string) {
    this.base64 = base64;
  }

  static async fromKeyPair(keyPair: KeyPair, content: string): Promise<EncryptedContent> {
    const encrypted = new Uint8Array(await keyPair.encrypt(content));
    const b64 = Base64.fromUint8Array(encrypted, true);
    return new EncryptedContent(b64);
  }
  async toDecrypted(keyPair: KeyPair): Promise<string> {
    const bytes = Base64.toUint8Array(this.base64);
    return await keyPair.decrypt(bytes);
  }
}