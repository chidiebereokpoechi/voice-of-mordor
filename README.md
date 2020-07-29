## Voice of Mordor

Designer loosely-typed interpreter for **vom** programming language

### How to run the interpreter

```bash
 cargo build
 cargo run
```

### What it'll look like
  
```
struct Person {
  init:[name, age] {
    self.name = name;
    self.age = age;
  }

  def speak:[] {
    print:[`My name is ` + self.name + ` and i'm ` + self.age];
  }
}


def main:[args,] {
  # This is a comment up top
  let a = [1, 2, 3, 4]; # Comment by the side
  let b = a[1];

  let c = test a {
    a -> {
      break empty;
    },
    b -> {
      break false;
    }
  };

  while a < 10 {
    a = a + 1;
  }

  let d = if true {
    return `The world is indeed round`;
  } else {
    return `The world is flat`;
  };

  print:[a,];
  print:[`Hello, world`];
}
```

### _Gimmicks_

- Arguments are "attached" to functions using the `:` operator
- Like in rust, loops and conditionals will be able to return values using the `break` keyword

That's about it for _unique_ features.
<br />
This is going to be quite a challenge :)

