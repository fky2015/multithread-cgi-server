#! /bin/bash

if [ $# -lt 1 ]
then
    chmod 755 cgi-bin/*.py
    echo "/cgi-bin/*.py is set Executable"
    sudo /usr/bin/python3 -m pip install pymysql peewee cryptography
elif [ $1 = 'test' ]
then 
    echo "Now starting the web server"
    python3 -m http.server --cgi
else
    echo 'Wrong arguments'
fi

