(explore-derivatives (re.++ (str.to_re "")(re.++ (re.union (str.to_re (seq.++ "1" (seq.++ "0" (seq.++ "0" "")))) ((_ re.loop 0 2) (re.range "0" "9"))) (str.to_re ""))))
