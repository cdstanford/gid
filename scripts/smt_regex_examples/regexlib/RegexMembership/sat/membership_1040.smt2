(explore-derivatives (re.++ (str.to_re (seq.++ "&" (seq.++ "l" (seq.++ "t" (seq.++ ";" (seq.++ "t" (seq.++ "a" (seq.++ "b" (seq.++ "l" (seq.++ "e" (seq.++ "&" (seq.++ "g" (seq.++ "t" (seq.++ ";" ""))))))))))))))(re.++ (re.* (re.++ (str.to_re (seq.++ "&" (seq.++ "l" (seq.++ "t" (seq.++ ";" (seq.++ "t" (seq.++ "r" (seq.++ "&" (seq.++ "g" (seq.++ "t" (seq.++ ";" "")))))))))))(re.++ (re.+ (re.++ (str.to_re (seq.++ "&" (seq.++ "l" (seq.++ "t" (seq.++ ";" (seq.++ "t" (seq.++ "d" (seq.++ "&" (seq.++ "g" (seq.++ "t" (seq.++ ";" "")))))))))))(re.++ (re.* (re.union (re.range "0" "9")(re.union (re.range "A" "Z") (re.range "a" "z")))) (str.to_re (seq.++ "&" (seq.++ "l" (seq.++ "t" (seq.++ ";" (seq.++ "/" (seq.++ "t" (seq.++ "d" (seq.++ "&" (seq.++ "g" (seq.++ "t" (seq.++ ";" ""))))))))))))))) (str.to_re (seq.++ "&" (seq.++ "l" (seq.++ "t" (seq.++ ";" (seq.++ "/" (seq.++ "t" (seq.++ "r" (seq.++ "&" (seq.++ "g" (seq.++ "t" (seq.++ ";" ""))))))))))))))) (str.to_re (seq.++ "&" (seq.++ "l" (seq.++ "t" (seq.++ ";" (seq.++ "/" (seq.++ "t" (seq.++ "a" (seq.++ "b" (seq.++ "l" (seq.++ "e" (seq.++ "&" (seq.++ "g" (seq.++ "t" (seq.++ ";" ""))))))))))))))))))
