(explore-derivatives (re.++ (str.to_re "")(re.++ (re.range "0" "9")(re.++ (re.range "." ".")(re.++ (re.+ (re.range "0" "9"))(re.++ (re.union (re.range "\x09" "\x0d")(re.union (re.range " " " ")(re.union (re.range "\x85" "\x85") (re.range "\xa0" "\xa0"))))(re.++ (re.range "x" "x")(re.++ (re.union (re.range "\x09" "\x0d")(re.union (re.range " " " ")(re.union (re.range "\x85" "\x85") (re.range "\xa0" "\xa0"))))(re.++ (str.to_re (seq.++ "1" (seq.++ "0" "")))(re.++ (re.union (re.range "E" "E")(re.union (re.range "^" "^") (re.range "e" "e")))(re.++ (re.opt (re.range "-" "-"))(re.++ (re.+ (re.range "0" "9")) (str.to_re "")))))))))))))
