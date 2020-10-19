# Doc for CGI Part

## PrePare

Ensure that that there exist `/bin/bash` , `/usr/bin/python3` and `/usr/bin/pip3` in your env. 

> you can replace `./*.py` with `python3 *.py`

## Steps

1. **exec** `./deploy.sh` in the base directory.

2. please ensure there exists `mysql` (**port 3306**)

   in mysql-cli (as root) :  ( you can use `mysql -u root -p`)
           `create database student;`
           `create user 'studentquery'@'localhost' identified by 'testquery';`
   
3. **exec** `MYSQL_USER='xxx' MYSQL_PWD='xxx' ./init_mysql_database.py`(replace xxx with your MYSQL user and password, ensure the user has been granted **create select update delete**)

4. **exec** `grant select on student.* to 'studentquery'@'localhost';` (as root in mysql-cli)

5. then you can test the `/cgi-bin/*` by assess `/*.html` aftering **exec** `./deploy.sh test`

## TODO

- [ ] deploy this on docker
- [ ] enhance the security

