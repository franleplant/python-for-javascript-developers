// TODO start moving to a markdown file with straight
// comparision and use some lib to extract code blocks and run
// them as a way of validating them
function helloWorld() {
    console.log("hello world")
}

const helloWorldLambda = () => console.log("hello world")

helloWorld()
helloWorldLambda()


const obj = {
    key: "value",
    refFn: helloWorld,
    refLambda: x => console.log(x)
}

const array = [1,2,3]

// you can simulate tuples by using arrays in js
const myTuple = [1,2,3]

const mySet = new Set([1,2,3])

// string interpolation
const name = "fran"
console.log(`hello ${name}`)

