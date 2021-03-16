通話 = 69108
NULL = 26976

result = (1 - NULL / 通話) * 100
result = round(result * 100) / 100
print('100% -', NULL, '/', 通話, '=', result, '%')