"""caison — a CAISON parser for Python.

CAISON (Crush AI-native Semantic Object Notation) is a JSON-shaped format with four
AI-native primitives: semantic keys, confidence, annotations and synthesized
values. See the spec: https://github.com/nixpt/caison/blob/main/SPEC.md

    >>> import caison
    >>> doc = caison.parse('temperature: 21.5 ~0.8')
    >>> doc.root['temperature'].value, doc.root['temperature'].confidence
    (21.5, 0.8)
    >>> caison.loads('tags: ["a", "b"]')
    {'tags': ['a', 'b']}
"""
from ._json import project, project_values
from ._model import Annotation, CaisonError, Document, Node, Synthesize
from ._parser import parse

__all__ = ["parse", "loads", "Document", "Node", "Annotation", "Synthesize",
           "CaisonError", "project", "project_values"]
__version__ = "0.1.0"


def loads(text: str) -> dict:
    """Parse and project to plain JSON-compatible data (SPEC §6).

    Use `parse()` instead when you need confidence or annotations -- this
    drops them, matching the conformance corpus.
    """
    return parse(text).to_json()
