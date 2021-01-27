# Python for Javascript developers

## Variables

Javascript has a more complex model for declaring
variables, depending on what relation you want them
to have to blocks and lexical scopes:

```javascript
// lexically scoped week variable
var myVar = 123
// and you can redeclare them
var myVar = "hello"


// block scoped stronger variable
// that cannot be redeclared
{
   let myLet = 123
}

// block scoped strong variable
// that cannot be redeclared
// and that must be a constant reference
{
   const myConst = [1,2,3]
   // this does not mean immutability
   myConst.push(4)
}
```

Python has mainly one way of declaring variables i.e. by assignment.
It ressembles Javascript's `var`.

```python
myVar = 123
```

## Objects

```javascript
const obj = {
  key: "value",
  fn: (x) => console.log(x),
};
```

In python (and in most langs) the closest thing to a js object is a dict

```python
obj = {
   "key": "value",
    "lambda": lambda x: print(x)
}
```

## Arrays

```javascript
const array = [1, 2, 3];
```

```python
array = [1,2,3]
```

## Tuples

Javascript doesn't support native tupples 
but you can use arrays to replace them

```javascript
const my_tuple = [1,2,3]
```

```python
my_tuple = (1,2,3)
```

## Sets

```javascript
const my_set = new Set([1,2,3])
```

```python
my_set = set([1,2,3])
```

## String Interpolation

```javascript
const name = "fran"
console.log(`hello ${name}`)
```

```python
name = "fran"
print(f'hello {name}')
```

There are other ways of doing string interpolation in Python
and in both Javascript and Python you can concatenate as primitive
form of string interpolation.

## Modules

Export default

```javascript
import localModule from '../fake/localModule.js'
console.log(localModule.sayHi())

```

```python
import fake.localModule
print(localModule.sayHi())
```

Named exports

```javascript
import {sayHi} from '../fake/localModule.js'
console.log(sayHi())
```

```python
from fake.localModule import sayHi
print(sayHi())
```

```python
for el in [1,2,3,4]:
  print(el)

```

```javascript
for (const el of [1, 2, 3, 4]) {
  console.log(el);
}
```

```
untyped

```
