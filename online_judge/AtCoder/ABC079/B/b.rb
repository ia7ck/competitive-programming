#! /usr/bin/env ruby

n = gets.chomp.to_i

l = [2, 1]
(n - 1).times do
  l.push(l[-1] + l[-2])
end

puts l[-1]
