(explore-derivatives (re.++ (str.to_re "")(re.++ (re.union (re.++ (re.union (re.++ (re.opt (re.range "0" "0")) (re.range "1" "9")) (re.++ (re.range "1" "1") (re.range "0" "2")))(re.++ (re.union (re.range "." ".") (re.range ":" ":"))(re.++ (re.range "0" "5")(re.++ (re.range "0" "9")(re.++ (re.opt (re.++ (re.union (re.range "." ".") (re.range ":" ":"))(re.++ (re.range "0" "5") (re.range "0" "9"))))(re.++ (re.opt (re.range " " " ")) (re.union (str.to_re (seq.++ "A" (seq.++ "M" "")))(re.union (str.to_re (seq.++ "a" (seq.++ "m" "")))(re.union (str.to_re (seq.++ "a" (seq.++ "M" "")))(re.union (str.to_re (seq.++ "A" (seq.++ "m" "")))(re.union (str.to_re (seq.++ "P" (seq.++ "M" "")))(re.union (str.to_re (seq.++ "p" (seq.++ "m" "")))(re.union (str.to_re (seq.++ "p" (seq.++ "M" ""))) (str.to_re (seq.++ "P" (seq.++ "m" "")))))))))))))))) (re.++ (re.union (re.++ (re.opt (re.range "0" "0")) (re.range "0" "9"))(re.union (re.++ (re.range "1" "1") (re.range "0" "9")) (re.++ (re.range "2" "2") (re.range "0" "3"))))(re.++ (re.union (re.range "." ".") (re.range ":" ":"))(re.++ (re.range "0" "5")(re.++ (re.range "0" "9") (re.opt (re.++ (re.union (re.range "." ".") (re.range ":" ":"))(re.++ (re.range "0" "5") (re.range "0" "9"))))))))) (str.to_re ""))))
