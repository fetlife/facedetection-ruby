gem 'libfacedetection'
require 'libfacedetection'
require 'libfacedetection/facedetect'

require 'minitest/autorun'

class FaceDetectionTest < Minitest::Test
  def test_facedetection
    result = Libfacedetection::FaceDetection.detect_faces(Vips::Image.new_from_file(File.expand_path("female.webp", __dir__)))
    assert_instance_of(Array, result)
    assert_equal(1, result.count)
    assert(result[0][:confidence] > 90)
  end
end
