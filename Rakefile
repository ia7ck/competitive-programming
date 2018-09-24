desc ".dをテスト"
task :unittest do
  files = Dir["library/dlang/**.d"]
  failed = false
  files.each do |file|
    unless system("dmd -unittest -main -run #{file}")
      puts "#{file} : test failed"
      failed = true
    end
  end
  exit(failed ? 1 : 0)
end
