(explore-derivatives (re.++ (str.to_re "")(re.++ (re.union (re.++ (re.union (re.++ (str.to_re (seq.++ "3" (seq.++ "1" (seq.++ "/" "")))) (re.union (re.++ (re.opt (re.range "0" "0")) (re.union (re.range "1" "1")(re.union (re.range "3" "3")(re.union (re.range "5" "5") (re.range "7" "8"))))) (re.++ (re.range "1" "1") (re.union (re.range "0" "0") (re.range "2" "2"))))) (re.++ (re.union (str.to_re (seq.++ "2" (seq.++ "9" ""))) (str.to_re (seq.++ "3" (seq.++ "0" ""))))(re.++ (re.range "/" "/") (re.union (re.++ (re.opt (re.range "0" "0")) (re.union (re.range "," ",")(re.union (re.range "1" "1") (re.range "3" "9")))) (re.++ (re.range "1" "1") (re.range "0" "2"))))))(re.++ (re.range "/" "/")(re.++ (re.opt (re.union (re.++ (re.range "1" "1") (re.range "6" "9")) (re.++ (re.range "2" "9") (re.range "0" "9")))) ((_ re.loop 2 2) (re.range "0" "9")))))(re.union (re.++ (str.to_re (seq.++ "2" (seq.++ "9" (seq.++ "/" ""))))(re.++ (re.opt (re.range "0" "0"))(re.++ (str.to_re (seq.++ "2" (seq.++ "/" ""))) (re.union (re.++ (re.opt (re.union (re.++ (re.range "1" "1") (re.range "6" "9")) (re.++ (re.range "2" "9") (re.range "0" "9")))) (re.union (re.++ (re.range "0" "0") (re.union (re.range "4" "4") (re.range "8" "8")))(re.union (re.++ (re.union (re.range "2" "2")(re.union (re.range "4" "4")(re.union (re.range "6" "6") (re.range "8" "8")))) (re.union (re.range "0" "0")(re.union (re.range "4" "4") (re.range "8" "8")))) (re.++ (re.union (re.range "1" "1")(re.union (re.range "3" "3")(re.union (re.range "5" "5")(re.union (re.range "7" "7") (re.range "9" "9"))))) (re.union (re.range "2" "2") (re.range "6" "6")))))) (re.++ (re.union (str.to_re (seq.++ "1" (seq.++ "6" "")))(re.union (re.++ (re.union (re.range "2" "2")(re.union (re.range "4" "4")(re.union (re.range "6" "6") (re.range "8" "8")))) (re.union (re.range "0" "0")(re.union (re.range "4" "4") (re.range "8" "8")))) (re.++ (re.union (re.range "3" "3")(re.union (re.range "5" "5")(re.union (re.range "7" "7") (re.range "9" "9")))) (re.union (re.range "2" "2") (re.range "6" "6"))))) (str.to_re (seq.++ "0" (seq.++ "0" "")))))))) (re.++ (re.union (re.++ (re.opt (re.range "0" "0")) (re.range "1" "9"))(re.union (re.++ (re.range "1" "1") (re.range "0" "9")) (re.++ (re.range "2" "2") (re.range "0" "8"))))(re.++ (re.range "/" "/")(re.++ (re.union (re.++ (re.opt (re.range "0" "0")) (re.range "1" "9")) (re.++ (re.range "1" "1") (re.range "0" "2")))(re.++ (re.range "/" "/") (re.++ (re.opt (re.union (re.++ (re.range "1" "1") (re.range "6" "9")) (re.++ (re.range "2" "9") (re.range "0" "9")))) ((_ re.loop 2 2) (re.range "0" "9")))))))))(re.++ (re.range " " " ")(re.++ (re.union (str.to_re (seq.++ "2" (seq.++ "0" "")))(re.union (str.to_re (seq.++ "2" (seq.++ "1" "")))(re.union (str.to_re (seq.++ "2" (seq.++ "2" "")))(re.union (str.to_re (seq.++ "2" (seq.++ "3" ""))) (re.++ (re.opt (re.range "0" "1")) (re.range "0" "9"))))))(re.++ (re.range ":" ":")(re.++ (re.opt (re.range "0" "5"))(re.++ (re.range "0" "9")(re.++ (re.range ":" ":")(re.++ (re.opt (re.range "0" "5"))(re.++ (re.range "0" "9") (str.to_re ""))))))))))))
