openssl genpkey -genparam -algorithm ec -pkeyopt ec_paramgen_curve:P-256 -out ECPARAM.pem
openssl req -x509 -nodes -newkey ec:ECPARAM.pem -keyout privkey.pem -out fullchain.pem -days 10 -config openssl.cnf
