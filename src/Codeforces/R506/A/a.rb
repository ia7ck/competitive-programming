#! /usr/bin/env ruby

n, k=gets.split.map(&:to_i)
s=gets.chomp

def c(s, t)
  cnt=0
  (0..(s.size-t.size)).each do |i|
    cnt+=1 if s[i...(i+t.size)]==t
  end
  # print "#{s} #{t} #{cnt}\n"
  return cnt==2
end

n.times do |i|
  t=s+s[-(i+1)..-1]
  # print "#{t} #{s} #{s[-(i+1)..-1]}\n"
  if c(t, s)
    puts s+s[-(i+1)..-1]*(k-1)
    break
  end
end
