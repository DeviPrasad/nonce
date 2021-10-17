curl -L -X GET "http://oauth.indus.in:40401/authorize?client_id=123456&response_type=code&client_id=98765"
curl -L -X POST "http://oauth.indus.in:40401/authorize?client_id=123456&response_type=code&client_id=98765"

curl -L -X GET "http://loiter.xyz.in:45001/lobby" -H "Origin: http://loiter.xyz.in:45001"
