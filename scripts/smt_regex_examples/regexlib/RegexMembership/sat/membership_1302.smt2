(explore-derivatives (re.++ (str.to_re "")(re.++ (re.union (re.range "A" "Z") (re.range "a" "z"))(re.++ (re.range ":" ":")(re.++ (re.union (re.range "\x5c" "\x5c") (re.+ (re.++ (re.range "\x5c" "\x5c") (re.+ (re.union (re.range "\x00" "\x08")(re.union (re.range "\x0e" "\x1f")(re.union (re.range "!" "!")(re.union (re.range "#" ")")(re.union (re.range "+" ".")(re.union (re.range "0" "9")(re.union (re.range ";" ";")(re.union (re.range "=" "=")(re.union (re.range "?" "[")(re.union (re.range "]" "{")(re.union (re.range "}" "\x84")(re.union (re.range "\x86" "\x9f") (re.range "\xa1" "\xff"))))))))))))))))) (re.range ">" ">"))))))
