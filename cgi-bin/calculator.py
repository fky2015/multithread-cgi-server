#! /usr/bin/python3
# -*- coding: UTF-8 -*-

import cgi,cgitb

form=cgi.FieldStorage()
value1=int(form.getvalue('value1'))
value2=int(form.getvalue('value2'))

print('Content-type:text/html')
print('')

print('<html>')
print('<head>')
print('<meta charset=\"utf-8\">')
print('<title>两数之和与之积</title>')
print('</head>')

print('<body>')
print(f'<h2>两数之和: {value1 + value2}</h2>')
print(f'<h2>两数之积: {value1 * value2}</h2>')
print('</body>')
print('</html>')
