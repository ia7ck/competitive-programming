#! /usr/bin/env ruby
n = gets.chomp.to_i
ans = 0.0
n.times do
  args = gets.split
  if args[1] == "JPY"
    ans += args[0].to_i
  else
    ans += args[0].to_f * 380000
  end
end
puts ans
