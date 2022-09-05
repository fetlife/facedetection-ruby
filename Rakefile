require 'rake/extensiontask'
require 'rubygems/package_task'

spec = eval(File.read("libfacedetection.gemspec"))

platforms = %w[x86_64-linux x86_64-darwin arm64-darwin aarch64-linux]

Rake::ExtensionTask.new("libfacedetection", spec) do |ext|
  ext.lib_dir = "lib/libfacedetection"
  ext.source_pattern = "*.{rs,toml}"
  ext.cross_compile = true
  ext.cross_platform = platforms
  ext.cross_config_options << "--enable-cross-build"
  ext.config_script = ENV["ALTERNATE_CONFIG_SCRIPT"] || "extconf.rb"
end

Gem::PackageTask.new(spec) do |pkg|
  pkg.need_zip = true
  pkg.need_tar = true
end
