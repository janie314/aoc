require "pry-byebug"

blocks = File.read(File.join(__dir__, "..", "input")).split("\n\n")

seed_ranges = blocks[0].split(": ")[1].split(" ").map { |t| t.to_i }
  .each_slice(2)

blocks = blocks[1..].map { |block|
  res = block.split("\n")[1..].map { |row|
    row.split(" ").map { |t| t.to_i }
  }
    .to_a.sort { |a, b| (a[1] <= b[1]) ? -1 : 1 }
  [res, res[0][1], res[-1][1] + res[-1][2]]
}.to_a

binding.pry

min = 2**32
puts seed_ranges.each { |r|
  puts "next " + r[0].to_s + "\t" + r[1].to_s
  cache = {}
  (r[0]..r[0] + r[1] - 1).each_with_index { |seed, k|
    s = seed
    if k % 1000000 == 0
      puts "s #{s}"
    end
    blocks.each_with_index do |block, i|
      # binding.pry
      if (s < block[1]) || (block[2] <= s)
        break
      else
        start = cache[i] || 0
        block[0].each_with_index.drop(start) do |row, j|
          if row[1] <= s && s < row[1] + row[2]
            new_s = row[0] + s - row[1]
            cache[i] = j
            s = new_s
            break
          end
        end
      end
    end
    min = [s, min].min
  }
}
puts min
