Please ensure that there exist `/bin/bash` , `/usr/bin/python3` and `/usr/bin/pip3`
you can replace './*.py' with 'python3 *.py'

1. exec './deploy.sh'

2. please ensure there exists mysql (port 3306)
    in mysql-cli (as root) :  ( you can use `mysql -u root -p`)
        `create database student;`
        `create user 'studentquery'@'localhost' identified by 'testquery';`

3. then exec init-mysql-database.py by 'MYSQL_USER=`xxx` MYSQL_PWD=`xxx` ./init_mysql_database.py' (replace xxx with your MYSQL user and password, ensure the user has been granted create select update delete)

4. then exec `grant select on student.* to 'studentquery'@'localhost';` (as root in mysql-cli)

5. then you can test the `/cgi-bin/*` by assess `/*.html` aftering exec './deploy.sh test'