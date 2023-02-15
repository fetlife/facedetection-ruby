require_relative "libfacedetection/version"

begin
  RUBY_VERSION =~ /(\d+\.\d+)/
  require "#{$1}/libfacedetection_ruby"
rescue LoadError
  require "libfacedetection_ruby"
end

module Libfacedetection
  class Error < StandardError; end
  # Your code goes here...
end
