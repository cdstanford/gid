(explore-derivatives (re.++ (str.to_re "")(re.++ ((_ re.loop 4 4) (re.range "0" "9"))(re.++ ((_ re.loop 0 2) (re.union (re.range "\x09" "\x0d")(re.union (re.range " " " ")(re.union (re.range "\x85" "\x85") (re.range "\xa0" "\xa0")))))(re.++ ((_ re.loop 2 2) (re.range "A" "z")) (str.to_re ""))))))
