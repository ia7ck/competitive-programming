#! /usr/bin/env ruby

n, m = gets.split.map(&:to_i)
s = gets.chomp
t = gets.chomp

if s.include?("*")
  a, b = s.split("*")
  if a == nil and b == nil
    puts "YES"
  elsif a == nil
    puts (t.end_with?(b) ? "YES" : "NO")
  elsif b == nil
    puts (t.start_with?(a) ? "YES" : "NO")
  else
    if t.start_with?(a) and t.end_with?(b)
      if a.size + b.size <= m
        puts "YES"
      else
        puts "NO"
      end
    else
      puts "NO"
    end
  end
else
  puts (s == t ? "YES" : "NO")
end
