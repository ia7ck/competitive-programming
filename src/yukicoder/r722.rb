a, b = gets.split.map(&:to_i)

if a % 100 == 0 and b % 100 == 0
  res = a * b
  a, b = a.abs.to_s, b.abs.to_s
  if a.count("0") + 1 == a.size and b.count("0") + 1 == b.size
    puts res / 10
  else
    puts res
  end
else
  if -99999999 <= a * b and a * b <= 99999999
    puts a * b
  else
    puts "E"
  end
end
