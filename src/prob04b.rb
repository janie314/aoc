wins = File.read(File.join(__dir__, "..", "input")).split("\n")
  .filter { |line| line.length > 0 }
  .map { |line| line.gsub(/Card \d+: /, "") }
  .map { |line|
       a, b = line.split("|").map { |t| t.split(/\s+/).filter { |s| s.length > 0 } }
       a.intersection(b).length
     }

multiplier = wins.map { |t| 1 }

for i in 0..(wins.length - 1)
  n = wins[i]
  for j in (i+1)..([wins.length - 1, i+n].min)
    multiplier[j] += multiplier[i]
  end
end

puts multiplier
  .reduce { |a,b| a+b}