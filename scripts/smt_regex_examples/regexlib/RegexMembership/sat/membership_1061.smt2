(explore-derivatives (re.++ (str.to_re "")(re.++ (re.union (re.++ (re.union (str.to_re (seq.++ "+" (seq.++ "3" (seq.++ "1" "")))) (str.to_re (seq.++ "0" (seq.++ "0" (seq.++ "3" (seq.++ "1" ""))))))(re.++ (re.union (re.range "\x09" "\x0d")(re.union (re.range " " " ")(re.union (re.range "\x85" "\x85") (re.range "\xa0" "\xa0"))))(re.++ (str.to_re (seq.++ "(" (seq.++ "0" (seq.++ ")" "")))) ((_ re.loop 9 9) (re.range "0" "9")))))(re.union (re.++ (re.union (str.to_re (seq.++ "+" (seq.++ "3" (seq.++ "1" "")))) (str.to_re (seq.++ "0" (seq.++ "0" (seq.++ "3" (seq.++ "1" ""))))))(re.++ (re.union (re.range "\x09" "\x0d")(re.union (re.range " " " ")(re.union (re.range "\x85" "\x85") (re.range "\xa0" "\xa0"))))(re.++ (re.range "0" "0") ((_ re.loop 9 9) (re.range "0" "9"))))) (re.++ (re.range "0" "0") ((_ re.loop 9 9) (re.range "0" "9"))))) (str.to_re ""))))
