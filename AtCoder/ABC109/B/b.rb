#! /usr/bin/env ruby
n = gets.chomp.to_i
require 'set'
dic = Set.new
ok = true
last = "$"
n.times do |i|
  w = gets.chomp
  if dic.include?(w)
    ok = false
  end
  dic.add(w)
  if i>0
    if last!=w[0]
      ok = false
    end
  end
  last = w[-1]
end

puts ok ? "Yes" : "No"
