# ccc-lang

A simple language to learn compiler frontends/interpreters at [37C3](https://events.ccc.de/congress/2023/hub/en/event/building-tiny-programming-languages-mohit-karekar/).

Refer the related blogpost here: https://mohitkarekar.com/posts/2023/compiler-frontend/
A simpler version to get started (contains only addition expressions): https://github.com/mohitk05/ccc-lang/tree/simple

Following are some example programs in `ccc-lang`:

```
let a = 1;
print(a);
```

```
let a = 1;
let b = a + 2;
print(b);
```

```
function add(a, b) {
  a + b
}
let a = 1;
let c = 2
let sum = add(a, c);
print(sum);
```

```
function add(a, b) { a + b };

let a = 1;
let c = 3;
let b = add(a, c);
print(b);

function sub(a, b) { a - b };
let d = sub(c, a);
print(d);
```
