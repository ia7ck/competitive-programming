#!/usr/bin/env ruby

a, b=gets.split.map(&:to_i)
x=b-a
puts x*(x-1)/2-a