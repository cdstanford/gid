(explore-derivatives (re.++ (str.to_re "")(re.++ (re.union (re.range "\x00" "-")(re.union (re.range "/" "^") (re.range "`" "\xff")))(re.++ ((_ re.loop 2 2) (re.++ (re.* (re.union (re.range "0" "9")(re.union (re.range "A" "Z")(re.union (re.range "_" "_") (re.range "a" "z")))))(re.++ (re.opt (re.range "." "."))(re.++ (re.+ (re.union (re.range "0" "9")(re.union (re.range "A" "Z")(re.union (re.range "_" "_") (re.range "a" "z"))))) (re.union (re.range "\x00" "^") (re.range "`" "\xff"))))))(re.++ (re.range "@" "@")(re.++ (re.+ (re.union (re.range "0" "9") (re.range "a" "z")))(re.++ (re.range "." ".")(re.++ (re.union ((_ re.loop 2 3) (re.range "a" "z")) (re.++ ((_ re.loop 2 3) (re.range "a" "z"))(re.++ (re.range "." ".") ((_ re.loop 2 3) (re.range "a" "z"))))) (str.to_re "")))))))))
