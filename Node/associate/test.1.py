from jinja2 import Template

template = Template('Hello {{ name }}!')
dst = template.render(name='suzu6')

print(dst)


template = Template('Hello {{ name }}! It is {{ weather }} today.')
dst = template.render(name='suzu6', weather='fine')

print(dst)


template = Template('<title>{% block title %}aaa{% endblock %}</title>')
dst = template.render(title="hello", block="hello")

print(dst)

param = {
    'name': 'suzu6',
    'age': 28
}

template = Template('{{ param.name }} is {{ param.age }} years old.')
dst = template.render(param=param)

print(dst)

template = Template('{{ param["name"] }} is {{ param["age"] }} years old.')
dst = template.render(param=param)

print(dst)
