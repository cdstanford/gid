(explore-derivatives (re.++ (str.to_re "")(re.++ (re.* (re.union (re.range "\x00" "/") (re.range ":" "\xff")))(re.++ ((_ re.loop 10 10) (re.++ (re.range "0" "9") (re.* (re.union (re.range "\x00" "/") (re.range ":" "\xff"))))) (str.to_re "")))))
