def matches(s)
  s.to_enum(:scan, /\d+/).map do |m,|
    [$`.size, m]
  end
end

grid = File.read(File.join(__dir__, "..", "input")).split("\n")
m = grid.length
n = grid[0].length
res = grid.map.with_index do |row, i|
  matches(row).map do |x, num|
    y = x + num.length
    if ((i - 1)..(i + 1)).to_a.product(((x - 1)..y).to_a)
        .filter { |r, s| r >= 0 && r <= n - 1 && s >= 0 && s <= m - 1 && !(r == i && x <= s && s <= y - 1) }
        .map { |a, b| !/^\d+$/.match?(grid[a][b]) && grid[a][b] != "." }
        .reduce { |a, b| a || b }
      num.to_i
    else
      0
    end
  end
end
  .flatten
  .reduce { |a, b| a + b }

puts res
