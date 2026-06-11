# frozen_string_literal: true

require "mkmf"
require "pathname"
require "rb_sys/mkmf"

create_rust_makefile("libfacedetection") do |config|
  relative_ext_dir = Pathname(__dir__).relative_path_from(Pathname.pwd)
  # rb_sys strips one leading "./" while constructing the Cargo manifest path.
  config.ext_dir = "./#{relative_ext_dir}"
  config.profile = ENV.fetch("RB_SYS_CARGO_PROFILE", "release").to_sym
end
