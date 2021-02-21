s = gets.split.map(&:to_s).sort.join
t = [1, 9, 7, 4].sort.join
if s == t
  puts "YES"
else
  puts "NO"
end
