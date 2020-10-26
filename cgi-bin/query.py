#! /usr/bin/python3
# -*- coding: UTF-8 -*-

import cgi, cgitb
from peewee import *
import datetime

db = MySQLDatabase(database="student", host="localhost", port=3306, user="studentquery", passwd="testquery")
db.connect()

class BaseModel(Model):
    class Meta:
        database = db

class Class(BaseModel):
    ClassId = CharField(unique=True, primary_key=True)
    ClassName = CharField()
    

class Student(BaseModel):
    StudentId = CharField(unique=True, primary_key=True)
    StudentName = CharField()
    StudentClass = ForeignKeyField(Class, related_name='belong_to')
    
form=cgi.FieldStorage()
studentId=form.getvalue('student-id')

student_get = Student.filter(StudentId=studentId)

print('Content-type:text/html')
print('')

print('<html>')
print('<head>')
print('<meta charset=\"utf-8\">')
print('<title>学生信息查询结果</title>')
print('</head>')
print('<body>')

if student_get.count() > 0:
    print('<h1>学生信息</h1>')
    for student in student_get:
        print(f'<h2>学生学号：{student.StudentId}</h2>')
        print(f'<h2>学生姓名：{student.StudentName} </h2>')
        print(f'<h2>学生班号：{student.StudentClass.ClassId} </h2>' )
        print(f'<h2>学生班级：{student.StudentClass.ClassName} </h2>')
else:
    print('<h1>查无此人</h1>')
    
print('</body>')
print('</html>')

db.close()