/**
 * caison — a CAISON parser for JavaScript and TypeScript.
 *
 * CAISON (Crush AI-native Semantic Object Notation) is a JSON-shaped format with four
 * AI-native primitives: semantic keys, confidence, annotations and synthesized
 * values. See the spec: https://github.com/nixpt/caison/blob/main/SPEC.md
 *
 *     import { parse, loads } from "caison";
 *
 *     const doc = parse('temperature: 21.5 ~0.8');
 *     doc.root.get("temperature").value;        // 21.5
 *     doc.root.get("temperature").confidence;   // 0.8
 *
 *     loads('tags: ["a", "b"]');                // { tags: ["a", "b"] }
 *
 * Zero dependencies and no platform APIs: runs unchanged in Node and a browser.
 */
export { Annotation, CaisonError, Document, Node, Synthesize } from "./model.mjs";
export { parse } from "./parser.mjs";
export { project, projectValues } from "./project.mjs";

import { parse } from "./parser.mjs";
import { project } from "./project.mjs";

/** Parse and project to plain JSON-compatible data (SPEC §6). */
export function loads(text) {
  return project(parse(text).root);
}
