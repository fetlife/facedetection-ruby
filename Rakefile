# frozen_string_literal: true

begin
  require "bundler/setup"
  require "bundler/gem_tasks"
rescue LoadError
  # Allow plain Ruby environments to use already-installed gems.
end

require "rb_sys/extensiontask"
require "shellwords"

GEMSPEC = Gem::Specification.load(File.expand_path("libfacedetection.gemspec", __dir__))
CROSS_PLATFORMS = %w[
  x86_64-linux
  aarch64-linux
  x86_64-darwin
  arm64-darwin
].freeze

RbSys::ExtensionTask.new("libfacedetection", GEMSPEC) do |ext|
  ext.lib_dir = "lib/libfacedetection"
  ext.cross_compile = true
  requested_platforms = [ENV["RUBY_TARGET"]].compact
  ext.cross_platform = requested_platforms.empty? ? CROSS_PLATFORMS : requested_platforms
end

def normalize_platforms(value)
  Array(value)
    .flat_map { |entry| entry.to_s.split(",") }
    .map(&:strip)
    .reject(&:empty?)
end

def resolve_platforms(args)
  requested = normalize_platforms(args.to_a + [ENV["PLATFORMS"], ENV["TARGET"]])
  platforms = requested.empty? ? CROSS_PLATFORMS : requested
  invalid_platforms = platforms - CROSS_PLATFORMS

  return platforms if invalid_platforms.empty?

  abort "Unsupported platform(s): #{invalid_platforms.join(', ')}. Supported: #{CROSS_PLATFORMS.join(', ')}"
end

def run_rb_sys_dock!(platform)
  command = ["rb-sys-dock", "--platform", platform, "--build"]

  puts "==> #{command.shelljoin}"
  sh(*command)
  return if $?.success?

  abort "rb-sys-dock build failed for #{platform}"
end

namespace :dock do
  namespace :build do
    desc "Cross-compile the native gem for all configured platforms via rb-sys-dock"
    task :all do
      CROSS_PLATFORMS.each { |platform| run_rb_sys_dock!(platform) }
    end

    desc "Cross-compile the native gem via rb-sys-dock. Use TARGET=platform or PLATFORMS=platform1,platform2"
    task :selected, [:platform] do |_task, args|
      resolve_platforms([args[:platform]]).each do |platform|
        run_rb_sys_dock!(platform)
      end
    end

    CROSS_PLATFORMS.each do |platform|
      task_name = platform.tr("-", "_").to_sym

      desc "Cross-compile the native gem for #{platform} via rb-sys-dock"
      task task_name do
        run_rb_sys_dock!(platform)
      end
    end
  end
end
