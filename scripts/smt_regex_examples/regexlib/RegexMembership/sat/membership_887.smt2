(explore-derivatives (re.++ (re.opt (str.to_re (seq.++ "j" (seq.++ "a" (seq.++ "r" (seq.++ ":" ""))))))(re.++ (str.to_re (seq.++ "f" (seq.++ "i" (seq.++ "l" (seq.++ "e" (seq.++ ":" (seq.++ "/" "")))))))(re.++ (re.++ (re.opt (re.++ (re.range "A" "Z") (re.range ":" ":")))(re.++ (re.range "/" "/") (re.+ (re.union (re.range "!" "!")(re.union (re.range "#" "$")(re.union (re.range "&" "&")(re.union (re.range "(" "+")(re.union (re.range "-" "9")(re.union (re.range "@" "[")(re.union (re.range "]" "]")(re.union (re.range "_" "_") (re.range "~" "~")))))))))))) (re.union (re.++ (re.range "/" "/")(re.++ (re.+ (re.union (re.range "(" ")")(re.union (re.range "+" "+")(re.union (re.range "-" "-")(re.union (re.range "0" "9")(re.union (re.range "=" "=")(re.union (re.range "A" "[")(re.union (re.range "]" "]")(re.union (re.range "_" "_") (re.range "~" "~")))))))))) (str.to_re (seq.++ "." (seq.++ "j" (seq.++ "a" (seq.++ "r" (seq.++ "!" "")))))))) (re.++ (re.union (re.range "\x00" " ") (re.range "\x22" "\xff")) (str.to_re (seq.++ "/" (seq.++ "c" (seq.++ "o" (seq.++ "m" (seq.++ "/" (seq.++ "r" (seq.++ "e" (seq.++ "g" (seq.++ "e" (seq.++ "x" (seq.++ "l" (seq.++ "i" (seq.++ "b" (seq.++ "/" (seq.++ "e" (seq.++ "x" (seq.++ "a" (seq.++ "m" (seq.++ "p" (seq.++ "l" (seq.++ "e" (seq.++ "/" "")))))))))))))))))))))))))))))
