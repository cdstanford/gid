(explore-derivatives (re.++ (str.to_re "")(re.++ (re.union (re.++ (re.union (str.to_re (seq.++ "h" (seq.++ "t" (seq.++ "t" (seq.++ "p" "")))))(re.union (str.to_re (seq.++ "h" (seq.++ "t" (seq.++ "t" (seq.++ "p" (seq.++ "s" "")))))) (str.to_re (seq.++ "f" (seq.++ "t" (seq.++ "p" "")))))) (str.to_re (seq.++ ":" (seq.++ "/" (seq.++ "/" ""))))) (str.to_re (seq.++ "w" (seq.++ "w" (seq.++ "w" (seq.++ "." ""))))))(re.++ (re.+ (re.union (re.range "-" ".")(re.union (re.range "0" "9")(re.union (re.range "A" "Z") (re.range "a" "z")))))(re.++ (re.range "." ".")(re.++ ((_ re.loop 2 4) (re.union (re.range "A" "Z") (re.range "a" "z")))(re.++ (re.* (re.++ (re.range "/" "/")(re.++ (re.* (re.union (re.range "#" "%")(re.union (re.range "'" "'")(re.union (re.range "+" ".")(re.union (re.range "0" "9")(re.union (re.range "=" "=")(re.union (re.range "?" "?")(re.union (re.range "A" "Z")(re.union (re.range "_" "_")(re.union (re.range "a" "z") (re.range "~" "~"))))))))))) (re.union (re.range "\x00" "\x08")(re.union (re.range "\x0e" "\x1f")(re.union (re.range "!" "'")(re.union (re.range "*" "+")(re.union (re.range "-" "-")(re.union (re.range "/" "\x84")(re.union (re.range "\x86" "\x9f") (re.range "\xa1" "\xff"))))))))))) (str.to_re ""))))))))
