const CRYPTO_ALGORITHMS = new Set([
  "BLAKE3",
  "BLAKE3 (rayon)",
  "BLAKE2b512",
  "BLAKE2b256",
  "BLAKE2s256",
  "SHA-1",
  "SHA-224",
  "SHA-256",
  "SHA-384",
  "SHA-512",
  "SHA-512/224",
  "SHA-512/256",
  "SHA3-224",
  "SHA3-256",
  "SHA3-384",
  "SHA3-512",
  "SHAKE128",
  "SHAKE256",
  "Keccak-224",
  "Keccak-256",
  "Keccak-384",
  "Keccak-512",
  "MD5",
  "RIPEMD-128",
  "RIPEMD-160",
  "RIPEMD-256",
  "RIPEMD-320",
  "SM3",
  "Streebog-256",
  "Streebog-512",
  "Whirlpool",
  "Ascon-Hash256",
  "Tiger2",
]);

export function isCryptographic(algorithm: string): boolean {
  return CRYPTO_ALGORITHMS.has(algorithm);
}

export const CRYPTO_COLOR = "#4361ee";
export const NONCRYPTO_COLOR = "#2ec4b6";
export const PLATFORM_COLORS = [
  "#4361ee",
  "#e63946",
  "#f4a261",
  "#2ec4b6",
  "#9b5de5",
];
