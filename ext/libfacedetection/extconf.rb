# frozen_string_literal: true

require "mkmf"
require "rb_sys/mkmf"

create_rust_makefile("libfacedetection/libfacedetection") do |config|
  config.profile = ENV.fetch("RB_SYS_CARGO_PROFILE", "release").to_sym
end
