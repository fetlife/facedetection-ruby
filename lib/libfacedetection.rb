require_relative "libfacedetection/version"

begin
  RUBY_VERSION =~ /(\d+\.\d+)/
  require "libfacedetection/#{$1}/libfacedetection"
rescue LoadError
  require "libfacedetection/libfacedetection"
end

module Libfacedetection
  class Error < StandardError; end
  # Your code goes here...
end
