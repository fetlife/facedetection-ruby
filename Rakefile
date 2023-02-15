require 'rubygems/package_task'

spec = eval(File.read("libfacedetection.gemspec"))

gem_task = Gem::PackageTask.new(spec) do |pkg|
  pkg.need_zip = true
  pkg.need_tar = true
end

desc "Generate a pre-compiled native gem"
task "gem:native" => ["gem"] do
  sh "gem compile #{gem_task.package_dir_path}.gem"
end
