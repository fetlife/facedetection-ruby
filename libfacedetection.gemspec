# frozen_string_literal: true

$:.push File.expand_path("../lib", __FILE__)
require "libfacedetection/version"

Gem::Specification.new do |spec|
  spec.name = "libfacedetection"
  spec.version = Libfacedetection::VERSION
  spec.authors = ["Fetlife", "Andrii Dmytrenko"]
  spec.email = ["andrii@fetlife.com"]

  spec.summary = "Face Detection gem wrapper for libfacedetection"
  spec.description = "A test gem"
  spec.homepage = "https://github.com/"
  spec.license = "MIT"
  spec.required_ruby_version = ">= 2.3.0"

  spec.platform = Gem::Platform::RUBY

  # Specify which files should be added to the gem when it is released.
  # The `git ls-files -z` loads the files in the RubyGem that have been added into git.
  spec.files = Dir["lib/**/*.rb", "ext/**/*.{rs,toml,lock,rb}"]
  spec.bindir = "exe"
  spec.executables = spec.files.grep(%r{\Aexe/}) { |f| File.basename(f) }
  spec.require_paths = ["lib"]
  spec.extensions = ["ext/libfacedetection/extconf.rb"]
  spec.add_dependency "rb_sys", "~> 0.9"
  spec.add_development_dependency "rake", "~> 13.0"
  spec.add_development_dependency "rake-compiler", "~> 1.2.0"
  # spec.add_development_dependency "rake-compiler-dock", "~> 1.2.2"

  spec.metadata = { "github_repo" => "ssh://github.com/fetlife/facedetection-ruby" }

  # Uncomment to register a new dependency of your gem
  # spec.add_dependency "example-gem", "~> 1.0"

  # For more information and examples about making a new gem, check out our
  # guide at: https://bundler.io/guides/creating_gem.html
end
