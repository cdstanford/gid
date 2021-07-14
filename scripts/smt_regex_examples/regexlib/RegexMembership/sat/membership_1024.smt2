(explore-derivatives (re.++ (str.to_re "")(re.++ (re.opt (str.to_re (seq.++ "C" (seq.++ "Y" ""))))(re.++ ((_ re.loop 8 8) (re.range "0" "9"))(re.++ (re.range "A" "Z") (str.to_re ""))))))
