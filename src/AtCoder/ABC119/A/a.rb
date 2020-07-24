#! /usr/bin/env ruby
y, m, d = gets.chomp.split("/").map(&:to_i)
if y < 2019
  puts "Heisei"
elsif y > 2019
  puts "TBD"
else
  if m < 4
    puts "Heisei"
  elsif m > 4
    puts "TBD"
  else
    if d <= 30
      puts "Heisei"
    else
      puts "TBD"
    end
  end
end
