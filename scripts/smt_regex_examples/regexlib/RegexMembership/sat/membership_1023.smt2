(explore-derivatives (re.++ (str.to_re "")(re.++ (re.opt (str.to_re (seq.++ "C" (seq.++ "Z" ""))))(re.++ ((_ re.loop 8 10) (re.range "0" "9")) (str.to_re "")))))
