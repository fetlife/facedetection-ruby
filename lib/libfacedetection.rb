# frozen_string_literal: true

require_relative "libfacedetection/version"

module Libfacedetection
  RUBY_API_VERSION = RUBY_VERSION.split(".")[0, 2].join(".").freeze

  class Error < StandardError; end

  class << self
    def available?
      @available == true
    end
  end
end

[
  "libfacedetection/#{Libfacedetection::RUBY_API_VERSION}/libfacedetection",
  "libfacedetection/libfacedetection",
  "../ext/libfacedetection/libfacedetection",
].each do |path|
  require_relative path
  Libfacedetection.instance_variable_set(:@available, true)
  break
rescue LoadError
  next
end

unless Libfacedetection.available?
  begin
    extension_dir = Gem.loaded_specs["libfacedetection"]&.extension_dir if defined?(Gem)
    require File.join(extension_dir, "libfacedetection") if extension_dir
    Libfacedetection.instance_variable_set(:@available, true)
  rescue LoadError
    nil
  end
end

Libfacedetection.instance_variable_set(:@available, false) unless Libfacedetection.available?
