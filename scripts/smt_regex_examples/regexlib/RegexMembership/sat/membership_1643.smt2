(explore-derivatives (re.++ (str.to_re "")(re.++ (re.range "0" "0")(re.++ (re.++ (re.range "6" "6")(re.++ (re.union (re.range "0" "0")(re.union (re.range "4" "7") (re.range "9" "9"))) (re.union (re.range "0" "0")(re.union (re.range "4" "4")(re.union (re.range "6" "6") (re.range "9" "9"))))))(re.++ (re.opt (re.range "-" "-"))(re.++ (re.opt (re.range "1" "1"))(re.++ (re.range "1" "9")(re.++ ((_ re.loop 6 6) (re.range "0" "9")) (str.to_re "")))))))))
