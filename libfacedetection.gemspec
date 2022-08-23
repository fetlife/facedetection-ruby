# frozen_string_literal: true

require_relative "lib/libfacedetection/version"

Gem::Specification.new do |spec|
  spec.name = "facedetection"
  spec.version = Libfacedetection::VERSION
  spec.authors = ["Andrii Dmytrenko"]
  spec.email = ["andrii@fetlife.com"]

  spec.summary = "Face Detection gem wrapper for libfacedetection"
  spec.description = "A test gem"
  spec.homepage = "https://github.com/"
  spec.license = "MIT"
  spec.required_ruby_version = ">= 2.3.0"

  # Specify which files should be added to the gem when it is released.
  # The `git ls-files -z` loads the files in the RubyGem that have been added into git.
  spec.files = Dir["lib/**/*.rb", "ext/**/*.{rs,toml,lock,rb}"]
  spec.bindir = "exe"
  spec.executables = spec.files.grep(%r{\Aexe/}) { |f| File.basename(f) }
  spec.require_paths = ["lib"]
  spec.extensions = ["ext/libfacedetection/extconf.rb"]
  spec.add_dependency "rb_sys", "~> 0.9"


  # Uncomment to register a new dependency of your gem
  # spec.add_dependency "example-gem", "~> 1.0"

  # For more information and examples about making a new gem, check out our
  # guide at: https://bundler.io/guides/creating_gem.html
end
