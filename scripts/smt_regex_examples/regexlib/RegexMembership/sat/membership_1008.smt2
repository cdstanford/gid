(explore-derivatives (re.++ (str.to_re "")(re.++ (re.union (re.++ (re.+ (re.union (re.range "A" "Z") (re.range "a" "z"))) (re.+ (re.range "0" "9"))) (re.++ (re.+ (re.range "0" "9")) (re.+ (re.union (re.range "A" "Z") (re.range "a" "z")))))(re.++ (re.* (re.union (re.range "0" "9")(re.union (re.range "A" "Z") (re.range "a" "z")))) (str.to_re "")))))
