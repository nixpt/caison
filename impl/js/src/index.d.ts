/**
 * TypeScript types for the CAISON parser.
 * Spec: https://github.com/nixpt/caison/blob/main/SPEC.md
 */

/** A parse or projection error, carrying position (SPEC §7). */
export declare class CaisonError extends Error {
  readonly name: "CaisonError";
  /** The message without the position suffix. */
  readonly reason: string;
  readonly line: number;
  readonly col: number;
}

/** `@name(args) { k: "v" }` — SPEC §4.3. */
export declare class Annotation {
  readonly name: string;
  readonly args: string | null;
  readonly properties: Record<string, string>;
}

/** `@synthesize("description")` — SPEC §4.4. An instruction, not data. */
export declare class Synthesize {
  readonly description: string;
}

/** A CAISON value. Objects are `Map`s so key order and metadata survive. */
export type CaisonValue =
  | string | number | boolean | null
  | Synthesize
  | Map<string, Node>
  | Node[];

/**
 * A value plus its metadata.
 *
 * `confidence` is `null` when unstated. SPEC §4.2: absent is NOT the same as
 * 1.0 — absent means "no claim". Keep them distinguishable.
 */
export declare class Node {
  readonly value: CaisonValue;
  readonly confidence: number | null;
  readonly annotations: Annotation[];
  /** True when the key was written `~"intent"` (SPEC §4.1). */
  readonly semantic: boolean;
}

export declare class Document {
  readonly root: Map<string, Node>;
  readonly version: string;
}

/** A node's §6 projection: a bare value, or a wrapper when it carries metadata. */
export type Projected =
  | string | number | boolean | null
  | Projected[]
  | { $synthesize: string }
  | { [key: string]: Projected }
  | {
      $value: Projected;
      $confidence?: number;
      $annotations?: Array<{
        name: string;
        args: string | null;
        properties: Record<string, string>;
      }>;
    };

/** Parse a CAISON document. Throws {@link CaisonError} with position on failure. */
export declare function parse(text: string): Document;

/** Parse and project to plain JSON-compatible data (SPEC §6). */
export declare function loads(text: string): Record<string, Projected>;

/** The SPEC §6 projection of an already-parsed root. */
export declare function project(root: Map<string, Node>): Record<string, Projected>;

/** Value-only projection: drops all metadata. Lossy by construction. */
export declare function projectValues(root: Map<string, Node>): Record<string, unknown>;
