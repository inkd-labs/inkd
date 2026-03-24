import { InkdClient } from './client';
import { AttestationRecord, AttestationStatus } from './types';
import { bytesToHex, isExpired, slugToBytes } from './utils';

const sample: AttestationRecord = {
  issuer: '6VKQVC6RQj5wDnXQ1pVfuTGga2iUUjFPMxAHSN6fjPck',
  recipient: '3C7Mqhnb9N4cerkMkVMZKa5ceQm1i5rSqQNaC9goNxEN',
  leaf: new Uint8Array(32),
  credential: 'solana-degen',
  issuedAt: 1700000000,
  expiresAt: 0,
  revokedAt: 0,
  index: 0n,
  status: AttestationStatus.Active,
};

describe('slug helpers', () => {
  it('packs a short slug into 32 bytes', () => {
    const b = slugToBytes('solana-degen');
    expect(b.length).toBe(32);
    expect(b[0]).toBe('s'.charCodeAt(0));
  });

  it('truncates oversized input', () => {
    const b = slugToBytes('x'.repeat(40));
    expect(b.length).toBe(32);
  });

  it('hex encodes bytes', () => {
    const b = new Uint8Array([0xde, 0xad, 0xbe, 0xef]);
    expect(bytesToHex(b)).toBe('deadbeef');
  });
});

describe('expiry logic', () => {
  it('never expires when expiresAt is zero', () => {
    expect(isExpired(0, 9_999_999_999)).toBe(false);
  });

  it('expires when clock reaches boundary', () => {
    expect(isExpired(1000, 1000)).toBe(true);
    expect(isExpired(1000, 999)).toBe(false);
  });
});

describe('InkdClient', () => {
  const client = new InkdClient({ cluster: 'mainnet-beta', rpcUrl: 'https://example.invalid' });

  it('reports the program id', () => {
    expect(client.programId).toHaveLength(44);
  });

  it('formats active attestations as valid', () => {
    const res = client.verifyLocally(sample, sample.issuedAt + 10);
    expect(res.valid).toBe(true);
  });

  it('formats revoked attestations as invalid', () => {
    const revoked = { ...sample, status: AttestationStatus.Revoked };
    const res = client.verifyLocally(revoked, sample.issuedAt + 10);
    expect(res.valid).toBe(false);
    expect(res.reason).toBe('revoked');
  });

  it('returns non-empty describe string', () => {
    expect(client.describe().length).toBeGreaterThan(10);
  });

  it('summarizes an issuer record', () => {
    const summary = client.summarizeIssuer({
      authority: sample.issuer,
      slug: 'inkd',
      issuedCount: 100n,
      revokedCount: 5n,
      createdAt: sample.issuedAt,
      active: true,
    });
    expect(summary).toContain('inkd');
    expect(summary).toContain('100');
  });
});
