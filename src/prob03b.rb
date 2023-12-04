require "json"

def matches(s)
  s.to_enum(:scan, /\d+/).map do |m,|
    [$`.size, m]
  end
end

grid = File.read(File.join(__dir__, "..", "input")).split("\n")
m = grid.length
n = grid[0].length

stars = {}

# observation; we can find 'parts' by only finding numbers that are adjacent to '*'s.
# if a number is not adjacent to a '*', then it is not relevant in this problem
# so, find all parts (numbers adjacent to a '*'). for each part, tag the part's stars with its part number
# then at the end, we'll go back and find which stars are tagged with exactly two parts
parts = grid.map.with_index do |row, i|
  matches(row).map do |x, num|
    y = x + num.length
    if ((i - 1)..(i + 1)).to_a.product(((x - 1)..y).to_a)
        .filter { |r, s| 0 <= r && r <= n - 1 && 0 <= s && s <= m - 1 && !(r == i && x <= s && s <= y - 1) }
        .filter { |a, b| grid[a][b] == "*" }
        .map { |a, b|
         stars[a] ||= {}
         stars[a][b] ||= []
         stars[a][b].push(num.to_i)
       }
    end
  end
end
  .filter { |t| !t.nil? }

puts stars.values.map { |t|
  t.values.filter { |u| u.length == 2 }
    .map { |r, s| r * s }
}
  .reduce { |a, b| a + b }
  .reduce { |a, b| a + b }
