from time import time

first_num = 1
second_num = 1
new_num = 0
limit = 1_476

start = time()
for i in range(limit - 2):
    new_num = first_num + second_num
    second_num = first_num
    first_num = new_num
end = time()

print(first_num)
print(second_num)

print(f"{end - start} seconds — {(end - start) * 1000} ms")
