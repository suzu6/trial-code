from jinja2 import Environment, FileSystemLoader, select_autoescape

env = Environment(
    loader=FileSystemLoader('.'),
    autoescape=select_autoescape(['html', 'xml'])
)

param = {
    'link': 'https://amzn.to/2VO1hze',
    'title': 'Node.js超入門[第2版] '
}


template = env.get_template('template.txt')
html = template.render(param=param)

dst_path = './dst.html'

with open(dst_path, mode='w', encoding="UTF-8") as f:
    f.write(html)
