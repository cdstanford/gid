(explore-derivatives (re.++ (str.to_re "")(re.++ ((_ re.loop 3 3) (re.++ (re.union (re.range "0" "0")(re.union (re.++ (re.range "1" "1") ((_ re.loop 0 2) (re.range "0" "9")))(re.union (re.++ (re.range "2" "2") (re.opt (re.range "0" "9")))(re.union (re.++ (re.range "2" "2")(re.++ (re.range "0" "4") (re.range "0" "9")))(re.union (re.++ (str.to_re (seq.++ "2" (seq.++ "5" ""))) (re.range "0" "5")) (re.++ (re.range "3" "9") (re.opt (re.range "0" "9")))))))) (re.range "." ".")))(re.++ (re.union (re.range "0" "0")(re.union (re.++ (re.range "1" "1") ((_ re.loop 0 2) (re.range "0" "9")))(re.union (re.++ (re.range "2" "2") (re.opt (re.range "0" "9")))(re.union (re.++ (re.range "2" "2")(re.++ (re.range "0" "4") (re.range "0" "9")))(re.union (re.++ (str.to_re (seq.++ "2" (seq.++ "5" ""))) (re.range "0" "5")) (re.++ (re.range "3" "9") (re.opt (re.range "0" "9")))))))) (str.to_re "")))))
