(explore-derivatives (re.union (re.++ (str.to_re "")(re.++ (re.range "0" "9")(re.++ (re.opt (re.range "%" "%")) (str.to_re ""))))(re.union (re.++ (str.to_re "")(re.++ (re.range "1" "1")(re.++ (re.range "0" "9")(re.++ (re.opt (re.range "%" "%")) (str.to_re "")))))(re.union (re.++ (str.to_re "")(re.++ (re.range "2" "2")(re.++ (re.range "0" "9")(re.++ (re.opt (re.range "%" "%")) (str.to_re "")))))(re.union (re.++ (str.to_re "")(re.++ (re.range "3" "3")(re.++ (re.range "0" "5")(re.++ (re.opt (re.range "%" "%")) (str.to_re "")))))(re.union (re.++ (str.to_re "")(re.++ (re.range "0" "9")(re.++ (re.range "." ".")(re.++ ((_ re.loop 1 2) (re.range "0" "9"))(re.++ (re.opt (re.range "%" "%")) (str.to_re ""))))))(re.union (re.++ (str.to_re "")(re.++ (re.range "1" "1")(re.++ (re.range "0" "9")(re.++ (re.range "." ".")(re.++ ((_ re.loop 1 2) (re.range "0" "9"))(re.++ (re.opt (re.range "%" "%")) (str.to_re "")))))))(re.union (re.++ (str.to_re "")(re.++ (re.range "2" "2")(re.++ (re.range "0" "9")(re.++ (re.range "." ".")(re.++ ((_ re.loop 1 2) (re.range "0" "9"))(re.++ (re.opt (re.range "%" "%")) (str.to_re "")))))))(re.union (re.++ (str.to_re "")(re.++ (re.range "3" "3")(re.++ (re.range "0" "4")(re.++ (re.range "." ".")(re.++ ((_ re.loop 1 2) (re.range "0" "9"))(re.++ (re.opt (re.range "%" "%")) (str.to_re ""))))))) (re.++ (str.to_re "")(re.++ (str.to_re (seq.++ "3" (seq.++ "5" "")))(re.++ (re.opt (re.range "%" "%")) (str.to_re "")))))))))))))
