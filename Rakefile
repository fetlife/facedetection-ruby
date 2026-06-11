# frozen_string_literal: true

require "bundler/gem_tasks"
require "rake/testtask"
require "rb_sys/extensiontask"

GEMSPEC = Gem::Specification.load("libfacedetection.gemspec")

task default: :test
task gem: :build

RbSys::ExtensionTask.new("libfacedetection", GEMSPEC) do |ext|
  ext.lib_dir = "lib/libfacedetection"
end

Rake::TestTask.new do |t|
  t.deps << "compile:dev"
  t.libs << "lib"
  t.libs << "tests"
  t.test_files = FileList["tests/test_*.rb"]
end
