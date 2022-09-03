require 'rake/extensiontask'
spec = eval(File.read("libfacedetection.gemspec"))

Rake::ExtensionTask.new("libfacedetection", spec) do |ext|
  ext.lib_dir = "lib/libfacedetection"
  ext.source_pattern = "*.{rs,toml}"
  ext.cross_compile = true
  ext.cross_platform = %w[x86_64-linux x86_64-darwin arm64-darwin aarch64-linux]
  ext.config_script = ENV["ALTERNATE_CONFIG_SCRIPT"] || "extconf.rb"
end
