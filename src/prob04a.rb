puts File.read(File.join(__dir__, "..", "input")).split("\n")
  .filter { |line| line.length > 0 }
  .map { |line| line.gsub(/Card \d+: /, "") }
  .map { |line|
       a, b = line.split("|").map { |t| t.split(/\s+/).filter { |s| s.length > 0 } }
       n = a.intersection(b).length
       if n == 0
         0
       else
         2**(n - 1)
       end
     }
  .reduce { |a, b| a + b }
