# frozen_string_literal: true

$:.push File.expand_path("lib", __dir__)
require "libfacedetection/version"

Gem::Specification.new do |spec|
  spec.name = "libfacedetection"
  spec.version = Libfacedetection::VERSION
  spec.authors = ["Fetlife", "Andrii Dmytrenko"]
  spec.email = ["andrii@fetlife.com"]

  spec.summary = "Face Detection gem wrapper for libfacedetection"
  spec.description = "Face Detection gem wrapper for libfacedetection"
  spec.homepage = "https://github.com/fetlife/facedetection-ruby"
  spec.license = "MIT"
  spec.required_ruby_version = ">= 3.2"

  spec.platform = Gem::Platform::RUBY

  spec.files = Dir[
    "Cargo.toml",
    "lib/**/*.rb",
    "ext/**/*",
    "*.gemspec",
    "README.md",
    "LICENSE.txt"
  ].reject do |path|
    path.match?(%r{\A(?:lib/libfacedetection/(?:.*\.(?:bundle|so|dll|dylib))|ext/libfacedetection/(?:Makefile|mkmf\.log|target/|.*\.(?:bundle|so|dll)))})
  end
  spec.bindir = "exe"
  spec.executables = spec.files.grep(%r{\Aexe/}) { |f| File.basename(f) }
  spec.require_paths = ["lib"]
  spec.extensions = ["ext/libfacedetection/extconf.rb"]
  spec.add_dependency "rb_sys", "~> 0.9.126"
  spec.add_development_dependency "rake", "~> 13.0"
  spec.add_development_dependency "rake-compiler", "~> 1.2"
  spec.add_development_dependency "ruby-vips"
  spec.add_development_dependency "ffi"

  spec.metadata = {
    "github_repo" => "ssh://github.com/fetlife/facedetection-ruby",
    "cargo_crate_name" => "libfacedetection-ruby"
  }
end
