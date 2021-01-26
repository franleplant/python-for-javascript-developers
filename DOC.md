# example of a markdown doc

## some titles

## Objects

```javascript
const obj = {
    key: "value",
    refFn: helloWorld,
    refLambda: x => console.log(x)
}
```

In python (and in most langs) the closest thing to a js object is a dict

```python
obj = {
"key": "value",
    # this is a function pointer, but you need to define it elsewhere
 "ref_fn": hello_world,
    # lambdas in python are like arrow functions 
    # without { }, i.e. they must be mostly one liners
    "ref_lambda": lambda x: print(x)
}
```

## Arrays

```javascript
const array = [1,2,3]
```

```python
array = [1,2,3]
```

# python support native tuples!
my_tuple = (1,2,3)

set_example = set([1,2,3])

# string interpolation
name = "fran"
print(f'hello {name}')



wow so much markdown

```python
print("this is python")
```

- examples
- test `123`

```javascript
const lang = "javascript"
console.log(`this is ${lang}`)

```

```python
for el in [1,2,3,4]:
  print(el)

```

```javascript
for (const el of [1,2,3,4]) {
  console.log(el)
}

```

```
untyped

```
