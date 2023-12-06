blocks = File.read(File.join(__dir__, "..", "input")).split("\n\n")

seeds = blocks[0].split(": ")[1].split(" ").map { |t| t.to_i }
  .each_slice(2).map { |a, b|
  a..(a + b - 1)
}

blocks = blocks[1..].map { |block|
  res = block.split("\n")[1..].map { |row|
    row.split(" ").map { |t| t.to_i }
  }
    .to_a.sort { |a, b| (a[0] <= b[0]) ? -1 : 1 }
  [res, res[0][1], res[-1][1] + res[-1][2]]
}

cache = {}

min = 2**32
puts seeds.each { |r|
  r.each_with_index { |seed, k|
    s = seed
    if k % 1000000 == 0
      puts s
    end
    blocks.each_with_index do |block, i|
      if (s < block[1]) || (block[2] <= s)
        break
      elsif !cache[i].nil? && !cache[i][s].nil?
        s = cache[i][s]
      else
        cache ||= {}
        block[0].each_with_index do |row, j|
          if row[1] <= s && s < row[1] + row[2]
            new_s = row[0] + s - row[1]
            cache[i] ||= {}
            cache[i][s] = new_s
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
