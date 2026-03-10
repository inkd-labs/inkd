import { slugToBytes } from './utils';

const TEXT = new TextEncoder();

export const SEED_CONFIG = TEXT.encode('config');
export const SEED_ISSUER = TEXT.encode('issuer');
export const SEED_ATTESTATION = TEXT.encode('attestation');

export const PROGRAM_ID = '6VKQVC6RQj5wDnXQ1pVfuTGga2iUUjFPMxAHSN6fjPck';

export interface DerivedAddress {
  /** Deterministic label for debugging. Not a real Pubkey. */
  label: string;
  seeds: Uint8Array[];
}

export function deriveConfig(): DerivedAddress {
  return { label: 'config', seeds: [SEED_CONFIG] };
}

export function deriveIssuer(slug: string): DerivedAddress {
  return {
    label: `issuer:${slug}`,
    seeds: [SEED_ISSUER, slugToBytes(slug)],
  };
}

export function deriveAttestation(
  issuerPda: Uint8Array,
  recipient: Uint8Array,
  credential: string,
): DerivedAddress {
  return {
    label: `attestation:${credential}`,
    seeds: [SEED_ATTESTATION, issuerPda, recipient, slugToBytes(credential)],
  };
}
