(explore-derivatives (re.++ (str.to_re "")(re.++ (str.to_re (seq.++ "~" (seq.++ "/" "")))(re.++ (re.union (re.range "0" "9")(re.union (re.range "A" "Z")(re.union (re.range "_" "_") (re.range "a" "z"))))(re.++ (re.* (re.union (re.range "-" "-")(re.union (re.range "/" "9")(re.union (re.range "A" "Z")(re.union (re.range "_" "_") (re.range "a" "z"))))))(re.++ (re.range "." ".")(re.++ (re.+ (re.union (re.range "-" "-")(re.union (re.range "0" "9")(re.union (re.range "A" "Z")(re.union (re.range "_" "_") (re.range "a" "z")))))) (str.to_re ""))))))))
