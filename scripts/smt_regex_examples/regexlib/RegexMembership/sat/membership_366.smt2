(explore-derivatives (re.++ (str.to_re "")(re.++ (re.union (re.range "0" "9")(re.union ((_ re.loop 1 9) (re.range "0" "9"))(re.union (re.++ (re.range "1" "1") ((_ re.loop 1 9) (re.range "0" "9")))(re.union (re.++ (str.to_re (seq.++ "2" (seq.++ "0" ""))) ((_ re.loop 8 8) (re.range "0" "9")))(re.union (re.++ (str.to_re (seq.++ "2" (seq.++ "1" (seq.++ "3" "")))) ((_ re.loop 7 7) (re.range "0" "9")))(re.union (re.++ (str.to_re (seq.++ "2" (seq.++ "1" (seq.++ "4" (seq.++ "6" ""))))) ((_ re.loop 6 6) (re.range "0" "9")))(re.union (re.++ (str.to_re (seq.++ "2" (seq.++ "1" (seq.++ "4" (seq.++ "7" (seq.++ "3" "")))))) ((_ re.loop 5 5) (re.range "0" "9")))(re.union (re.++ (str.to_re (seq.++ "2" (seq.++ "1" (seq.++ "4" (seq.++ "7" (seq.++ "4" (seq.++ "7" ""))))))) ((_ re.loop 4 4) (re.range "0" "9")))(re.union (re.++ (str.to_re (seq.++ "2" (seq.++ "1" (seq.++ "4" (seq.++ "7" (seq.++ "4" (seq.++ "8" (seq.++ "2" "")))))))) ((_ re.loop 3 3) (re.range "0" "9")))(re.union (re.++ (str.to_re (seq.++ "2" (seq.++ "1" (seq.++ "4" (seq.++ "7" (seq.++ "4" (seq.++ "8" (seq.++ "3" (seq.++ "5" ""))))))))) ((_ re.loop 2 2) (re.range "0" "9"))) (re.++ (str.to_re (seq.++ "2" (seq.++ "1" (seq.++ "4" (seq.++ "7" (seq.++ "4" (seq.++ "8" (seq.++ "3" (seq.++ "6" (seq.++ "4" "")))))))))) (re.range "0" "7")))))))))))) (str.to_re ""))))
