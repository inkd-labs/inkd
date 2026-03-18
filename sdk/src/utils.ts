export function slugToBytes(slug: string): Uint8Array {
  const out = new Uint8Array(32);
  const bytes = new TextEncoder().encode(slug);
  out.set(bytes.slice(0, 32));
  return out;
}

export function bytesToHex(bytes: Uint8Array): string {
  return Array.from(bytes)
    .map((b) => b.toString(16).padStart(2, '0'))
    .join('');
}

export function assertDefined<T>(value: T | undefined | null, name: string): asserts value is T {
  if (value === undefined || value === null || value === '') {
    throw new Error(`missing required field: ${name}`);
  }
}

export function now(): number {
  return Math.floor(Date.now() / 1000);
}

export function isExpired(expiresAt: number, clock: number = now()): boolean {
  return expiresAt > 0 && expiresAt <= clock;
}
