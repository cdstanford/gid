(explore-derivatives (re.++ (str.to_re "")(re.++ (re.union (re.range "0" "9") (re.range "a" "z"))(re.++ (re.* (re.union (re.range "-" ".")(re.union (re.range "0" "9")(re.union (re.range "_" "_") (re.range "a" "z")))))(re.++ (re.union (re.range "0" "9") (re.range "a" "z"))(re.++ (re.range "@" "@")(re.++ (re.union (re.range "0" "9") (re.range "a" "z"))(re.++ (re.* (re.union (re.range "-" ".")(re.union (re.range "0" "9")(re.union (re.range "_" "_") (re.range "a" "z")))))(re.++ (re.union (re.range "0" "9") (re.range "a" "z"))(re.++ (re.range "." ".")(re.++ ((_ re.loop 2 4) (re.union (re.range "0" "9") (re.range "a" "z"))) (str.to_re ""))))))))))))
