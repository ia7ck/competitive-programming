#! /usr/bin/env ruby

s = gets.chomp
op = ['+', '-']

op.each do |op1|
  op.each do |op2|
    op.each do |op3|
      exp = s[0] + op1 + s[1] + op2 + s[2] + op3 + s[3]
      if eval(exp) == 7
        puts exp + '=7'
        exit 0
      end
    end
  end
end
