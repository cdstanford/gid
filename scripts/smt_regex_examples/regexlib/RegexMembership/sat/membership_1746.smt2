(explore-derivatives (re.++ (str.to_re "")(re.++ (re.range "(" "(")(re.++ ((_ re.loop 1 2) (re.range "0" "9"))(re.++ ((_ re.loop 1 2) (re.++ (re.union (re.range "\x09" "\x0d")(re.union (re.range " " " ")(re.union (re.range "\x85" "\x85") (re.range "\xa0" "\xa0")))) ((_ re.loop 1 2) (re.range "0" "9"))))(re.++ (re.range ")" ")")(re.++ (re.union (re.range "\x09" "\x0d")(re.union (re.range " " " ")(re.union (re.range "\x85" "\x85") (re.range "\xa0" "\xa0"))))(re.++ (re.++ ((_ re.loop 1 2) (re.range "0" "9")) ((_ re.loop 1 2) (re.++ (re.union (re.range "\x09" "\x0d")(re.union (re.range " " " ")(re.union (re.range "\x85" "\x85") (re.range "\xa0" "\xa0")))) ((_ re.loop 1 2) (re.range "0" "9")))))(re.++ (re.opt (re.++ (re.range "-" "-") ((_ re.loop 1 4) (re.range "0" "9")))) (str.to_re ""))))))))))
