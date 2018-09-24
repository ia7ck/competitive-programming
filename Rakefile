desc ".dをテスト"
task :unittest do
  files = Dir["library/dlang/**.d"]
  files.each do |file|
    unless system("dmd -unittest -main -run #{file}")
      puts "#{file} : test failed"
    end
  end
end
