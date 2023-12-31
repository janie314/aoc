blocks = File.read(File.join(__dir__, "..", "input")).split("\n\n")

seed_ranges = blocks[0].split(": ")[1].split(" ").map { |t| t.to_i }

blocks = blocks[1..].map { |block|
  block.split("\n")[1..].map { |row|
    row.split(" ").map { |t| t.to_i }
  }
    .to_a.sort { |a, b| (a[1] <= b[1]) ? -1 : 1 }
}

puts (seed_ranges.map do |seed|
  s = seed
  blocks.each do |block|
    block.each do |row|
      if row[1] <= s && s < row[1] + row[2]
        s = row[0] + s - row[1]
        break
      end
    end
  end
  s
end).min
