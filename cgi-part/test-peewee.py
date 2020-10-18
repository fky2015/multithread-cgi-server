#! /usr/bin/python3
# -*- coding: UTF-8 -*-

from peewee import *
import os
mysql_user = os.getenv('MYSQL_USER')
mysql_passwd = os.getenv('MYSQL_PWD')
db = MySQLDatabase(database="student", host="localhost", port=3306, user=mysql_user, passwd=mysql_passwd)
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

if __name__ == "__main__":
    # Class.create_table()
    # Student.create_table()
    # 创建表
    student_get = Student.filter(StudentId='1120172124')
    print(student_get.count())
    for student in student_get:
        print(f'{student.StudentName}')
    # print(dir(student_get))

db.close()