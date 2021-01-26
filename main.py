

def hello_world():
    print("hello world")

hello_world_lambda = lambda: print("hello_world")

hello_world()
hello_world_lambda()


# In python (and in most langs) the closest thing to a js object is a dict
obj = {
 "key": "value",
    # this is a function pointer, but you need to define it elsewhere
 "ref_fn": hello_world,
    # lambdas in python are like arrow functions 
    # without { }, i.e. they must be mostly one liners
    "ref_lambda": lambda x: print(x)
}

array = [1,2,3]

# python support native tuples!
my_tuple = (1,2,3)

set_example = set([1,2,3])

# string interpolation
name = "fran"
print(f'hello {name}')



# called keyword arguments
def named_args(name, age):
    print("named args", name, age)

named_args(age="23", name="fran")

def var_args(*args):
    print("var args", *args)

var_args("hi", 1, True)


def var_named_args(**kwargs):
    print("kwargs", kwargs)

var_named_args(name="fran", age=123)


def rest_array_args(name, *args):
    print("rest", name, args)

rest_array_args("fran", "23", 1, 2, True)

def rest_kargs(name, **kwargs):
    print("rest", name, kwargs)

rest_kargs("fran", age=23, status="midly content")

def mix(name, *args, **kwargs):
    print("mix", name,args, kwargs)

mix("fran", "leplant", age=23, status="midly content")
