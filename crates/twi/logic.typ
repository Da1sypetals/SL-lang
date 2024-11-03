Let:
- first search from current stack down to the bottom of stack;
- if not found, find global;
- if further not, runtime error.

= Note
- Intermediate result is allocated on heap, but is not tracked on stack, thus 