(explore-derivatives (re.++ (str.to_re "")(re.++ (re.opt (re.++ (re.union (re.range "8" "8") (str.to_re (seq.++ "+" (seq.++ "3" (seq.++ "8" ""))))) (re.opt (re.range "-" "-"))))(re.++ (re.opt (re.++ (re.opt (re.range "(" "("))(re.++ (str.to_re (seq.++ "0" (seq.++ "4" (seq.++ "4" "")))) (re.opt (re.range ")" ")")))))(re.++ (re.opt (re.range "-" "-"))(re.++ ((_ re.loop 3 3) (re.range "0" "9"))(re.++ (re.opt (re.range "-" "-"))(re.++ ((_ re.loop 2 2) (re.range "0" "9"))(re.++ (re.opt (re.range "-" "-"))(re.++ ((_ re.loop 2 2) (re.range "0" "9")) (str.to_re "")))))))))))
