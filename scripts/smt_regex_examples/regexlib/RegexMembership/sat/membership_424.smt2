(explore-derivatives (re.union (re.++ (str.to_re "")(re.++ (str.to_re (seq.++ "1" (seq.++ "0" (seq.++ "0" ""))))(re.++ (re.opt (re.++ (re.range "." ".") ((_ re.loop 0 2) (re.range "0" "0"))))(re.++ (re.* (re.range " " " "))(re.++ (re.opt (re.range "%" "%")) (str.to_re "")))))) (re.++ (str.to_re "")(re.++ ((_ re.loop 1 2) (re.range "0" "9"))(re.++ (re.opt (re.++ (re.range "." ".") ((_ re.loop 1 2) (re.range "0" "9"))))(re.++ (re.* (re.range " " " "))(re.++ (re.opt (re.range "%" "%")) (str.to_re ""))))))))
