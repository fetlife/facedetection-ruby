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

begin
  [
    "libfacedetection/#{Libfacedetection::RUBY_API_VERSION}/libfacedetection_ruby",
    "libfacedetection/libfacedetection_ruby",
    "../ext/libfacedetection/libfacedetection_ruby"
  ].each do |path|
    begin
      require_relative path
      Libfacedetection.instance_variable_set(:@available, true)
      break
    rescue LoadError
      next
    end
  end
rescue LoadError
  Libfacedetection.instance_variable_set(:@available, false)
end

Libfacedetection.instance_variable_set(:@available, false) unless Libfacedetection.available?
