(explore-derivatives (re.++ (str.to_re "")(re.++ (re.range "$" "$")(re.++ (re.range "a" "z")(re.++ ((_ re.loop 0 6) (re.union (re.range "0" "9") (re.range "a" "z"))) (str.to_re ""))))))
