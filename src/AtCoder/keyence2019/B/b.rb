#!/usr/bin/env ruby
s = gets.chomp
n = s.size
n.times do |i|
  (i...n).each do |j|
    t = s[0...i] + s[j...n]
    if t == "keyence"
      puts "YES"
      exit 0
    end
  end
end
puts "NO"
