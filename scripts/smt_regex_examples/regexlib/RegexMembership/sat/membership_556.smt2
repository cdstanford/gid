(explore-derivatives (re.union (re.++ (str.to_re "")(re.++ (re.opt (re.range "0" "0"))(re.++ (re.opt (re.range "0" "9"))(re.++ (re.range "0" "9") (str.to_re ""))))) (re.++ (str.to_re "")(re.++ (str.to_re (seq.++ "1" (seq.++ "0" (seq.++ "0" "")))) (str.to_re "")))))
