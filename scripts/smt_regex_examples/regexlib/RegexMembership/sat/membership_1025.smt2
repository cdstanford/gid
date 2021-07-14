(explore-derivatives (re.++ (str.to_re "")(re.++ (re.opt (str.to_re (seq.++ "B" (seq.++ "E" ""))))(re.++ (re.opt (re.range "0" "0"))(re.++ ((_ re.loop 9 9) (re.range "0" "9")) (str.to_re ""))))))
