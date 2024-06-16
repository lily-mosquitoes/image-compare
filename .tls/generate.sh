mkdir -p live/localhost
openssl genpkey -genparam -algorithm ec -pkeyopt ec_paramgen_curve:P-256 -out live/localhost/ECPARAM.pem
openssl req -x509 -nodes -newkey ec:live/localhost/ECPARAM.pem -keyout live/localhost/privkey.pem -out live/localhost/fullchain.pem -days 10 -config openssl.cnf
