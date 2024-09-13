class PemKey {
  pem: string;
  private constructor(pem: string) {
    this.pem = pem;
  }
  static async fromKey(key: CryptoKey): Promise<PemKey> {
    const pem_array = new Uint8Array(
      await window.crypto.subtle.exportKey("pkcs8", key),
    );
    const decoder = new TextDecoder();
    const pem_string = decoder.decode(pem_array);
  }
}
