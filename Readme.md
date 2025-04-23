# libfacedetection

[![run-tests](https://github.com/fetlife/facedetection-ruby/actions/workflows/workflow.yml/badge.svg)](https://github.com/fetlife/facedetection-ruby/actions/workflows/workflow.yml)

## Releases

This gem is automatically released using Github Releases when a new version tag is pushed to the repository. The release process includes:

1. Building the source gem
2. Compiling native extensions for multiple platforms:
   - x86_64-linux (Ubuntu 24.04)
   - aarch64-linux (Ubuntu 24.04 ARM)
   - arm64-darwin (macOS)

To create a new release:

1. Update the version in the lib/libfacedetection/version.rb
2. Create and push a new tag:
   ```bash
   git tag 1.0.0
   git push origin 1.0.0
   ```

You can find all releases on the [GitHub Releases page](https://github.com/fetlife/facedetection-ruby/releases).

