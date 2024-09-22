# λ-calculus

- wikipedia
  - [lambda calculus](https://en.wikipedia.org/wiki/Lambda_calculus)
  - [backus-naur form](https://en.wikipedia.org/wiki/Backus%E2%80%93Naur_form)
  - [de bruijn index](https://en.wikipedia.org/wiki/De_Bruijn_index)
  - [evaluation strategy](https://en.wikipedia.org/wiki/Evaluation_strategy)
  - [lazy evaluation](https://en.wikipedia.org/wiki/Lazy_evaluation)
  - [church-rosser theorem](https://en.wikipedia.org/wiki/Church%E2%80%93Rosser_theorem)
  - [fixed-point combinator](https://en.wikipedia.org/wiki/Fixed-point_combinator)

## lambda abstraction (function definition)

$`(\lambda x.M)`$

### BNF: Backus-Naur Form

```text
e = v      varaible
  | λv.e   λ abstraction
  | (e e)  λ application
```

### variable

$`x`$

#### bound variable

$`(\lambda x.x)`$

#### free variable

$`(\lambda x.y)`$

### application

$`(M{ }N)`$

### α-conversion

$`(\lambda x.M[x]) \rightarrow (\lambda y.M[y])`$

- rename bound variable
- used to avoid name collisions

#### substitution

replace free variables in expression M with expression N

- $`M[x := N]`$
- $`E \{ x \mapsto y \}`$

### β-reduction

- $`((\lambda x.M[x]){ }N) \rightarrow (M[x := N))`$
- $`(\lambda x.M{ }N) \rightarrow M \{ x \mapsto N \}`$

### evaluation

$`(\lambda x.M{ }N)`$

#### normal-order evaluation

- β-reduction first
- lazy evaluation
- non-strict evaluation

example:

1. $`(\lambda x.(x + x){ }(3 + 4))`$
2. $`(3 + 4) + (3 + 4)`$
3. $`7 + (3 + 4)`$
4. $`7 + 7`$
5. $`14`$

call-by-need:

1. $`(\lambda z.y{ }(\lambda x.(x{ }x){ }\lambda x.(x{ }x)))`$
2. $`y{ }\{z \mapsto (\lambda x.(x{ }x){ }\lambda x.(x{ }x))\}`$
3. $`y`$

#### applicative-order evaluation

- N evaluation first
- eager evaluation
- strict evaluation

example:

1. $`(\lambda x.(x + x){ }(3 + 4))`$
2. $`(\lambda x.(x + x){ }(7))`$
3. $`7 + 7`$
4. $`14`$

loop:

1. $`(\lambda z.y{ }(\lambda x.(x{ }x){ }\lambda x.(x{ }x)))`$
2. $`(\lambda z.y{ }(x{ }x)\{x \mapsto \lambda x.(x{ }x)\})`$
3. $`(\lambda z.y{ }(\lambda x.(x{ }x){ }\lambda x.(x{ }x)))`$

### curring

$`\lambda x \lambda y.(x \times y)`$

1. $`((\lambda x \lambda y.(x \times y) { } 3) { } 4)`$
2. $`(\lambda y.(3 \times y) { } 4)`$
3. $`3 \times 4`$
4. $`12`$

### partial application

1. $`(\lambda x \lambda y.(x \times y) { } 2) \rightarrow \lambda y.(2 \times y) = F`$
2. $`(F{ }5) = (\lambda y.(2 \times y) { } 5) \rightarrow 2 \times 5 \rightarrow 10`$

### higher-order function

$`\lambda x \lambda f.(f{ }x)`$

1. $`((\lambda x \lambda f.(f{ }x) { } 3) { } \lambda x.x^{2})`$
2. $`(\lambda f.(f{ } 3) { } \lambda x.x^{2})`$
3. $`(\lambda x.x^{2} 3)`$
4. $`3^{2}`$
5. $`9`$

### fixed-point combinator

$`f{ }(fix{ }f) = fix{ }f`$

#### y combinator

$`Y = \lambda f.(\lambda x.(f{ }(x{ }x)){ }\lambda x.(f{ }(x{ }x)))`$

#### z combinator

$`Z = \lambda f.(\lambda x.(f{ }\lambda y.((x{ }x){ }y){ }\lambda x.(f{ }\lambda y.((x{ }x){ }y)))`$

