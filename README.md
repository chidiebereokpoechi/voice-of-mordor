## Voice of Mordor

<img src="https://user-images.githubusercontent.com/50222963/92097821-69122a00-edd0-11ea-8ca4-98dff122517d.png" alt="Logo" width="200" />

Interpreter for loosely-typed designer programming language **vom**

### How to run the interpreter

```bash
cargo build
cargo run
```

### Run example files 
Currently, this just prints  out the tokens from lexical analysis

```bash
cargo run examples/hello_world[.vom] examples/getting_started[.vom]
```


### What it'll look like
  
```
struct Person {
    init:[name, age] {
        this.name = name;
        this.age = age;
    }

    def speak:[] {
        print:[`My name is ` + this.name + ` and i'm ` + this.age];
    }
}


def main:[args,] {
    # This is a comment up top
    let a = [1, 2, 3, 4]; # Comment by the side
    let b = a[1];

    let c = test a {
        a -> {
            exit empty;
        },
        b -> {
            exit false;
        }
    };

    while a < 10 {
        a = a + 1;

        if a == 5 {
            exit;
        }

        # next;
    }

    let d = if true {
        exit `The world is indeed round`;
    } else {
        exit `The world is flat`;
    };

    print:[a,];
    print:[`Hello, world`];
}
```

### _Gimmicks_

- Arguments are "attached" to functions using the `:` operator
- Like in rust, loops and conditionals will be able to return values using the `exit` keyword

That's about it for _unique_ features.
<br />
This is going to be quite a challenge :)

