(explore-derivatives (re.++ (str.to_re (seq.++ "\x5c" (seq.++ "s" (seq.++ "\x5c" ""))))(re.++ ((_ re.loop 2 2) (re.range "d" "d"))(re.++ (str.to_re (seq.++ "-" (seq.++ "\x5c" "")))(re.++ ((_ re.loop 3 3) (re.range "w" "w"))(re.++ (str.to_re (seq.++ "-" (seq.++ "\x5c" ""))) ((_ re.loop 4 4) (re.range "d" "d"))))))))
