(explore-derivatives (re.union (re.++ (str.to_re "")(re.++ (re.opt (re.range "0" "0"))(re.++ (re.union (re.range "\x00" "\x09") (re.range "\x0b" "\xff"))(re.++ ((_ re.loop 1 2) (re.range "0" "0"))(re.++ (re.range "1" "9") (str.to_re ""))))))(re.union (re.++ (str.to_re "")(re.++ (re.opt (re.range "0" "0"))(re.++ (re.union (re.range "\x00" "\x09") (re.range "\x0b" "\xff"))(re.++ (re.range "1" "9")(re.++ ((_ re.loop 0 2) (re.range "0" "9")) (str.to_re ""))))))(re.union (re.++ (str.to_re "")(re.++ (re.union (re.range "1" "1") (re.++ (re.range "1" "1")(re.++ (re.union (re.range "\x00" "\x09") (re.range "\x0b" "\xff")) ((_ re.loop 1 3) (re.range "0" "0"))))) (str.to_re ""))) (re.++ (str.to_re "")(re.++ (re.opt (re.range "0" "0"))(re.++ (re.union (re.range "\x00" "\x09") (re.range "\x0b" "\xff"))(re.++ (re.range "0" "0")(re.++ (re.range "1" "9")(re.++ (re.range "0" "9") (str.to_re "")))))))))))
