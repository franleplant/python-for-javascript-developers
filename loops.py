
# in python you usually use these types of loops
for element in ["first element", "second element", "etc"]:
    print(element)

for index, element in enumerate(["first element", "second element", "etc"]):
    print(index,element)

elements = ["first element", "second element", "etc"]
for index in range(len(elements)):
    element = elements[index]
    print(index,element)


def double(x):
    return x + x

def is_odd(x):
    return x % 2 == 0
# functional looping methods are also available

numbers = [1, 2, 3, 4]
result = map(lambda x: x + x, numbers) 
# recast it to a list
print(list(result)) 

# or use larger funcs defined previously
result = map(double, numbers) 
print(list(result)) 

# filter
result = filter(is_odd, numbers) 
print(list(result)) 

# chaining is awkward but you could use
# https://github.com/EntilZha/PyFunctional
# or one of tecniques exposed here
# https://stackoverflow.com/questions/24831476/what-is-the-python-way-of-chaining-maps-and-filters
result = map(double, filter(is_odd, numbers))
print(list(result)) 

