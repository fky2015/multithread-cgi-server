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
    class1 = Class.create(ClassId = '07111701', ClassName = '计科1班')
    szl = Student.create(StudentId='1120172124', StudentClass = class1, StudentName="孙璋亮")
    hsx = Student.create(StudentId='1120172150', StudentClass=class1, StudentName="谢威宇")

db.close()